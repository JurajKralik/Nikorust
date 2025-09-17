use crate::Nikolaj;
use rust_sc2::{prelude::*, units::AllUnits};
use crate::units::helpers::surroundings::*;


pub fn flee_bio(bot: &mut Nikolaj, unit: &Unit, surroundings: SurroundingsInfo) {
    let units = bot.units.clone();
    if flee_to_bunker(units.clone(), unit) {
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
    unit.move_to(Target::Pos(idle_point), false);
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
fn flee_to_medivac(units: AllUnits, unit: &Unit) -> bool {
    let medivacs = units
        .my
        .units
        .of_type(UnitTypeId::Medivac);
    let nearby_medivacs = medivacs.closer(12.0, unit.position());
    if let Some(medivac) = nearby_medivacs.first() {
        if medivac.cargo_left().unwrap_or(0) >= unit.cargo_size() {
            unit.smart(Target::Tag(medivac.tag()), false);
            return true;
        }
    }
    false
}
fn flee_to_tank(units: AllUnits, unit: &Unit) -> bool {
    let tanks = units
        .my
        .units
        .of_type(UnitTypeId::SiegeTankSieged);
    let nearby_tanks = tanks.closer(12.0, unit.position());
    if let Some(tank) = nearby_tanks.first() {
        if unit.distance(tank.position()) > 3.0 {
            unit.move_to(Target::Pos(tank.position()), false);
            return true;
        } else {
            unit.attack(Target::Pos(unit.position()), false);
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
            dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
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
    grid.into_iter()
        .filter(|p| bot.is_pathable(*p))
        .collect()
}