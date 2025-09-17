use crate::Nikolaj;
use rust_sc2::prelude::*;


pub fn strategy_step(bot: &mut Nikolaj) {
    refresh_idle_point(bot);
    refresh_defense_point(bot);
    refresh_attack_point(bot);
    refresh_harass_points(bot);
    refresh_repair_points(bot);
}
fn refresh_idle_point(bot: &mut Nikolaj) {
    let bases_amount = bot.units.my.townhalls.ready().len();
    if bases_amount == 1 {
        if let Some(base) = bot.units.my.townhalls.ready().first() {
            if base.position() == bot.start_location {
                let ramp = bot.ramps.my.barracks_in_middle().unwrap_or(Point2 { x: 0.0, y: 0.0 });
                bot.strategy_data.idle_point = ramp.towards(bot.start_location, 8.0);
                return;
            }
        }
    }
    if bases_amount > 0 {
        let ready_townhalls = bot.units.my.townhalls.ready();
        let mut bases: Vec<_> = ready_townhalls.iter().collect();
        bases.sort_unstable_by(|a, b| b.tag().cmp(&a.tag()));
        let mut frontal_base: Option<&Unit> = None;
        for base in bases {
            if let Some(frontal) = frontal_base {
                let distance_to_enemy = base.position().distance(bot.enemy_start) + 5.0;
                let frontal_distance_to_enemy = frontal.position().distance(bot.enemy_start);
                if distance_to_enemy < frontal_distance_to_enemy {
                    frontal_base = Some(base);
                }
            } else {
                frontal_base = Some(base);
            }
        }
        if let Some(frontal) = frontal_base {
            bot.strategy_data.idle_point = frontal.position().towards(bot.enemy_start, 5.0);
            return;
        }
    }
    bot.strategy_data.idle_point = bot.start_location;
}
fn refresh_defense_point(bot: &mut Nikolaj) {
    let enemies = bot.units.enemy.units.clone();
    if enemies.is_empty() {
        bot.strategy_data.defend = false;
        return;
    }
    let mut closest_enemy: Option<Unit> = None;
    let mut closest_distance = f32::MAX;
    for enemy in enemies {
        if !enemy.can_attack() {
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
                Some(closest_structure.position().towards(closest.position(), 4.0))
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
    let ramp = bot.ramps.enemy.barracks_in_middle().unwrap_or(Point2 { x: 0.0, y: 0.0 }).towards(bot.enemy_start, -2.0);

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


pub struct StrategyData {
    pub idle_point: Point2,
    pub defense_point: Point2,
    pub attack_point: Point2,
    pub harass_points: Vec<Point2>,
    pub repair_points: Vec<Point2>,
    pub defend: bool,
    pub attack: bool,
}

impl Default for StrategyData {
    fn default() -> Self {
        StrategyData {
            idle_point: Point2::new(0.0, 0.0),
            defense_point: Point2::new(0.0, 0.0),
            attack_point: Point2::new(0.0, 0.0),
            harass_points: Vec::new(),
            repair_points: Vec::new(),
            defend: false,
            attack: false,
        }
    }
}