use crate::helpers::construction::*;
use crate::Nikolaj;
use rust_sc2::prelude::*;

pub fn construct_refinery(bot: &mut Nikolaj) {
    if !bot.can_afford(UnitTypeId::Refinery, false) {
        return;
    }

    for under_construction in bot.construction_info.under_construction.iter() {
        if under_construction.structure == UnitTypeId::Refinery {
            return;
        }
    }

    if bot.already_pending(UnitTypeId::Refinery) > 0 {
        return;
    }

    if bot.units.my.structures.of_type_including_alias(UnitTypeId::Barracks).is_empty() {
        return;
    }

    let mut geyser_tag = None;

    for base in bot.units.my.townhalls.ready() {
        if let Some(geyser) = bot.find_gas_placement(base.position()) {
            let vespene_contents = geyser.vespene_contents();
            if vespene_contents > 0 {
                geyser_tag = Some(geyser.tag());
                break;
            }
        }
    }

    if let Some(geyser_tag) = geyser_tag {
        if let Some(builder) = get_builder(bot, Target::Tag(geyser_tag)) {
            builder.build_gas(geyser_tag, false);
        }
    }
}