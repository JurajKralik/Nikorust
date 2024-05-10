use crate::Nikolaj;
use rust_sc2::prelude::*;

fn get_builder(bot: &mut Nikolaj, target: Target) -> Option<&Unit> {
    match target {
        Target::None => {
            return None;
        }
        Target::Tag(tag) => {
            let position = bot.units.vespene_geysers.get(tag).unwrap().position();
            return bot
                .units
                .my
                .workers
                .iter()
                .filter(|u| {
                    !(u.is_constructing() || u.is_returning() || u.is_carrying_resource())
                })
                .closest(position);
        }
        Target::Pos(pos) => {
            let position = pos;
            return bot
                .units
                .my
                .workers
                .iter()
                .filter(|u| {
                    !(u.is_constructing() || u.is_returning() || u.is_carrying_resource())
                })
                .closest(position);
        }
    }
}
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
            if let Some(worker) = get_builder(bot, Target::Pos(building.position())) {
                worker.smart(Target::Tag(building.tag()), false);
            }
        }
    }
}

pub(crate) fn get_structure_position(bot: &mut Nikolaj, structure: UnitTypeId) -> Target {
    match structure {
        UnitTypeId::SupplyDepot => {
            if let Some(depots) = bot.ramps.my.corner_depots() {
                for depot in depots {
                    if bot.units.my.structures.closer(2.0, depot).is_empty() {
                        return Target::Pos(depot);
                    }
                }
            }
            if let Some(depot) = bot.ramps.my.depot_in_middle() {
                if bot.units.my.structures.closer(2.0, depot).is_empty() {
                    return Target::Pos(depot);
                }
            }
            for base in bot.units.my.structures.find_tags(&bot.bases.clone()) {
                let base_depots_pos = base.position().towards(bot.enemy_start, -10.0);
                if let Some(depot) =
                    bot.find_placement(UnitTypeId::SupplyDepot, base_depots_pos, Default::default())
                {
                    return Target::Pos(depot);
                }
            }
            Target::None
        }
        UnitTypeId::Refinery => {
            for base in bot.units.my.structures.find_tags(&bot.bases.clone()) {
                if let Some(geyser) = bot.find_gas_placement(base.position()) {
                    return Target::Tag(geyser.tag());
                }
            }
            Target::None
        }
        _ => Target::None,
    }
}

pub(crate) fn construct(bot: &mut Nikolaj, structure: UnitTypeId) {
    let pos = get_structure_position(bot, structure);
    let builder = get_builder(bot, pos);
    match pos {
        Target::None => {
            return;
        }
        Target::Tag(tag) => {
            if let Some(builder) = builder {
                builder.build_gas(tag, false);
            }
        }
        Target::Pos(pos) => {
            if let Some(builder) = builder {
                builder.build(structure, pos, false);
            }
        }
    }
}
