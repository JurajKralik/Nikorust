use crate::Nikolaj;
use rust_sc2::prelude::*;

pub fn get_builder(bot: &mut Nikolaj, target: Target) -> Option<&Unit> {
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
                .filter(|u| !(u.is_constructing() || u.is_returning() || u.is_carrying_resource()))
                .closest(position);
        }
        Target::Pos(pos) => {
            let position = pos;
            return bot
                .units
                .my
                .workers
                .iter()
                .filter(|u| !(u.is_constructing() || u.is_returning() || u.is_carrying_resource()))
                .closest(position);
        }
    }
}
pub fn finish_building_without_workers(bot: &mut Nikolaj) {
    for building in bot.units.my.structures.not_ready().clone() {
        if building.is_addon() {
            continue;
        }
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

pub fn get_structure_position(bot: &mut Nikolaj, structure: UnitTypeId) -> Target {
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
        UnitTypeId::CommandCenter => {
            if let Some(expansion) = bot.get_expansion() {
                return Target::Pos(expansion.loc);
            }
            Target::None
        }
        UnitTypeId::Barracks => get_production_position(bot),
        UnitTypeId::Factory => get_production_position(bot),
        UnitTypeId::Starport => get_production_position(bot),
        UnitTypeId::EngineeringBay => get_production_position(bot),
        UnitTypeId::Armory => get_production_position(bot),
        _ => Target::None,
    }
}

fn get_production_position(bot: &mut Nikolaj) -> Target {
    //ramp middle
    if let Some(ramp_middle) = bot.ramps.my.barracks_in_middle() {
        if bot.can_place(UnitTypeId::Barracks, ramp_middle) {
            return Target::Pos(ramp_middle);
        }
    }
    for base in bot.units.my.townhalls.clone() {
        let near =base.position().towards(bot.enemy_start, 7.0);
        let pos = bot.find_placement(UnitTypeId::Barracks, near, Default::default());
        if let Some(pos) = pos {
            if bot.can_place(UnitTypeId::Barracks, pos) {
                return Target::Pos(pos);
            }
        }
    }
    for base in bot.units.my.townhalls.clone() {
        let near =base.position().towards(bot.enemy_start, 7.0);
        let placementOptions = PlacementOptions {
            step: 7,
            ..Default::default()
        };
        let pos = bot.find_placement(UnitTypeId::Barracks, near, placementOptions);
        if let Some(pos) = pos {
            if bot.can_place(UnitTypeId::Barracks, pos) {
                return Target::Pos(pos);
            }
        }
    }
    Target::None
}

pub fn construct(bot: &mut Nikolaj, structure: UnitTypeId) {
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
