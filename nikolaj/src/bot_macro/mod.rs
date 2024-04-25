use crate::Nikolaj;
use rust_sc2::prelude::*;

pub(crate) fn get_macro_conditions(bot: &mut Nikolaj, structure: &UnitTypeId) -> bool {
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
        UnitTypeId::EngineeringBay => false,
        _ => false,
    }
}
