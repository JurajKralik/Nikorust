use crate::helpers::construction::*;
use crate::Nikolaj;
use rust_sc2::prelude::*;

pub fn construct_refinery(bot: &mut Nikolaj) {
    if !bot.can_afford(UnitTypeId::Refinery) {
        return;
    }

    if bot.already_pending(UnitTypeId::Refinery) > 0 {
        return;
    }

    if bot.units.my.structures.of_type_including_alias(UnitTypeId::Barracks).is_empty() {
        return;
    }
    
}