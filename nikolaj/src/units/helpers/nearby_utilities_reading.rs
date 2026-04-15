use crate::Nikolaj;
use rust_sc2::prelude::*;


pub fn get_closest_medivac(bot: &Nikolaj, unit: &Unit) -> Option<Unit> {
    let medivacs = bot
        .units
        .my
        .units
        .of_type(UnitTypeId::Medivac)
        .ready()
        .closer(20.0, unit.position());
    if !medivacs.is_empty() {
        return medivacs.closest(unit.position()).cloned();
    }
    None
}

const MINIMAL_TANK_DISTANCE: f32 = 15.0;

pub fn get_closest_tank_cover(bot: &Nikolaj, unit: &Unit) -> Option<Unit> {
    for possible_tank in bot
        .units
        .my
        .units
        .of_type(UnitTypeId::SiegeTankSieged)
        .closer(MINIMAL_TANK_DISTANCE, unit.position())
        .iter()
        .sort_by_distance(unit.position())
    {
        return Some(possible_tank.clone());
    }
    None
}

pub fn get_standing_on_depot(bot: &Nikolaj, unit: &Unit) -> Option<Unit> {
    let depots = bot
        .units
        .my
        .structures
        .of_type(UnitTypeId::SupplyDepotLowered)
        .closer(1.5, unit.position());
    if !depots.is_empty() {
        return depots.closest(unit.position()).cloned();
    }
    None
}