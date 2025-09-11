use crate:: Nikolaj;
use std::collections::HashSet;
use rust_sc2::prelude::*;

const START_REPAIR_HP_PERCENTAGE: f32 = 0.80;
const STOP_REPAIR_HP_PERCENTAGE: f32 = 1.0;

const MAX_DEPOT_REPAIRERS: usize = 2;
const MAX_CC_REPAIRERS:    usize = 5;
const MAX_BUNKER_REPAIRERS:usize = 3;
const MAX_OTHER_REPAIRERS: usize = 1;

const MECH_TYPES: [UnitTypeId; 4] = [
    UnitTypeId::SiegeTank,
    UnitTypeId::SiegeTankSieged,
    UnitTypeId::Banshee,
    UnitTypeId::Thor,
];

pub fn repair(bot: &mut Nikolaj) {
    delete_dead_or_repaired_targets(bot);
    delete_dead_workers(bot);
    discover_new_targets(bot);
    assign_workers(bot); // TODO
}

fn delete_dead_or_repaired_targets(bot: &mut Nikolaj) {
    let structures = bot.units.my.structures.clone();
    let units = bot.units.my.units.clone();

    bot.scvs.repair_list.retain(|tag, _| {
        structures.contains_tag(tag.clone()) || units.contains_tag(tag.clone())
    });
}

fn delete_dead_workers(bot: &mut Nikolaj) {
    let alive_workers: HashSet<_> = bot.units.my.workers.tags().copied().collect();
    for (_target, workers_for_target) in bot.scvs.repair_list.iter_mut() {
        workers_for_target.retain(|w| alive_workers.contains(w));
    }
    bot.scvs.repair_list.retain(|_, workers_for_target| !workers_for_target.is_empty());
}

fn discover_new_targets(bot: &mut Nikolaj) {
    for structure in bot.units.my.structures.ready().clone() {
        if structure.health_percentage().unwrap_or(1.0) < START_REPAIR_HP_PERCENTAGE {
            bot.scvs.repair_list.entry(structure.tag()).or_insert_with(Vec::new);
        }
    }
    for unit in bot.units.my.units.ready().of_types(&&MECH_TYPES[..]).clone() {
        if unit.health_percentage().unwrap_or(1.0) < START_REPAIR_HP_PERCENTAGE {
            bot.scvs.repair_list.entry(unit.tag()).or_insert_with(Vec::new);
        }
    }
}

fn assign_workers(bot: &mut Nikolaj) {
    for (target, workers) in bot.scvs.repair_list.iter_mut() {

    }
}

fn cap_for(t: UnitTypeId) -> usize {
    match t {
        UnitTypeId::SupplyDepot | UnitTypeId::SupplyDepotLowered => MAX_DEPOT_REPAIRERS,
        UnitTypeId::CommandCenter | UnitTypeId::OrbitalCommand | UnitTypeId::PlanetaryFortress => MAX_CC_REPAIRERS,
        UnitTypeId::Bunker => MAX_BUNKER_REPAIRERS,
        _ => MAX_OTHER_REPAIRERS,
    }
}
