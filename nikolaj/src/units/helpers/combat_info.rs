use crate::Nikolaj;
use rand::Rng;
use rust_sc2::prelude::*;
use std::collections::HashMap;


pub fn combat_info_step(bot: &mut Nikolaj) {
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
    bot.combat_info.heatmaps.clear();
}

#[derive(Default, Debug, Clone)]
pub struct CombatInfo {
    pub last_time: f32,
    pub unsiege_timer: Vec<UnsiegeTimer>,
    pub scanner_sweep_time: f32,
    pub heatmaps: HashMap<u64, Heatmap>,
}

impl CombatInfo {
    pub fn get_unsiege_timer(&self, tag: u64) -> Option<&UnsiegeTimer> {
        self.unsiege_timer.iter().find(|t| t.tag == tag)
    }
    pub fn add_unsiege_timer(&mut self, tag: u64) {
        let new_time = rand::rng().random_range(2.0..6.0);
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
	pub unit_tag: u64,
	pub points: Vec<HeatPoint>,
}

impl Heatmap {
	pub fn add_heat_to_point(&mut self, point: Point2, intensity: f32) {
		if let Some(hp) = self.points.iter_mut().find(|hp| hp.position == point) {
			hp.intensity += intensity;
		} else {
			self.points.push(HeatPoint { position: point, intensity });
		}
	}

	pub fn get_best_position(&self) -> Option<Point2> {
		self.points
			.iter()
			.max_by(|a, b| a.intensity.partial_cmp(&b.intensity).unwrap_or(std::cmp::Ordering::Equal))
			.map(|hp| hp.position)
	}

	pub fn get_best_attack_position(&self) -> Option<Point2> {
		self.points
			.iter()
			.filter(|hp| hp.intensity >= 1000.0) 
			.max_by(|a, b| a.intensity.partial_cmp(&b.intensity).unwrap_or(std::cmp::Ordering::Equal))
			.map(|hp| hp.position)
	}
}

#[derive(Clone, Debug)]
pub struct HeatPoint {
	pub position: Point2,
	pub intensity: f32,
}

pub fn get_heatmap_for_unit(bot: &mut Nikolaj, unit_tag: u64) -> Heatmap {
    if let Some(heatmap) = bot.combat_info.heatmaps.get_mut(&unit_tag) {
        heatmap.clone()
    } else {
        let heatmap = create_heatmap_for_unit(bot, unit_tag);
        bot.combat_info.heatmaps.insert(unit_tag, heatmap.clone());
        return heatmap;
    }
}

fn create_heatmap_for_unit(bot: &mut Nikolaj, unit_tag: u64) -> Heatmap {
    let mut heatmap = Heatmap {
        unit_tag,
        points: Vec::new(),
    };
    
    if let Some(unit) = bot.units.my.units.get(unit_tag) {
        let unit_pos = unit.position();
        let sight_range = unit.sight_range();
        let step = 1.0;

        let weapon_range = if unit.is_flying() {
            unit.air_range().max(unit.ground_range())
        } else {
            unit.ground_range()
        };

        let grid_size = (sight_range / step).ceil() as i32;
        for dx in -grid_size..=grid_size {
            for dy in -grid_size..=grid_size {
                let pos = Point2::new(unit_pos.x + dx as f32 * step, unit_pos.y + dy as f32 * step);

                if unit_pos.distance(pos) > sight_range {
                    continue;
                }

                if !bot.is_pathable(pos) && !unit.is_flying() { 
                    continue; 
                }

                let mut intensity = 0.0;
                let mut has_target = false;

                for enemy in bot.units.enemy.units.iter() {
                    if can_attack_unit(enemy, &unit) {
                        let dist_to_enemy = enemy.position().distance(pos);
                        let enemy_weapon_range = get_weapon_range(enemy, &unit);

                        if dist_to_enemy <= enemy_weapon_range {
                            let damage = enemy.real_range_vs(&unit);
                            intensity -= damage;
                        }
                    }

                    if !has_target && can_attack_unit(&unit, enemy) {
                        let distance_to_enemy = enemy.position().distance(pos);
                        if distance_to_enemy <= weapon_range {
                            has_target = true;
                        }
                    }
                }

                for enemy_structure in bot.units.enemy.structures.iter() {
                    if can_attack_unit(enemy_structure, &unit) {
                        let dist_to_structure = enemy_structure.position().distance(pos);
                        let structure_weapon_range = get_weapon_range(enemy_structure, &unit);

                        if dist_to_structure <= structure_weapon_range {
                            let damage = enemy_structure.real_range_vs(&unit);
                            intensity -= damage;
                        }
                    }

                    if !has_target && can_attack_unit(&unit, enemy_structure) {
                        let distance_to_structure = enemy_structure.position().distance(pos);
                        if distance_to_structure <= weapon_range {
                            has_target = true;
                        }
                    }
                }

                if has_target {
                    intensity += 1000.0;
                }

                heatmap.points.push(HeatPoint { 
                    position: pos, 
                    intensity 
                });
            }
        }
    }
    
    heatmap
}

fn can_attack_unit(attacker: &Unit, target: &Unit) -> bool {
    if target.is_flying() && attacker.can_attack_air() {
        return true;
    }
    if !target.is_flying() && attacker.can_attack_ground() {
        return true;
    }
    false
}

fn get_weapon_range(attacker: &Unit, target: &Unit) -> f32 {
    if target.is_flying() {
        attacker.air_range()
    } else {
        attacker.ground_range()
    }
}