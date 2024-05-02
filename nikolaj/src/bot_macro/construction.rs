use crate::Nikolaj;
use rust_sc2::prelude::*;

pub(crate) fn finish_building_without_workers(bot: &mut Nikolaj) {
    for building in bot.units.my.structures.not_ready().clone() {
        //TODO: no function for structures_without_construction_SCVs
    }
}

