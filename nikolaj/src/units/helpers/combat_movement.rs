use crate::units::helpers::threat_detection::*;
use crate::units::helpers::surroundings::*;
use crate::Nikolaj;
use rust_sc2::{prelude::*, units::AllUnits};

pub fn flee_bio(bot: &mut Nikolaj, unit: &Unit, surroundings: SurroundingsInfo) {
    let units = bot.units.clone();
    if flee_to_bunker(units.clone(), unit) {
        return;
    }
    if flee_to_mine(units.clone(), unit) {
        return;
    }
    if flee_to_medivac(units.clone(), unit) {
        return;
    }
    if flee_to_tank(units.clone(), unit) {
        return;
    }
    if flee_from_threat(bot, unit, surroundings.closest_threat) {
        return;
    }
    let idle_point = bot.strategy_data.idle_point;
    move_no_spam(unit, Target::Pos(idle_point))
}
fn flee_to_bunker(units: AllUnits, unit: &Unit) -> bool {
    let bunkers = units.my.structures.of_type(UnitTypeId::Bunker).ready();
    let nearby_bunkers = bunkers.closer(12.0, unit.position());
    if let Some(bunker) = nearby_bunkers.first() {
        if bunker.cargo_left().unwrap_or(0) >= unit.cargo_size() {
            unit.smart(Target::Tag(bunker.tag()), false);
            return true;
        }
    }
    false
}
fn flee_to_mine(units: AllUnits, unit: &Unit) -> bool {
    let mines = units.my.units.of_type_including_alias(UnitTypeId::WidowMine).ready();
    let nearby_mines = mines.closer(12.0, unit.position());
    let sorted_mines = nearby_mines.iter().sort_by_distance(unit.position());
    for mine in sorted_mines {
        if mine.buff_duration_remain().unwrap_or(1) == 0 {
            if unit.distance(mine.position()) > 3.0 {
                move_no_spam(unit, Target::Pos(mine.position()));
                return true;
            } else {
                attack_no_spam(unit, Target::Pos(mine.position()));
                return true;
            }
        }
    }
    false
}
fn flee_to_medivac(units: AllUnits, unit: &Unit) -> bool {
    let medivacs = units.my.units.of_type(UnitTypeId::Medivac);
    let nearby_medivacs = medivacs.closer(12.0, unit.position());
    if let Some(medivac) = nearby_medivacs.first() {
        if medivac.cargo_left().unwrap_or(0) >= unit.cargo_size()
            && medivac.health_percentage() > 0.5
        {
            unit.smart(Target::Tag(medivac.tag()), false);
            return true;
        }
    }
    false
}
fn flee_to_tank(units: AllUnits, unit: &Unit) -> bool {
    let tanks = units.my.units.of_type(UnitTypeId::SiegeTankSieged);
    let nearby_tanks = tanks.closer(12.0, unit.position());
    if let Some(tank) = nearby_tanks.first() {
        if unit.distance(tank.position()) > 3.0 {
            move_no_spam(unit, Target::Pos(tank.position()));
            return true;
        } else {
            attack_no_spam(unit, Target::Pos(unit.position()));
            return true;
        }
    }
    false
}
fn flee_from_threat(bot: &mut Nikolaj, unit: &Unit, threat: Option<Unit>) -> bool {
    if let Some(threat_unit) = threat {
        // Use combat grid to find furthest point
        let combat_grid = combat_grid8_pathable(bot, unit.position(), 3.0);
        let furthest_point = combat_grid.into_iter().max_by(|a, b| {
            let dist_a = a.distance(threat_unit.position());
            let dist_b = b.distance(threat_unit.position());
            dist_a
                .partial_cmp(&dist_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        if let Some(point) = furthest_point {
            unit.move_to(Target::Pos(point), false);
            return true;
        }
        // Just run
        let escape_point = unit.position().towards(threat_unit.position(), 5.0);
        unit.move_to(Target::Pos(escape_point), false);
        return true;
    }
    false
}
pub fn flee_flying_unit(bot: &mut Nikolaj, unit: &Unit, surroundings: SurroundingsInfo) {
    let in_danger = surroundings.clone().threat_level > ThreatLevel::None;
    if surroundings.clone().closest_threat.is_some()
        || (in_danger && surroundings.clone().closest_structure.is_some())
    {
        if let Some(threat) = surroundings.clone().closest_threat {
            let retreat_position = unit.position().towards(threat.position(), -5.0);
            move_no_spam(unit, Target::Pos(retreat_position));
            return;
        } else if let Some(structure) = surroundings.clone().closest_structure {
            let retreat_position = unit.position().towards(structure.position(), -5.0);
            move_no_spam(unit, Target::Pos(retreat_position));
            return;
        }
    }
    let idle_point = bot.strategy_data.idle_point;
    move_no_spam(unit, Target::Pos(idle_point));
}
fn combat_grid4(position: Point2, distance: f32) -> Vec<Point2> {
    let x = position.x;
    let y = position.y;
    let d = distance;

    vec![
        Point2 { x: x - d, y },
        Point2 { x: x + d, y },
        Point2 { x, y: y - d },
        Point2 { x, y: y + d },
    ]
}

fn combat_grid8(position: Point2, distance: f32) -> Vec<Point2> {
    let mut grid = combat_grid4(position, distance);
    let x = position.x;
    let y = position.y;
    let d = distance;

    grid.extend(vec![
        Point2 { x: x - d, y: y - d },
        Point2 { x: x - d, y: y + d },
        Point2 { x: x + d, y: y - d },
        Point2 { x: x + d, y: y + d },
    ]);
    grid
}

fn combat_grid8_pathable(bot: &mut Nikolaj, position: Point2, distance: f32) -> Vec<Point2> {
    let grid = combat_grid8(position, distance);
    grid.into_iter().filter(|p| bot.is_pathable(*p)).collect()
}

pub fn use_stimpack(unit: &Unit, surroundings: &SurroundingsInfo) {
    if (surroundings.best_target_in_range.is_some()
        || surroundings.better_target_off_range.is_some())
        && !unit.has_buff(BuffId::Stimpack)
        && !unit.has_buff(BuffId::StimpackMarauder)
        && unit.health_percentage() > 0.5
    {
        unit.use_ability(AbilityId::EffectStimMarine, false);
        unit.use_ability(AbilityId::EffectStimMarauder, false);
    }
}

pub fn attack_no_spam(unit: &Unit, target: Target) {
    if let Some(order) = &unit.order() {
        if order.0 == AbilityId::Attack {
            if order.1 == target {
                return;
            } else if let Target::Pos(position) = order.1 {
                if let Target::Pos(new_position) = target {
                    if position.distance(new_position) < 2.0 {
                        return;
                    }
                }
            }
        }
    }
    if let Target::Pos(position) = target {
        if unit.distance(position) < 4.0 {
            return;
        }
    }
    unit.attack(target, false);
}

pub fn move_no_spam(unit: &Unit, target: Target) {
    if let Some(order) = &unit.order() {
        if order.0 == AbilityId::Move {
            if order.1 == target {
                return;
            } else if let Target::Pos(position) = order.1 {
                if let Target::Pos(new_position) = target {
                    if position.distance(new_position) < 2.0 {
                        return;
                    }
                }
            }
        }
    }
    if let Target::Pos(position) = target {
        if unit.distance(position) < 4.0 {
            return;
        }
    }
    unit.move_to(target, false);
}

pub fn unsiege(bot: &mut Nikolaj, unit: &Unit) {
    let unsiege_timer = bot.combat_info.get_unsiege_timer(unit.tag());
    if let Some(timer) = unsiege_timer {
        if timer.unsiege_in <= 0.0 {
            bot.combat_info.remove_unsiege_timer(unit.tag());
            if unit.type_id() == UnitTypeId::SiegeTankSieged {
                unit.use_ability(AbilityId::UnsiegeUnsiege, false);
            } else if unit.type_id() == UnitTypeId::WidowMineBurrowed {
                unit.use_ability(AbilityId::BurrowUpWidowMine, false);
            }
        }
    } else {
        if unit.type_id() == UnitTypeId::SiegeTankSieged
            || unit.type_id() == UnitTypeId::WidowMineBurrowed
        {
            bot.combat_info.add_unsiege_timer(unit.tag());
        }
    }
}

pub fn siege_up(bot: &mut Nikolaj, unit: &Unit) {
    bot.combat_info.remove_unsiege_timer(unit.tag());
    if unit.type_id() == UnitTypeId::SiegeTank {
        unit.use_ability(AbilityId::SiegeModeSiegeMode, false);
    } else if unit.type_id() == UnitTypeId::WidowMine {
        unit.use_ability(AbilityId::BurrowDownWidowMine, false);
    }
}

pub fn force_unsiege(bot: &mut Nikolaj, unit: &Unit) {
    bot.combat_info.remove_unsiege_timer(unit.tag());
    if unit.type_id() == UnitTypeId::SiegeTankSieged {
        unit.use_ability(AbilityId::UnsiegeUnsiege, false);
    } else if unit.type_id() == UnitTypeId::WidowMineBurrowed {
        unit.use_ability(AbilityId::BurrowUpWidowMine, false);
    }
}

pub fn get_closest_harass_point(bot: &Nikolaj, unit: &Unit) -> Point2 {
    let harass_points = &bot.strategy_data.harass_points;
    let attack_point = bot.strategy_data.attack_point;

    if let Some(closest_point) = harass_points.iter().min_by(|a, b| {
        let dist_a = unit.distance(**a);
        let dist_b = unit.distance(**b);
        dist_a
            .partial_cmp(&dist_b)
            .unwrap_or(std::cmp::Ordering::Equal)
    }) {
        return *closest_point;
    } else {
        return attack_point;
    }
}
pub fn get_closest_repair_point(bot: &Nikolaj, unit: &Unit) -> Point2 {
    let repair_points = &bot.strategy_data.repair_points;
    let idle_point = bot.strategy_data.idle_point;
    if let Some(closest_point) = repair_points.iter().min_by(|a, b| {
        let dist_a = unit.distance(**a);
        let dist_b = unit.distance(**b);
        dist_a
            .partial_cmp(&dist_b)
            .unwrap_or(std::cmp::Ordering::Equal)
    }) {
        return *closest_point;
    } else {
        return idle_point;
    }
}
pub fn kd8_charge(unit: &Unit, surroundings: &SurroundingsInfo) -> bool {
    if let Some(threat) = surroundings.clone().closest_threat {
        if let Some(abilities) = unit.abilities() {
            if !abilities.contains(&AbilityId::KD8ChargeKD8Charge) {
                return false;
            }
            let target_position = threat.position().towards(unit.position(), 4.0);
            unit.command(
                AbilityId::KD8ChargeKD8Charge,
                Target::Pos(target_position),
                false,
            );
            return true;
        }
    }
    false
}

pub fn banshee_cloak(unit: &Unit, surroundings: &SurroundingsInfo) -> bool {
    let in_danger = surroundings.clone().threat_level > ThreatLevel::None;
    if let Some(abilities) = unit.abilities() {
        if abilities.contains(&AbilityId::BehaviorCloakOnBanshee) && !unit.is_cloaked() {
            if surroundings.clone().closest_threat.is_some() || in_danger {
                unit.use_ability(AbilityId::BehaviorCloakOnBanshee, false);
                return true;
            }
        }
    }
    false
}
