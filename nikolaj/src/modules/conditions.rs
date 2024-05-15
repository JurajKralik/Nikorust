use crate::Nikolaj;
use rust_sc2::prelude::*;
use crate::params::*;

pub(crate) fn get_conditions(bot: &mut Nikolaj, structure: &UnitTypeId) -> bool {
    match structure {
        UnitTypeId::SupplyDepot => {
            let pending = bot.already_pending(UnitTypeId::SupplyDepot);
            let supply_left = bot.supply_left;
            let supply_cap = bot.supply_cap;
            let supply_used = bot.supply_used;

            //stop
            if supply_cap > 199 || bot.units.my.townhalls.is_empty() {
                return false;
            }
            //classic
            if supply_left < 6 && pending == 0 {
                return true;
            }
            //supply block close
            if supply_left < 6 && pending < 2 && supply_used > 30 {
                return true;
            }
            //supply block too close
            if supply_used > 45 && supply_left < 3 && pending < 3 {
                return true;
            }
            //lategame
            if supply_cap > 40 && supply_left < 8 && pending == 0 {
                return true;
            }
            //close middle ramp
            if let Some(depot_in_middle) = bot.ramps.my.depot_in_middle() {
                if bot
                    .units
                    .my
                    .structures
                    .closer(1.5, depot_in_middle)
                    .is_empty()
                    && bot
                        .units
                        .my
                        .structures
                        .of_type(UnitTypeId::Barracks)
                        .closer(4.0, depot_in_middle)
                        .is_empty()
                    && !bot
                        .units
                        .my
                        .structures
                        .of_type_including_alias(UnitTypeId::Barracks)
                        .is_empty()
                    && pending == 0
                {
                    return true;
                }
            }

            false
        }
        UnitTypeId::Refinery => {
            if bot.already_pending(UnitTypeId::Refinery) > 0
                || bot.bases.len() == 0
                || bot
                    .units
                    .my
                    .structures
                    .of_type_including_alias(UnitTypeId::Barracks)
                    .is_empty()
            {
                return false;
            }
            for base in bot.units.my.structures.find_tags(&bot.bases) {
                let geysers = bot.units.vespene_geysers.closer(10.0, base.position());
                for geyser in geysers {
                    if let Some(gas) = geyser.vespene_contents() {
                        if gas > 0 && bot.units.my.structures.closer(1.0, geyser.position().clone()).is_empty(){
                            return true;
                        }
                    }
                }
            }
            false
        }
        UnitTypeId::EngineeringBay => {
            let bio_amount = bot.units.my.units.of_types(&BIO).len();
            let amount = bot
                .units
                .my
                .structures
                .of_type(UnitTypeId::EngineeringBay)
                .len();

            if bio_amount < 6 || amount > 1 || bot.already_pending(UnitTypeId::EngineeringBay) > 0 {
                return false;
            }
            //first
            if amount == 0 && bot.bases.len() > 1 {
                return true;
            }
            //second
            if amount == 1 && bot.bases.len() > 2 && bio_amount > 20 {
                return true;
            }
            false
        }
        UnitTypeId::Armory => {
            let amount = bot.units.my.structures.of_type(UnitTypeId::Armory).len();
            let mech_amount = bot.units.my.units.of_types(&MECH).len();
            let hellion_amount = bot.units.my.units.of_type(UnitTypeId::Hellion).len();

            if bot.already_pending(UnitTypeId::Armory) > 0 || amount > 1 {
                return false;
            }

            //hellion fast
            if amount == 0 && hellion_amount > 2 {
                return true;
            }

            //first
            if amount == 0 && bot.bases.len() > 1 && mech_amount > 5 {
                return true;
            }

            //second
            if amount == 1 && bot.bases.len() > 2 && mech_amount > 15 {
                return true;
            }
            false
        }
        UnitTypeId::Bunker => false,
        UnitTypeId::MissileTurret => {
            if bot.already_pending(UnitTypeId::MissileTurret) > 0
                || bot
                    .units
                    .my
                    .structures
                    .of_type(UnitTypeId::EngineeringBay)
                    .ready()
                    .is_empty()
            {
                return false;
            }
            //not needed
            if !(bot.enemy_cloaking || bot.enemy_fliers) {
                return false;
            }

            //turret missing
            for base in bot.units.my.structures.find_tags(&bot.bases) {
                if bot
                    .units
                    .my
                    .structures
                    .of_type(UnitTypeId::MissileTurret)
                    .closer(12.0, base.position())
                    .is_empty()
                {
                    return true;
                }
            }
            false
        }
        UnitTypeId::CommandCenter => {
            let ccs = bot.units.my.townhalls.clone();
            //TODO: saturation check change
            let saturated = bot.units.my.workers.len() > (bot.units.my.townhalls.len() * 22) || bot.units.my.workers.len() > 50;

            if let Some(expansion) = bot.get_expansion() {
                if saturated
                    && !bot
                        .units
                        .my
                        .structures
                        .of_type_including_alias(UnitTypeId::Factory)
                        .is_empty()
                    && !bot
                        .units
                        .my
                        .structures
                        .of_type_including_alias(UnitTypeId::Starport)
                        .is_empty()
                    && bot.already_pending(UnitTypeId::CommandCenter) == 0
                    && ccs.closer(8.0, expansion.loc).is_empty()
                    && bot.units.enemy.units.closer(15.0, expansion.loc).is_empty()
                    && bot.defensive_point.is_none()
                {
                    return true;
                }
            }
            false
        }
        _ => false,
    }
}
