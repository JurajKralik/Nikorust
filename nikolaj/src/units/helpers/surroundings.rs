use crate::units::helpers::targeting::*;
use crate::units::helpers::threat_detection::*;
use crate::Nikolaj;
use rust_sc2::prelude::*;

#[derive(Clone)]
pub struct SurroundingsInfo {
    pub best_target_in_range: Option<Unit>,
    pub better_target_off_range: Option<Unit>,
    pub closest_threat: Option<Unit>,
    pub closest_counter: Option<Unit>,
    pub closest_structure: Option<Unit>,
    pub threat_level: ThreatLevel,
}
impl Default for SurroundingsInfo {
    fn default() -> Self {
        SurroundingsInfo {
            best_target_in_range: None,
            better_target_off_range: None,
            closest_threat: None,
            closest_counter: None,
            closest_structure: None,
            threat_level: ThreatLevel::None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SurroundingsOptions {
    pub extra_avoidance: f32,
}

impl Default for SurroundingsOptions {
    fn default() -> Self {
        SurroundingsOptions {
            extra_avoidance: 0.0,
        }
    }
}

pub fn get_surroundings_info(bot: &mut Nikolaj, unit: &Unit, options: SurroundingsOptions) -> SurroundingsInfo {
    get_surroundings_info_with_options(bot, unit, options)
}

pub fn get_surroundings_info_with_options(bot: &mut Nikolaj, unit: &Unit, options: SurroundingsOptions) -> SurroundingsInfo {
    let targeting_priorities = get_targeting_priorities(&unit.type_id());
    let threat_levels = get_threat_levels(&unit.type_id());
    let mut surroundings = SurroundingsInfo::default();
    
    // Use enemy army snapshots instead of direct enemy units
    let enemy_snapshots = bot.strategy_data.enemy_army.units.clone();
    let sorted_snapshots: Vec<_> = enemy_snapshots
        .iter()
        .filter(|snapshot| unit.position().distance(snapshot.unit.position()) <= unit.sight_range() + 2.0)
        .collect();
    
    let enemy_structures = bot.units.enemy.structures.clone();
    let sorted_structures = enemy_structures
        .iter()
        .sort_by_distance(unit.position())
        .closer(unit.sight_range() + 2.0, unit.position());
    
    for snapshot in sorted_snapshots {
        let enemy = &snapshot.unit;
        
        // Check threat
        if can_attack(enemy, unit) {
            let harmless_worker =
                enemy.type_id().is_worker() && enemy.distance(unit.position()) > 3.5;
            if !harmless_worker {
                if surroundings.closest_threat.is_none() {
                    surroundings.closest_threat = Some(enemy.clone());
                }
                if in_range_with_avoidance(enemy, unit, options.extra_avoidance) {
                    if surroundings.threat_level == ThreatLevel::None {
                        surroundings.threat_level = ThreatLevel::Danger;
                    }
                }
            }
        }
        
        // Targeting
        if can_attack(unit, enemy) {
            if snapshot.is_snapshot {
                continue;
            }
            if in_range(unit, enemy) {
                // Better target
                if let Some(best_target) = &surroundings.best_target_in_range {
                    surroundings.best_target_in_range =
                        Some(better_of_targets(unit, &best_target, enemy, targeting_priorities.clone()));
                // First
                } else {
                    surroundings.best_target_in_range = Some(enemy.clone());
                }
            } else {
                // First
                if surroundings.best_target_in_range.is_none()
                    && surroundings.better_target_off_range.is_none()
                {
                    surroundings.better_target_off_range = Some(enemy.clone());
                // Better from off range
                } else if let Some(better_target_off_range) = &surroundings.better_target_off_range
                {
                    surroundings.better_target_off_range =
                        Some(better_of_targets(unit, &better_target_off_range, enemy, targeting_priorities.clone()));
                // Better than in range
                } else if let Some(best_target_in_range) = &surroundings.best_target_in_range {
                    let better_target = better_of_targets(unit, &best_target_in_range, enemy, targeting_priorities.clone());
                    if better_target.tag() != best_target_in_range.tag() {
                        surroundings.better_target_off_range = Some(better_target);
                    }
                }
            }
        }
        
        let higher_threat_level = threat_levels.compare_threat_levels(surroundings.closest_counter.clone(), enemy.clone());
        if let Some(higher_threat) = higher_threat_level {
            surroundings.closest_counter = Some(higher_threat);
        }
    }

    // Avoid fake better target
    match (
        &surroundings.better_target_off_range,
        &surroundings.best_target_in_range,
    ) {
        (Some(better), Some(best)) => {
            if best.tag() == better_of_targets(unit, better, best, targeting_priorities).tag() {
                surroundings.better_target_off_range = None;
            }
        }
        _ => {}
    }

    for structure in sorted_structures {
        if surroundings.closest_structure.is_none() {
            if in_range(unit, structure) {
                surroundings.closest_structure = Some(structure.clone());
            }
        }
        // Check threat
        if structure_can_attack(structure, unit) {
            if surroundings.closest_threat.is_none() {
                surroundings.closest_threat = Some(structure.clone());
            }
            if in_range_with_avoidance(structure, unit, options.extra_avoidance) {
                if surroundings.threat_level == ThreatLevel::None {
                    surroundings.threat_level = ThreatLevel::Danger;
                }
            }
        }
    }
    surroundings
}
pub fn can_attack(attacker: &Unit, target: &Unit) -> bool {
    if attacker.type_id() == UnitTypeId::WidowMine || attacker.type_id() == UnitTypeId::WidowMineBurrowed {
        return true;
    }
    if target.is_flying() && attacker.can_attack_air() {
        return true;
    }
    if !target.is_flying() && attacker.can_attack_ground() {
        return true;
    }
    false
}
pub fn structure_can_attack(attacker: &Unit, target: &Unit) -> bool {
    if attacker.type_id() == UnitTypeId::PhotonCannon || attacker.type_id() == UnitTypeId::Bunker {
        return true;
    }
    if target.is_flying() && (attacker.type_id() == UnitTypeId::SporeCrawler || attacker.type_id() == UnitTypeId::MissileTurret) {
        return true;
    }
    if !target.is_flying() && (attacker.type_id() == UnitTypeId::SpineCrawler || attacker.type_id() == UnitTypeId::PlanetaryFortress) {
        return true;
    }
    false
}
pub fn in_range(attacker: &Unit, target: &Unit) -> bool {
    in_range_with_avoidance(attacker, target, 0.0)
}

pub fn in_range_with_avoidance(attacker: &Unit, target: &Unit, extra_avoidance: f32) -> bool {
    let distance = attacker.position().distance(target.position());
    if target.is_flying() {
        if distance <= attacker.air_range() + extra_avoidance {
            return true;
        }
    } else {
        if distance <= attacker.ground_range() + extra_avoidance {
            return true;
        }
    }
    false
}
fn better_of_targets(attacker: &Unit, current_target: &Unit, new_target: &Unit, targeting_priorities: TargetingPriorities) -> Unit {
    let current_target_damage = attacker.real_weapon_vs(current_target).damage as f32;
    let current_target_health =
        (current_target.health() + current_target.shield()) as f32;
    let current_target_dies = current_target_health - current_target_damage <= 0.0;
    let new_target_damage = attacker.real_weapon_vs(new_target).damage as f32;
    let new_target_health =
        (new_target.health() + new_target.shield()) as f32;
    let new_target_dies = new_target_health - new_target_damage <= 0.0;
    let higher_damage_target = if current_target_damage >= new_target_damage {current_target} else {new_target};
    let lower_health_target = if current_target_health <= new_target_health {current_target} else {new_target};
    let higher_priority_target = targeting_priorities.compare_priority(current_target.clone(), new_target.clone());

    if current_target_dies {
        if new_target_dies {
            if let Some(higher_priority) = higher_priority_target {
                return higher_priority;
            }
            return higher_damage_target.clone();
        } else {
            return current_target.clone();
        }
    } else if new_target_dies {
        return new_target.clone();
    } else if current_target_damage != new_target_damage {
        if let Some(higher_priority) = higher_priority_target {
            return higher_priority;
        }
        return higher_damage_target.clone();
    }
    lower_health_target.clone()
}
