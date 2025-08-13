use crate::Nikolaj;
use std::collections::HashSet;
use rust_sc2::prelude::*;

const START_REPAIR_HP: f32 = 0.80;
const STOP_REPAIR_HP:  f32 = 1.0;

const MAX_DEPOT_REPAIRERS: usize = 1;
const MAX_CC_REPAIRERS:    usize = 2;
const MAX_BUNKER_REPAIRERS:usize = 2;
const MAX_OTHER_REPAIRERS: usize = 1;

const MECH_TYPES: [UnitTypeId; 4] = [
    UnitTypeId::SiegeTank,
    UnitTypeId::SiegeTankSieged,
    UnitTypeId::Banshee,
    UnitTypeId::Thor,
];

pub fn update_targets(bot: &mut Nikolaj) {
    cleanup_repair_list(bot);
    discover_new_targets(bot);
    maintain_assignments(bot);
}

fn cleanup_repair_list(bot: &mut Nikolaj) {
    // Snapshot existing worker tags
    let existing_workers: HashSet<u64> =
        bot.units.my.workers.iter().map(|u| u.tag()).collect();

    let keys: Vec<u64> = bot.scvs.repair_list.keys().cloned().collect();
    for tag in keys {
        match bot.units.my.structures.iter().find_tag(tag) {
            None => {
                bot.scvs.repair_list.remove(&tag);
            }
            Some(s) => {
                if s.health_percentage().unwrap_or(1.0) >= STOP_REPAIR_HP {
                    bot.scvs.repair_list.remove(&tag);
                } else if let Some(v) = bot.scvs.repair_list.get_mut(&tag) {
                    v.retain(|wtag| existing_workers.contains(wtag));
                }
            }
        }
    }
}


fn discover_new_targets(bot: &mut Nikolaj) {
    for s in bot.units.my.structures.ready().clone() {
        if s.health_percentage().unwrap_or(1.0) < START_REPAIR_HP {
            bot.scvs.repair_list.entry(s.tag()).or_insert_with(Vec::new);
        }
    }
    for u in bot.units.my.units.ready().of_types(&&MECH_TYPES[..]).clone() {
        if u.is_structure() && u.health_percentage().unwrap_or(1.0) < START_REPAIR_HP {
            bot.scvs.repair_list.entry(u.tag()).or_insert_with(Vec::new);
        }
    }
}

// 3) keep each target at its cap using idle -> mineral -> gas; issue .smart(Target::Tag(...))
fn maintain_assignments(bot: &mut Nikolaj) {
    // collect tags first to avoid borrow issues
    let targets: Vec<u64> = bot.scvs.repair_list.keys().cloned().collect();

    for tag in targets {
        let Some(structure) = bot.units.my.structures.iter().find_tag(tag) else {
            // just in case it vanished after the snapshot
            bot.scvs.repair_list.remove(&tag);
            continue;
        };

        let cap = cap_for(structure.type_id());

        // — trim extras in a short scope (drop &mut before later borrows) —
        {
            if let Some(v) = bot.scvs.repair_list.get_mut(&tag) {
                while v.len() > cap { v.pop(); }
            }
        }

        let assigned_now = bot.scvs.repair_list.get(&tag).map_or(0, |v| v.len());
        if assigned_now < cap {
            let need = cap - assigned_now;

            // claim idle → mineral → gas (nearest first each tier)
            let mut claimed: Vec<Unit> = Vec::new();
            claim_from(&mut claimed, need, structure.position(), &bot.scvs.current_idle_workers);
            if claimed.len() < need {
                claim_from(&mut claimed, need - claimed.len(), structure.position(), &bot.scvs.current_mineral_workers);
            }
            if claimed.len() < need {
                claim_from(&mut claimed, need - claimed.len(), structure.position(), &bot.scvs.current_gas_workers);
            }

            // assign & command
            for w in claimed {
                remove_worker_from_all_resources(bot, w.tag()); // stop counting as miner
                bot.scvs.repair_list.entry(tag).or_default().push(w.tag());

                // Use SMART; no return-to-depot step
                w.smart(Target::Tag(tag), false);
            }
        } else {
            // ensure currently assigned workers are actually repairing (avoid spam)
            ensure_orders_for_assigned(bot, tag);
        }
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

/// choose up to `count` nearest units from `pool` to `pos` (no pool mutation)
fn claim_from(claimed: &mut Vec<Unit>, count: usize, pos: Point2, pool: &Units) {
    if count == 0 || pool.is_empty() { return; }
    let mut list: Vec<Unit> = pool.clone().into_iter().collect();
    list.sort_by(|a, b| a.distance(pos).partial_cmp(&b.distance(pos)).unwrap());
    for u in list.into_iter() {
        if claimed.len() >= count { break; }
        if !claimed.iter().any(|c| c.tag() == u.tag()) {
            claimed.push(u);
        }
    }
}

/// avoid re-issuing if already repairing; when needed, use SMART
fn ensure_orders_for_assigned(bot: &Nikolaj, target_tag: u64) {
    if let Some(v) = bot.scvs.repair_list.get(&target_tag) {
        for wtag in v {
            if let Some(w) = bot.units.my.workers.iter().find_tag(*wtag) {
                // If not already using repair, give a SMART to the target
                if !w.is_using(AbilityId::EffectRepairSCV) {
                    w.smart(Target::Tag(target_tag), false);
                }
            }
        }
    }
}

/// remove worker from any resource vector so counts don’t double
fn remove_worker_from_all_resources(bot: &mut Nikolaj, wtag: u64) {
    for v in bot.scvs.mining_distribution.values_mut() {
        if let Some(i) = v.iter().position(|&t| t == wtag) {
            v.swap_remove(i);
            break;
        }
    }
}
