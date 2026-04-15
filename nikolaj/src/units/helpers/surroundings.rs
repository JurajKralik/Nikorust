use crate::units::helpers::targeting::*;
use crate::units::helpers::threat_detection::*;
use crate::strategy::unit_snapshot::UnitSnapshot;
use crate::Nikolaj;
use rust_sc2::prelude::*;


#[derive(Clone)]
pub struct SurroundingsInfo {
    pub unit: Unit,
    pub options: SurroundingsOptions,
    pub best_target_in_range: Option<Unit>,
    pub better_target_off_range: Option<Unit>,
    pub closest_threat: Option<Unit>,
    pub closest_counter: Option<Unit>,
    pub closest_structure: Option<Unit>,
    pub threat_level: ThreatLevel,
}


#[derive(Clone, Debug)]
pub struct SurroundingsOptions {
    pub extra_avoidance: f32,
    pub advantage_only: bool,
}


impl Default for SurroundingsOptions {
    fn default() -> Self {
        SurroundingsOptions {
            extra_avoidance: 0.0,
            advantage_only: false,
        }
    }
}


pub fn get_surroundings_info(bot: &mut Nikolaj, unit: &Unit, options: SurroundingsOptions) -> SurroundingsInfo {
    let mut surroundings = SurroundingsInfo {
        unit: unit.clone(),
        options,
        best_target_in_range: None,
        better_target_off_range: None,
        closest_threat: None,
        closest_counter: None,
        closest_structure: None,
        threat_level: ThreatLevel::None,
    };

    let targeting_priorities = get_targeting_priorities(&unit.type_id());
    let threat_levels = get_threat_levels(&unit.type_id());
    let valid_range = unit.sight_range() + 2.0;
    let enemy_snapshots = bot.strategy_data.enemy_army.units.clone();
    let sorted_snapshots = get_sorted_valid_snapshots(&enemy_snapshots, unit.position(), valid_range);
    let enemy_structures = bot.units.enemy.structures.clone();
    let sorted_structures = get_sorted_valid_structures(&enemy_structures, unit.position(), valid_range);

    for snapshot in sorted_snapshots {
        let enemy = &snapshot.unit;
        
        compare_closest_threat(&mut surroundings, enemy);
        compare_targeting(&mut surroundings, enemy, &targeting_priorities);
        compare_closest_counter(&mut surroundings, enemy, &threat_levels);
    }

    validate_better_off_range_target(&mut surroundings, &targeting_priorities);

    for structure in sorted_structures {
        compare_closest_structure(&mut surroundings, &structure);
        compare_threat_structure(&mut surroundings, &structure);
    }

    bot.debugger.add_surroundings(surroundings.clone());
    surroundings
}


fn get_sorted_valid_snapshots(enemy_snapshots: &Vec<UnitSnapshot>, position: Point2, range: f32) -> Vec<UnitSnapshot> {
    let mut valid_snapshots: Vec<_> = enemy_snapshots
        .iter()
        .filter(|snapshot| position.distance(snapshot.position()) <= range)
        .cloned()
        .collect();
    valid_snapshots.retain(|snapshot| snapshot.is_position_still_relevant);
    valid_snapshots.retain(|snapshot| !snapshot.is_ignored_unit);
    valid_snapshots.sort_by(|a, b| {
        let dist_a = position.distance(a.position());
        let dist_b = position.distance(b.position());
        dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
    });
    
    valid_snapshots
}


fn get_sorted_valid_structures(enemy_structures: &Units, position: Point2, range: f32) -> Vec<Unit> {
    let mut valid_structures: Vec<_> = enemy_structures
        .iter()
        .filter(|structure| position.distance(structure.position()) <= range)
        .cloned()
        .collect();
    
    valid_structures.sort_by(|a, b| {
        let dist_a = position.distance(a.position());
        let dist_b = position.distance(b.position());
        dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
    });
    
    valid_structures
}


fn compare_closest_threat(surroundings: &mut SurroundingsInfo, enemy: &Unit) {
    let unit = &surroundings.unit;
    if !can_attack(enemy, unit) {
        return;
    }

    if surroundings.closest_threat.is_none() {
        surroundings.closest_threat = Some(enemy.clone());
    }

    let extra_avoidance = surroundings.options.extra_avoidance;
    if in_range_with_avoidance(enemy, unit, extra_avoidance) {
        surroundings.threat_level = ThreatLevel::Danger;
    }
}


fn compare_targeting(surroundings: &mut SurroundingsInfo, enemy: &Unit, targeting_priorities: &TargetingPriorities) {
    let unit = &surroundings.unit;
    if !can_attack(unit, enemy) {
        return;
    }

    if in_range(unit, enemy) {
        surroundings.best_target_in_range = match &surroundings.best_target_in_range {
            None => Some(enemy.clone()),
            Some(current_best) => Some(better_of_targets(unit, current_best, enemy, targeting_priorities)),
        };
    } else {
        surroundings.better_target_off_range = match &surroundings.better_target_off_range {
            None => Some(enemy.clone()),
            Some(current_off_range_target) => {Some(better_of_targets(unit, current_off_range_target, enemy, targeting_priorities))},
            // Off range target will be cleaned up if worse than in range target after looping
        };
    }
}


fn compare_closest_counter(surroundings: &mut SurroundingsInfo, enemy: &Unit, threat_levels: &ThreatLevels) {
    surroundings.closest_counter = threat_levels.get_higher_threat_unit(surroundings.closest_counter.clone(), enemy.clone());
}


fn validate_better_off_range_target(surroundings: &mut SurroundingsInfo, targeting_priorities: &TargetingPriorities) {
    if let Some(off_range_target) = &surroundings.better_target_off_range {
        if can_attack(off_range_target, &surroundings.unit) && surroundings.options.advantage_only {
            surroundings.better_target_off_range = None;
            return;
        }
    }

    match (
        &surroundings.better_target_off_range,
        &surroundings.best_target_in_range,
    ) {
        (Some(better), Some(best)) => {
            if best.tag() == better_of_targets(&surroundings.unit, better, best, &targeting_priorities).tag() {
                surroundings.better_target_off_range = None;
            }
        }
        _ => {}
    }
}


fn compare_closest_structure(surroundings: &mut SurroundingsInfo, structure: &Unit) {
    let unit = &surroundings.unit;
    if surroundings.closest_structure.is_none() {
        if in_range(unit, &structure) {
            surroundings.closest_structure = Some(structure.clone());
        }
    }
}


fn compare_threat_structure(surroundings: &mut SurroundingsInfo, structure: &Unit) {
    let unit = &surroundings.unit;
    if !structure_can_attack(&structure, unit) {
        return;
    }

    if let Some(closest_threat) = &surroundings.closest_threat {
        if closest_threat.distance(unit) < structure.distance(unit) {
            return;
        }
    }

    surroundings.closest_threat = Some(structure.clone());

    if in_range_with_avoidance(&structure, unit, surroundings.options.extra_avoidance) 
        && surroundings.threat_level == ThreatLevel::None {
        surroundings.threat_level = ThreatLevel::Danger;
    }
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


fn better_of_targets(attacker: &Unit, current_target: &Unit, new_target: &Unit, targeting_priorities: &TargetingPriorities) -> Unit {
    let current_target_damage = attacker.real_weapon_vs(current_target).damage as f32;
    let current_target_health = (current_target.health() + current_target.shield()) as f32;
    let current_target_dies = current_target_health - current_target_damage <= 0.0;

    let new_target_damage = attacker.real_weapon_vs(new_target).damage as f32;
    let new_target_health = (new_target.health() + new_target.shield()) as f32;
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
