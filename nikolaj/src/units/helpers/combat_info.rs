use crate::Nikolaj;
use rand::Rng;
use rust_sc2::prelude::*;
use std::collections::HashMap;
use crate::units::helpers::surroundings::can_attack;


const POINT_ATTACK_BONUS: f32 = 1000.0;
const POINT_DETECTION_PENALTY: f32 = 2000.0;

pub fn combat_info_step(bot: &mut Nikolaj) {
    siege_timer_step(bot);
    bot.combat_info.heatmaps.clear();
}

fn siege_timer_step(bot: &mut Nikolaj) {
    for timer in bot.combat_info.unsiege_timer.clone().iter() {
        if timer.unsiege_in < 0.0 {
            bot.combat_info.remove_unsiege_timer(timer.tag);
        }
    }
    let current_time = bot.time;
    let delta_time = current_time - bot.combat_info.last_time;
    bot.combat_info.last_time = current_time;
    for timer in bot.combat_info.unsiege_timer.iter_mut() {
        timer.unsiege_in -= delta_time;
    }
}

#[derive(Default, Debug, Clone)]
pub struct CombatInfo {
    pub last_time: f32,
    pub unsiege_timer: Vec<UnsiegeTimer>,
    pub scanner_sweep_time: f32,
    pub heatmaps: HashMap<u64, Heatmap>,
    pub detected: bool,
}

impl CombatInfo {
    pub fn get_unsiege_timer(&self, tag: u64) -> Option<&UnsiegeTimer> {
        self.unsiege_timer.iter().find(|t| t.tag == tag)
    }
    pub fn add_unsiege_timer(&mut self, tag: u64) {
        let new_time = rand::thread_rng().gen_range(2.0..6.0);
        self.remove_unsiege_timer(tag);
        self.unsiege_timer.push(UnsiegeTimer {
            tag,
            unsiege_in: new_time
        });
    }
    pub fn remove_unsiege_timer(&mut self, tag: u64) {
        self.unsiege_timer.retain(|t| t.tag != tag);
    }
}

#[derive(Debug, Clone)]
pub struct UnsiegeTimer {
    pub tag: u64,
    pub unsiege_in: f32
}


#[derive(Clone, Debug)]
pub struct Heatmap {
	pub points: Vec<HeatPoint>,
    pub options: HeatmapOptions
}

impl Heatmap {
	pub fn get_best_position(&self) -> Option<Point2> {
		self.points
			.iter()
			.max_by(|a, b| a.intensity.partial_cmp(&b.intensity).unwrap_or(std::cmp::Ordering::Equal))
			.map(|hp| hp.position)
	}
}

#[derive(Clone, Debug)]
pub struct HeatPoint {
	pub position: Point2,
	pub intensity: f32,
    pub can_attack: bool,
    pub detected: bool,
}

#[derive(Clone, Debug)]
pub struct HeatmapOptions {
    pub evaluate_detection: bool,
    pub step: f32,
    pub avoid_damage: bool,
}

impl Default for HeatmapOptions {
    fn default() -> Self {
        HeatmapOptions {
            evaluate_detection: false,
            step: 1.0,
            avoid_damage: false,
        }
    }
}

pub fn get_heatmap_for_unit(bot: &mut Nikolaj, unit_tag: u64, options: HeatmapOptions) -> Heatmap {
    if let Some(heatmap) = bot.combat_info.heatmaps.get_mut(&unit_tag) {
        heatmap.clone()
    } else {
        let heatmap = create_heatmap_for_unit(bot, unit_tag, options);
        bot.combat_info.heatmaps.insert(unit_tag, heatmap.clone());
        return heatmap;
    }
}

fn create_heatmap_for_unit(bot: &mut Nikolaj, unit_tag: u64, options: HeatmapOptions) -> Heatmap {
    let mut heatmap = Heatmap {
        points: Vec::new(),
        options,
    };
    
    if let Some(unit) = bot.units.my.units.get(unit_tag) {
        generate_heatmap_points(bot, &mut heatmap, unit);
        let nearby_enemy_units = bot.units.enemy.units.closer(18.0, unit.position());
        evaluate_enemy_units(&mut heatmap, unit, &nearby_enemy_units);
        let nearby_enemy_structures = bot.units.enemy.structures.closer(18.0, unit.position());
        evaluate_enemy_structures(&mut heatmap, unit, &nearby_enemy_structures);
        apply_damage_avoidance(&mut heatmap);
        let nearby_enemy_all = bot.units.enemy.all.closer(18.0, unit.position());
        apply_detection_evaluation(&mut heatmap, unit, &nearby_enemy_all, bot.combat_info.detected);
        blur_heatmap(&mut heatmap);
    }
    
    heatmap
}

