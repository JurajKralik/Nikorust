use crate::Nikolaj;
use crate::units::helpers::surroundings::can_attack;
use rust_sc2::prelude::*;


const POINT_ATTACK_BONUS: f32 = 1000.0;
const POINT_DETECTION_PENALTY: f32 = 2000.0;
const POINT_ALLY_BONUS: f32 = 100.0;
const POINT_DISTANCE_BONUS_PER_UNIT: f32 = 0.2;  // Small bonus for distance from enemy

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
    pub ally_present: bool,
}

#[derive(Clone, Debug)]
pub struct HeatmapOptions {
    pub evaluate_detection: bool,
    pub step: f32,
    pub avoid_damage: bool,
    pub allies_influence: bool,
}

impl Default for HeatmapOptions {
    fn default() -> Self {
        HeatmapOptions {
            evaluate_detection: false,
            step: 1.0,
            avoid_damage: false,
            allies_influence: false,
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
        
        // Check if unit is cloaked and detected
        let is_cloaked = unit.is_cloaked();
        let is_detected = bot.combat_info.detected;
        let vulnerable = !is_cloaked || is_detected;
        
        // Use enemy army snapshots - evaluate all enemies within 20.0 range
        let nearby_enemy_snapshots: Vec<_> = bot.strategy_data.enemy_army.units
            .iter()
            .filter(|snapshot| snapshot.position().distance(unit.position()) <= 20.0)
            .map(|snapshot| snapshot.unit.clone())
            .collect();
        evaluate_enemy_units_from_snapshots(&mut heatmap, unit, &nearby_enemy_snapshots, vulnerable);
        
        let nearby_enemy_structures = bot.units.enemy.structures.closer(20.0, unit.position());
        evaluate_enemy_structures(&mut heatmap, unit, &nearby_enemy_structures, vulnerable);
        apply_damage_avoidance(&mut heatmap);
        
        // Detection evaluation from snapshots
        let nearby_enemy_all: Vec<_> = bot.strategy_data.enemy_army.units
            .iter()
            .filter(|snapshot| snapshot.position().distance(unit.position()) <= 20.0)
            .map(|snapshot| snapshot.unit.clone())
            .collect();
        apply_detection_evaluation_from_snapshots(&mut heatmap, unit, &nearby_enemy_all, bot.combat_info.detected);
        
        let nearby_ally_units = bot.units.my.units.closer(20.0, unit.position());
        apply_allies_influence(&mut heatmap, unit_tag, &nearby_ally_units);
        blur_heatmap(&mut heatmap);
    }
    
    heatmap
}

fn generate_heatmap_points(bot: &Nikolaj, heatmap: &mut Heatmap, unit: &Unit) {
    let unit_pos = unit.position();
    let heatmap_range = 4.0;  // Only 4 steps around unit for immediate movement decisions
    let step = heatmap.options.step;
    let grid_size = (heatmap_range / step).ceil() as i32;

    for dx in -grid_size..=grid_size {
        for dy in -grid_size..=grid_size {
            let pos = Point2::new(unit_pos.x + dx as f32 * step, unit_pos.y + dy as f32 * step);

            // Out of range
            if unit_pos.distance(pos) > heatmap_range {
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
                ally_present: false,
            };
            heatmap.points.push(heatpoint);
        }
    }
}

fn evaluate_enemy_units_from_snapshots(heatmap: &mut Heatmap, unit: &Unit, enemy_units: &Vec<Unit>, vulnerable: bool) {
    for enemy in enemy_units.iter() {
        evaluate_attack_opportunities(heatmap, unit, enemy);
        evaluate_incoming_damage(heatmap, unit, enemy, vulnerable);
    }
}

fn evaluate_enemy_structures(heatmap: &mut Heatmap, unit: &Unit, enemy_structures: &Units, vulnerable: bool) {
    for enemy in enemy_structures.iter() {
        evaluate_incoming_damage(heatmap, unit, enemy, vulnerable);
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

fn evaluate_incoming_damage(heatmap: &mut Heatmap, unit: &Unit, enemy: &Unit, vulnerable: bool) {
    if !can_attack(enemy, unit) {
        return;
    }

    let mut enemy_weapon_range = enemy.real_range_vs(unit) + 1.0;
    if enemy.is_worker() {
        enemy_weapon_range += 1.5;
    }
    let damage = enemy.real_weapon_vs(unit).damage as f32;
    
    for heatpoint in heatmap.points.iter_mut() {
        let distance = heatpoint.position.distance(enemy.position());
        
        // Only apply damage penalty if unit is vulnerable (not cloaked or detected)
        if vulnerable && distance <= enemy_weapon_range {
            heatpoint.intensity -= damage;
        }
        
        // Add small bonus based on distance from enemy (further = safer)
        heatpoint.intensity += distance * POINT_DISTANCE_BONUS_PER_UNIT;
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

fn apply_detection_evaluation_from_snapshots(heatmap: &mut Heatmap, unit: &Unit, enemy_units: &Vec<Unit>, detected: bool) {
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

fn apply_allies_influence(heatmap: &mut Heatmap, unit_tag: u64, ally_units: &Units) {
    if !heatmap.options.allies_influence {
        return;
    }

    for heatpoint in heatmap.points.iter_mut() {
        if heatpoint.ally_present {
            continue;
        }

        let has_ally_nearby = ally_units.iter().any(|ally| {
            ally.tag() != unit_tag && 
            ally.position().distance(heatpoint.position) <= 5.0
        });

        if has_ally_nearby {
            heatpoint.ally_present = true;
            heatpoint.intensity += POINT_ALLY_BONUS;
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
        
        // Check all points within 2 heatpoint range (2 * step distance)
        let max_distance = 2.0 * step + 0.1; // Small epsilon for floating point comparison
        
        for neighbor in &original_points {
            let distance = current_pos.distance(neighbor.position);
            
            // Skip self and points outside 2-step range
            if distance < 0.1 || distance > max_distance {
                continue;
            }
            
            // Only consider negative intensities for blurring
            if neighbor.intensity >= 0.0 {
                continue;
            }
            
            total_intensity += neighbor.intensity;
            count += 1;
        }
        
        heatpoint.intensity = total_intensity / count as f32;
    }
}
