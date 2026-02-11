use crate::Nikolaj;
use rust_sc2::prelude::*;


pub fn refresh_points(bot: &mut Nikolaj) {
    refresh_idle_point(bot);
    refresh_defense_point(bot);
    refresh_attack_point(bot);
    refresh_army_center(bot);
    refresh_harass_points(bot);
    refresh_repair_points(bot);
}


fn refresh_idle_point(bot: &mut Nikolaj) {
    let ready_townhalls = bot.units.my.townhalls.ready();

    let mut closest_base: Option<Unit> = None;
    let mut shortest_path_distance = i32::MAX;
    let mut shortest_path: Option<Vec<Point2>> = None;
    
    for base in ready_townhalls {
        if let Some(path) = bot.get_path(base.position(), bot.enemy_start, PathfindingUnitType::Ground, false, false) {
            let path_distance = path.0.len() as i32;
            if path_distance < shortest_path_distance {
                shortest_path_distance = path_distance;
                closest_base = Some(base);
                shortest_path = Some(path.0);
            }
        }
    }
    
    if let Some(base) = closest_base {
        if let Some(path) = shortest_path {
            if let Some(target_point) = path.iter().nth(5) {
                bot.strategy_data.idle_point = *target_point;
            } else if let Some(last_point) = path.last() {
                bot.strategy_data.idle_point = *last_point;
            } else {
                bot.strategy_data.idle_point = base.position().towards(bot.enemy_start, 5.0);
            }
        }
    } else {
        bot.strategy_data.idle_point = bot.start_location;
    }
}

fn refresh_defense_point(bot: &mut Nikolaj) {
    let enemies = bot.units.enemy.units.clone();
    bot.strategy_data.defend = false;
    if enemies.is_empty() {
        return;
    }
    let mut closest_enemy: Option<Unit> = None;
    let mut closest_distance = f32::MAX;
    for enemy in enemies {
        if !enemy.can_attack() {
            continue;
        }
        if enemy.position().distance(bot.enemy_start) < bot.enemy_start.distance(bot.start_location)/2.0 {
            continue;
        }
        if let Some(closest) = closest_enemy.as_ref() {
            let closest_structure = bot.units.my.structures.closest(closest.position());
            if let Some(structure) = closest_structure {
                let distance = enemy.position().distance(structure.position());
                if distance < closest_distance {
                    closest_distance = distance;
                    closest_enemy = Some(enemy.clone());
                }
            }
        } else {
            closest_enemy = Some(enemy.clone());
            if let Some(closest) = closest_enemy.as_ref() {
                let closest_structure = bot.units.my.structures.closest(closest.position());
                if let Some(structure) = closest_structure {
                    closest_distance = enemy.position().distance(structure.position());
                }
            }
        }
    }
    if let Some(closest) = closest_enemy {
        let defense_point = {
            if let Some(closest_structure) = bot.units.my.structures.closest(closest.position()) {
                Some(
                    closest_structure
                        .position()
                        .towards(closest.position(), 4.0),
                )
            } else {
                None
            }
        };
        if let Some(point) = defense_point {
            bot.strategy_data.defend = true;
            bot.strategy_data.defense_point = point;
        } else {
            bot.strategy_data.defend = false;
        }
    }
}