fn generate_heatmap_points(bot: &Nikolaj, heatmap: &mut Heatmap, unit: &Unit) {
    let unit_pos = unit.position();
    let sight_range = unit.sight_range();
    let step = heatmap.options.step;
    let grid_size = (sight_range / step).ceil() as i32;

    for dx in -grid_size..=grid_size {
        for dy in -grid_size..=grid_size {
            let pos = Point2::new(unit_pos.x + dx as f32 * step, unit_pos.y + dy as f32 * step);

            // Out of range
            if unit_pos.distance(pos) > sight_range {
                continue;
            }

            // Not pathable
            if !bot.is_pathable(pos) && !unit.is_flying() { 
                continue; 
            }

            let heatpoint = HeatPoint {
                position: pos,
                intensity: 0.0,
                can_attack: false,
                detected: false,
            };
            heatmap.points.push(heatpoint);
        }
    }
}

fn evaluate_enemy_units(heatmap: &mut Heatmap, unit: &Unit, enemy_units: &Units) {
    for enemy in enemy_units.iter() {
        evaluate_attack_opportunities(heatmap, unit, enemy);
        evaluate_incoming_damage(heatmap, unit, enemy);
    }
}

fn evaluate_enemy_structures(heatmap: &mut Heatmap, unit: &Unit, enemy_structures: &Units) {
    for enemy in enemy_structures.iter() {
        evaluate_incoming_damage(heatmap, unit, enemy);
    }
}

fn evaluate_attack_opportunities(heatmap: &mut Heatmap, unit: &Unit, enemy: &Unit) {
    if !can_attack(unit, enemy) {
        return;
    }

    let weapon_range = unit.real_range_vs(enemy);
    for heatpoint in heatmap.points.iter_mut() {
        if heatpoint.can_attack {
            continue; // Already can attack from this point
        }
        let distance = heatpoint.position.distance(enemy.position());
        if distance <= weapon_range {
            heatpoint.can_attack = true;
            heatpoint.intensity += POINT_ATTACK_BONUS;
        }
    }
}

fn evaluate_incoming_damage(heatmap: &mut Heatmap, unit: &Unit, enemy: &Unit) {
    if !can_attack(enemy, unit) {
        return;
    }

    let enemy_weapon_range = enemy.real_range_vs(unit);
    let damage = enemy.real_weapon_vs(unit).damage as f32;
    
    for heatpoint in heatmap.points.iter_mut() {
        let distance = heatpoint.position.distance(enemy.position());
        if distance <= enemy_weapon_range {
            heatpoint.intensity -= damage;
        }
    }
}

fn apply_damage_avoidance(heatmap: &mut Heatmap) {
    if !heatmap.options.avoid_damage {
        return;
    }

    for heatpoint in heatmap.points.iter_mut() {
        if heatpoint.can_attack && heatpoint.intensity < POINT_ATTACK_BONUS {
            heatpoint.intensity -= POINT_ATTACK_BONUS;
        }
    }
}

fn apply_detection_evaluation(heatmap: &mut Heatmap, unit: &Unit, enemy_units: &Units, detected: bool) {
    if !heatmap.options.evaluate_detection {
        return;
    }

    for enemy_entity in enemy_units.iter() {
        let mut penalty_range: Option<f32> = None;
        if detected {
            penalty_range = Some(enemy_entity.real_range_vs(unit));
        }
        if penalty_range.is_none() && enemy_entity.is_detector() {
            penalty_range = Some(enemy_entity.sight_range());
        }

        if let Some(range) = penalty_range {
            for heatpoint in heatmap.points.iter_mut() {
                if heatpoint.detected {
                    continue;
                }
                if heatpoint.position.distance(enemy_entity.position()) <= range {
                    heatpoint.detected = true;
                    heatpoint.intensity -= POINT_DETECTION_PENALTY;
                }
            }
        }
    }
}

fn blur_heatmap(heatmap: &mut Heatmap) {
    let original_points = heatmap.points.clone();
    let step = heatmap.options.step;

    for heatpoint in heatmap.points.iter_mut() {
        let current_pos = heatpoint.position;
        let mut total_intensity = heatpoint.intensity;
        let mut count = 1;
        
        let directions = [
            Point2::new(0.0, step),
            Point2::new(0.0, -step),
            Point2::new(step, 0.0),
            Point2::new(-step, 0.0),
        ];
        
        for direction in &directions {
            let neighbor_pos = Point2::new(
                current_pos.x + direction.x,
                current_pos.y + direction.y
            );
            
            // Find the neighbor point in the original heatmap
            if let Some(neighbor) = original_points.iter().find(|p| {
                (p.position.x - neighbor_pos.x).abs() < 0.1 && 
                (p.position.y - neighbor_pos.y).abs() < 0.1
            }) {
                total_intensity += neighbor.intensity;
                count += 1;
            }
        }
        
        heatpoint.intensity = total_intensity / count as f32;
    }
}
