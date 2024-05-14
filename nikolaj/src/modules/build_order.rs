use crate::Nikolaj;
use rust_sc2::prelude::*;

pub(crate) fn decide_build_order(bot: &mut Nikolaj) -> Vec<UnitTypeId>{
    const BIO: &'static [UnitTypeId] = &[
        UnitTypeId::Marine,
        UnitTypeId::Marauder,
        UnitTypeId::Reaper,
        UnitTypeId::Ghost,
    ];
    let mut build_order: Vec<UnitTypeId> = vec![];

    if bot.enemy_race == Race::Terran {
        build_order = vec![
            UnitTypeId::SiegeTank,
            UnitTypeId::VikingFighter,
            UnitTypeId::Marine,
        ];
    }

    //add support units
    //raven
    if bot.enemy_cloaking && bot.my_unit_count(UnitTypeId::Raven) == 0 {
        build_order.push(UnitTypeId::Raven);
    }
    //medivac
    let bio_amount = bot.units.my.units.of_types(&BIO).len();
    if bio_amount > 6 && bot.my_unit_count(UnitTypeId::Medivac) * 5 < bio_amount {
        build_order.push(UnitTypeId::Medivac);
    }

    //starters
    for unit in [UnitTypeId::Reaper, UnitTypeId::Banshee].iter() {
        if bot.already_pending(*unit) == 0 && bot.my_units_memory.of_type(*unit).is_empty() {
            build_order.insert(0, *unit);
        }
    }
    build_order
}

pub(crate) fn execute_build_order(bot: &mut Nikolaj) {
    let build_order = decide_build_order(bot);
    for unit in build_order {
    }
}
