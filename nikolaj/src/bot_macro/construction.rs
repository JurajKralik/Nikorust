use crate::Nikolaj;
use rust_sc2::prelude::*;

pub(crate) fn finish_building_without_workers(bot: &mut Nikolaj) {
    for building in bot.units.my.structures.not_ready().clone() {
        let mut has_builder = false;
        for worker in bot.units.my.workers.clone() {
            if worker.is_constructing_any(&vec![building.type_id()]) {
                has_builder = true;
                break;
            }
        }
        if !has_builder {
            if let Some(worker) = bot.get_builder(building.position()) {
                worker.smart(Target::Tag(building.tag()), false);
            }
        }
    }
}

pub(crate) fn get_structure_position(bot: &mut Nikolaj, structure: UnitTypeId) -> Option<Point2> {
    let mut pos: Option<Point2> = None;

    match structure {
        UnitTypeId::SupplyDepot => {
            if let Some(depots) = bot.ramps.my.corner_depots() {
                for depot in depots {
                    if bot.units.my.structures.closer(2.0, depot).is_empty() {
                        pos = Some(depot);
                        return pos;
                    }
                }
            }
            if let Some(depot) = bot.ramps.my.depot_in_middle() {
                if bot.units.my.structures.closer(2.0, depot).is_empty() {
                    pos = Some(depot);
                    return pos;
                }
            }
            None
        }
        _ => None,
    }
}

pub(crate) fn construct(bot: &mut Nikolaj, structure: UnitTypeId) {
    let pos = get_structure_position(bot, structure);
    if let Some(pos) = pos {
        if let Some(worker) = bot.get_builder(pos) {
            worker.build(structure, pos, false);
        }
    }
}
