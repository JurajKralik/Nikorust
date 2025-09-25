use crate::Nikolaj;
use rust_sc2::prelude::*;

#[derive(Clone)]
pub struct SurroundingsInfo {
    pub best_target_in_range: Option<Unit>,
    pub better_target_off_range: Option<Unit>,
    pub closest_threat: Option<Unit>,
    pub closest_counter: Option<Unit>,
    pub closest_structure: Option<Unit>,
    pub in_danger: bool,
}
impl Default for SurroundingsInfo {
    fn default() -> Self {
        SurroundingsInfo {
            best_target_in_range: None,
            better_target_off_range: None,
            closest_threat: None,
            closest_counter: None,
            closest_structure: None,
            in_danger: false,
        }
    }
}

pub fn get_surroundings_info(bot: &mut Nikolaj, unit: &Unit) -> SurroundingsInfo {
    let mut surroundings = SurroundingsInfo::default();
    let enemies = bot.units.enemy.units.clone();
    let sorted_enemies = enemies.iter().sort_by_distance(unit.position()).closer(unit.sight_range() + 2.0, unit.position());
    let enemy_structures = bot.units.enemy.structures.clone();
    let sorted_structures = enemy_structures.iter().sort_by_distance(unit.position()).closer(unit.sight_range() + 2.0, unit.position());
    for enemy in sorted_enemies {
        // Check threat
        if can_attack(enemy, unit) {
            let harmless_worker = enemy.type_id().is_worker() && enemy.distance(unit.position()) > 3.5;
            if !harmless_worker {
                if surroundings.closest_threat.is_none() {
                    surroundings.closest_threat = Some(enemy.clone());
                }
                if in_range(enemy, unit) {
                    surroundings.in_danger = true;
                }
            }
        }
        // Targeting
        if can_attack(unit, enemy) {
            if in_range(unit, enemy) {
                // First
                if surroundings.best_target_in_range.is_none() {
                    surroundings.best_target_in_range = Some(enemy.clone());
                // Better
                } else {
                    surroundings.best_target_in_range = Some(better_of_targets(unit, &surroundings.best_target_in_range.clone().unwrap(), enemy));
                }
            } else {
                // First
                if surroundings.best_target_in_range.is_none() && surroundings.better_target_off_range.is_none() {
                    surroundings.better_target_off_range = Some(enemy.clone());
                // Better from off range
                } else if surroundings.better_target_off_range.is_some() {
                    surroundings.better_target_off_range = Some(better_of_targets(unit, &surroundings.better_target_off_range.clone().unwrap(), enemy));
                // Better than in range
                } else {
                    let better_target = Some(better_of_targets(unit, &surroundings.best_target_in_range.clone().unwrap(), enemy));
                    if better_target.clone().unwrap().tag() != surroundings.best_target_in_range.clone().unwrap().tag() {
                        surroundings.better_target_off_range = better_target;
                    }
                }
            }
        }
        surroundings.closest_counter = None; // TODO
    }
    for structure in sorted_structures {
        if surroundings.closest_structure.is_none() {
            if in_range(unit, structure) {
                surroundings.closest_structure = Some(structure.clone());
            }
        }
        // Check threat
        if can_attack(structure, unit) {
            if surroundings.closest_threat.is_none() {
                surroundings.closest_threat = Some(structure.clone());
            }
            if in_range(structure, unit) {
                surroundings.in_danger = true;
            }
        }
    }
    surroundings
}
fn can_attack(attacker: &Unit, target: &Unit) -> bool {
    if target.is_flying() && attacker.can_attack_air() {
        return true;
    }
    if !target.is_flying() && attacker.can_attack_ground() {
        return true;
    }
    false
}
fn in_range(attacker: &Unit, target: &Unit) -> bool {
    let distance = attacker.position().distance(target.position());
    if target.is_flying() {
        if distance <= attacker.air_range() {
            return true;
        }
    } else {
        if distance <= attacker.ground_range() {
            return true;
        }
    }
    false
}
fn better_of_targets(attacker: &Unit, current_target: &Unit, new_target: &Unit) -> Unit {
    let current_target_damage = attacker.real_range_vs(current_target);
    let current_target_health = (current_target.health().unwrap_or(0) + current_target.shield().unwrap_or(0)) as f32;
    let current_target_dies = current_target_health - current_target_damage <= 0.0;
    let new_target_damage = attacker.real_range_vs(new_target);
    let new_target_health = (new_target.health().unwrap_or(0) + new_target.shield().unwrap_or(0)) as f32;
    let new_target_dies = new_target_health - new_target_damage <= 0.0;
    let higher_damage_target = if current_target_damage >= new_target_damage { current_target } else { new_target };
    let lower_health_target = if current_target_health <= new_target_health { current_target } else { new_target };

    if current_target_dies {
        if new_target_dies {
            return higher_damage_target.clone();
        } else {
            return current_target.clone();
        }
    } else if new_target_dies {
        return new_target.clone();
    } else if current_target_damage != new_target_damage {
        return higher_damage_target.clone();
    }
    lower_health_target.clone()
}