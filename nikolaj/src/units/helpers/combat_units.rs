use crate::Nikolaj;
use rust_sc2::prelude::*;
use rust_sc2::units::AllUnits;
use crate::units::marine::*;
use crate::units::marauder::*;
use crate::units::reaper::*;
use crate::units::ghost::*;
use crate::units::hellion::*;
use crate::units::tank::*;
use crate::units::widowmine::*;
use crate::units::cyclone::*;
use crate::units::thor::*;
use crate::units::viking::*;
use crate::units::medivac::*;
use crate::units::raven::*;
use crate::units::banshee::*;
use crate::units::battlecruiser::*;


pub fn army_step(bot: &mut Nikolaj) {
    let units = get_combat_units(&bot.units.clone());
    for unit in &units {
        match unit.type_id() {
            UnitTypeId::Marine => marine_control(bot, unit),
            UnitTypeId::Marauder => marauder_control(bot, unit),
            UnitTypeId::Reaper => reaper_control(bot, unit),
            UnitTypeId::Ghost => ghost_control(bot, unit),
            UnitTypeId::Hellion | UnitTypeId::HellionTank => hellion_control(bot, unit),
            UnitTypeId::SiegeTank | UnitTypeId::SiegeTankSieged => tank_control(bot, unit),
            UnitTypeId::WidowMine | UnitTypeId::WidowMineBurrowed => widowmine_control(bot, unit),
            UnitTypeId::Cyclone => cyclone_control(bot, unit),
            UnitTypeId::Thor => thor_control(bot, unit),
            UnitTypeId::VikingFighter => viking_control(bot, unit),
            UnitTypeId::Medivac => medivac_control(bot, unit),
            UnitTypeId::Raven => raven_control(bot, unit),
            UnitTypeId::Banshee => banshee_control(bot, unit),
            UnitTypeId::Battlecruiser => battlecruiser_control(bot, unit),
            _ => {}
        }
    }
}
fn get_combat_units(units: &AllUnits) -> Units {
    let mut combat_units: Units = Units::new();

    for unit in &units.my.units {
        if !unit.type_id().is_worker() {
            combat_units.push(unit.clone());
        }
    }
    combat_units
}