fn refresh_attack_point(bot: &mut Nikolaj) {
    let enemy_structures = bot.units.enemy.structures.clone();
    enemy_structures.iter().sort_by_distance(bot.start_location);
    let my_units = bot.units.my.units.clone();
    let ramp = bot
        .ramps
        .enemy
        .barracks_in_middle()
        .unwrap_or(Point2 { x: 0.0, y: 0.0 })
        .towards(bot.enemy_start, -2.0);

    if !enemy_structures.is_empty() {
        let mut closest_structure: Option<Unit> = None;
        let mut closest_distance = f32::MAX;
        for structure in enemy_structures {
            if let Some(closest) = closest_structure.as_ref() {
                if closest.type_id().is_townhall() {
                    break;
                }
                let distance = structure.position().distance(bot.start_location);
                if distance < closest_distance || structure.type_id().is_townhall() {
                    closest_distance = distance;
                    closest_structure = Some(structure.clone());
                }
            } else {
                closest_structure = Some(structure.clone());
                closest_distance = structure.position().distance(bot.start_location);
            }
        }
        if let Some(closest) = closest_structure {
            bot.strategy_data.attack_point = closest.position();
            return;
        }
    } else if !my_units.is_empty() {
        let closest_unit = my_units.closest(bot.enemy_start);
        if let Some(unit) = closest_unit {
            let ramp_base_distance = ramp.distance(bot.enemy_start);
            let unit_base_distance = unit.position().distance(bot.enemy_start);
            if unit_base_distance > ramp_base_distance + 3.0 {
                bot.strategy_data.attack_point = ramp;
                return;
            } else {
                bot.strategy_data.attack_point = bot.enemy_start;
                return;
            }
        }
    } else {
        bot.strategy_data.attack_point = ramp;
    }
}
fn refresh_army_center(bot: &mut Nikolaj) {
    let attack_point = bot.strategy_data.attack_point;
    let my_units = bot.units.my.units.clone();
    // Prefer bio units for center
    if bot.unit_count(UnitTypeId::Marine) + bot.unit_count(UnitTypeId::Marauder) > 6 {
        let mut bio_units = Units::new();
        for unit in my_units.iter().sort_by_distance(attack_point) {
            if unit.type_id() != UnitTypeId::Marine && unit.type_id() != UnitTypeId::Marauder {
                continue;
            }
            if bio_units.len() >= 6 {
                break;
            }
            bio_units.push(unit.clone());
        }
        if let Some(center) = bio_units.center() {
            bot.strategy_data.army_center = center;
            return;
        }
    }
    // Any combat units
    let mut frontal_units = Units::new();
    for unit in my_units.iter().sort_by_distance(attack_point) {
        if frontal_units.len() >= 6 {
            break;
        }
        if unit.can_attack() 
            && !unit.type_id().is_worker() 
            && unit.type_id() != UnitTypeId::Banshee 
            && unit.type_id() != UnitTypeId::Reaper 
            && unit.type_id() != UnitTypeId::VikingAssault 
            && unit.type_id() != UnitTypeId::VikingFighter 
        {
            frontal_units.push(unit.clone());
        }
    }
    if let Some(center) = frontal_units.center() {
        bot.strategy_data.army_center = center;
    } else {
        bot.strategy_data.army_center = attack_point;
    }
}
fn refresh_harass_points(bot: &mut Nikolaj) {
    let enemy_bases = bot.units.enemy.townhalls.clone();
    if !enemy_bases.is_empty() {
        let mut harass_points: Vec<Point2> = Vec::new();
        for base in enemy_bases {
            let minerals = bot.units.mineral_fields.closer(13.0, base.position());
            let mineral_center = minerals.center();
            if let Some(center) = mineral_center {
                let harass_position = center.towards(base.position(), 2.0);
                harass_points.push(harass_position);
            } else {
                harass_points.push(base.position());
            }
        }
        bot.strategy_data.harass_points = harass_points;
    } else {
        let mut harass_points: Vec<Point2> = Vec::new();
        let minerals = bot.units.mineral_fields.closer(13.0, bot.enemy_start);
        let mineral_center = minerals.center();
        if let Some(center) = mineral_center {
            let harass_position = center.towards(bot.enemy_start, 2.0);
            harass_points.push(harass_position);
            bot.strategy_data.harass_points = harass_points;
            return;
        } else {
            harass_points.push(bot.enemy_start);
            bot.strategy_data.harass_points = harass_points;
            return;
        }
    }
}
fn refresh_repair_points(bot: &mut Nikolaj) {
    let bases = bot.units.my.townhalls.ready();
    let mut repair_points: Vec<Point2> = Vec::new();
    for base in bases {
        let workers = bot.units.my.workers.closer(20.0, base.position());
        if workers.len() > 5 {
            let repair_position = base.position().towards(bot.enemy_start, 3.0);
            repair_points.push(repair_position);
        }
    }
    if repair_points.is_empty() {
        repair_points.push(bot.start_location);
    }
    bot.strategy_data.repair_points = repair_points;
}