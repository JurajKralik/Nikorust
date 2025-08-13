use crate::helpers::construction::*;
use crate::Nikolaj;
use rust_sc2::prelude::*;
use std::collections::{HashMap, HashSet};

















pub struct ScvControl {
    mining_distribution: HashMap<u64, Vec<u64>>,
    repair_list: HashMap<u64, Vec<u64>>,
    current_refineries: Units,
    current_mineral_fields: Units,
    current_gas_workers: Units,
    current_mineral_workers: Units,
    current_idle_workers: Units,
}

impl Default for ScvControl {
    fn default() -> Self {
        Self {
            mining_distribution: HashMap::new(),
            repair_list: HashMap::new(),
            current_refineries: Units::default(),
            current_mineral_fields: Units::default(),
            current_gas_workers: Units::default(),
            current_mineral_workers: Units::default(),
            current_idle_workers: Units::default(),
        }
    }
}

struct SaturationInfo {
    deficit_at_mineral_fields: HashMap<u64, u32>,
    deficit_at_refineries: HashMap<u64, u32>,
    overflow_at_mineral_fields: HashMap<u64, u32>,
    overflow_at_refineries: HashMap<u64, u32>,
}

pub fn distribute_workers(bot: &mut Nikolaj) {
    reset_scv_control(bot);
    init_mineral_fields_and_refineries(bot);
    collect_valid_resource_tags(bot);
    init_repair_targets(bot);
    get_worker_allocations(bot);
    split_workers(bot);
}

fn reset_scv_control(bot: &mut Nikolaj) {
    bot.scvs.current_refineries.clear();
    bot.scvs.current_mineral_fields.clear();
    bot.scvs.current_gas_workers.clear();
    bot.scvs.current_mineral_workers.clear();
    bot.scvs.current_idle_workers.clear();
}

const GATHER_RADIUS: f32 = 10.0;

fn init_mineral_fields_and_refineries(bot: &mut Nikolaj) {
    let valid = collect_valid_resource_tags(bot);

    // Remove entries not in valid set (log what we remove)
    bot.scvs.mining_distribution.retain(|tag, _| {
        let keep = valid.contains(tag);
        if !keep {
            println!(
                "[{}]: Removing invalid mining_distribution tag: {}",
                bot.iteration, tag
            );
        }
        keep
    });

    // Add missing valid tags (log only new inserts)
    use std::collections::hash_map::Entry;
    for &tag in &valid {
        match bot.scvs.mining_distribution.entry(tag) {
            Entry::Occupied(_) => {}
            Entry::Vacant(v) => {
                println!(
                    "[{}]: Adding new mining_distribution tag: {}",
                    bot.iteration, tag
                );
                v.insert(Vec::new());
            }
        }
    }
}

fn collect_valid_resource_tags(bot: &Nikolaj) -> HashSet<u64> {
    let mut set = HashSet::new();

    for base in bot.units.my.townhalls.ready().clone() {
        // Minerals near the base
        for mineral in bot.units.mineral_fields.closer(GATHER_RADIUS, base.position()) {
            set.insert(mineral.tag());
        }

        // Refineries with gas remaining near the base
        for refinery in bot.units.my.structures
            .of_type_including_alias(UnitTypeId::Refinery)
            .closer(GATHER_RADIUS, base.position())
            .ready()
        {
            if refinery.vespene_contents().unwrap_or(0) > 0 {
                set.insert(refinery.tag());
            }
        }
    }

    set
}

fn init_repair_targets(bot: &mut Nikolaj) {
    // Delete repaired or destroyed structures
    let repair_keys: Vec<u64> = bot.scvs.repair_list.keys().cloned().collect();

    for structure_tag in repair_keys {
        if let Some(structure) = bot
            .units
            .my
            .structures
            .iter()
            .find_tag(structure_tag.clone())
        {
            if structure.health_percentage().unwrap_or(0.0) >= 1.0 {
                bot.scvs.repair_list.remove(&structure_tag);
            }
        }
    }

    // Newly damaged
    for structure in bot.units.my.structures.ready().clone() {
        if structure.health_percentage().unwrap_or(0.0) < 0.8
            && !bot.scvs.repair_list.contains_key(&structure.tag())
        {
            bot.scvs.repair_list.insert(structure.tag(), Vec::new());
        }
    }
}

fn get_worker_allocations(bot: &mut Nikolaj) {
    // Refresh resources
    for resource_tag in bot.scvs.mining_distribution.keys() {
        if let Some(mineral_field) = bot
            .units
            .mineral_fields
            .iter()
            .find_tag(resource_tag.clone())
        {
            bot.scvs.current_mineral_fields.push(mineral_field.clone());
        } else if let Some(refinery) = bot
            .units
            .my
            .structures
            .iter()
            .find_tag(resource_tag.clone())
        {
            bot.scvs.current_refineries.push(refinery.clone());
        } else {
            println!(
                "[{}]: (1) Unknown resource tag: {}",
                bot.iteration, resource_tag
            );
        }
    }

    // Get current worker category (gas/mineral)
    for (tag, workers) in &bot.scvs.mining_distribution {
        if workers.is_empty() {
            continue; // Skip empty entries
        }
        // Mineral fields
        if bot.units.mineral_fields.contains_tag(*tag) {
            for worker_tag in workers {
                if let Some(worker_unit) = bot.units.my.workers.iter().find_tag(*worker_tag) {
                    bot.scvs.current_mineral_workers.push(worker_unit.clone());
                }
            }
        // Refineries
        } else if bot.units.my.structures.contains_tag(*tag) {
            for worker_tag in workers {
                if let Some(worker_unit) = bot.units.my.workers.iter().find_tag(*worker_tag) {
                    bot.scvs.current_gas_workers.push(worker_unit.clone());
                }
            }
        } else {
            println!("[{}]: (2) Unknown resource tag: {}", bot.iteration, tag);
        }
    }

    // Idle workers
	for worker in bot.units.my.workers.clone() {
		if !(bot.scvs.current_gas_workers.contains_tag(worker.tag())
			|| bot.scvs.current_mineral_workers.contains_tag(worker.tag()))
		{
			bot.scvs.current_idle_workers.push(worker);
		}
	}
}

fn split_workers(bot: &mut Nikolaj) {
    let saturation = get_saturation_of_resources(bot);
    let imbalanced_saturation = !saturation.deficit_at_mineral_fields.is_empty()
        || !saturation.deficit_at_refineries.is_empty()
            && !saturation.overflow_at_mineral_fields.is_empty()
        || !saturation.overflow_at_refineries.is_empty();
	
	if imbalanced_saturation {
		make_overflow_workers_idle(&saturation, bot);
	}
	allocate_idle_workers(&saturation, bot);
}

fn allocate_idle_workers(saturation: &SaturationInfo, bot: &mut Nikolaj) {

}

fn get_saturation_of_resources(bot: &mut Nikolaj) -> SaturationInfo {
	let mut saturation = SaturationInfo {
		deficit_at_mineral_fields: HashMap::new(),
		deficit_at_refineries: HashMap::new(),
		overflow_at_mineral_fields: HashMap::new(),
		overflow_at_refineries: HashMap::new(),
    };

    for mineral_field in bot.scvs.current_mineral_fields.iter() {
        let tag = mineral_field.tag();
        let assigned = bot
            .scvs
            .mining_distribution
            .get(&tag)
            .map_or(0, |v| v.len() as u32);
        if assigned < 2 {
            saturation
                .deficit_at_mineral_fields
                .insert(tag, 2 - assigned);
        }
    }

    for refinery in bot.scvs.current_refineries.iter() {
        let tag = refinery.tag();
        let assigned = bot
            .scvs
            .mining_distribution
            .get(&tag)
            .map_or(0, |v| v.len() as u32);
        if assigned < 3 {
            saturation.deficit_at_refineries.insert(tag, 3 - assigned);
        }
    }

    for mineral_field in bot.scvs.current_mineral_fields.iter() {
        let tag = mineral_field.tag();
        let assigned = bot
            .scvs
            .mining_distribution
            .get(&tag)
            .map_or(0, |v| v.len() as u32);
        if assigned > 2 {
            saturation
                .overflow_at_mineral_fields
                .insert(tag, assigned - 2);
        }
    }

    for refinery in bot.scvs.current_refineries.iter() {
        let tag = refinery.tag();
        let assigned = bot
            .scvs
            .mining_distribution
            .get(&tag)
            .map_or(0, |v| v.len() as u32);
        if assigned > 3 {
            saturation.overflow_at_refineries.insert(tag, assigned - 3);
        }
    }

    return saturation;
}

fn make_overflow_workers_idle(saturation: &SaturationInfo, bot: &mut Nikolaj) {
    let mut workers_to_make_idle: Vec<u64> = Vec::new();
    // Handle overflow at mineral fields
    for (mineral_tag, overflow_count) in &saturation.overflow_at_mineral_fields {
        if let Some(workers) = bot.scvs.mining_distribution.get_mut(mineral_tag) {
            for _ in 0..*overflow_count {
                if let Some(worker_tag) = workers.pop() {
                    workers_to_make_idle.push(worker_tag);
                }
            }
        }
    }

    // Handle overflow at refineries
    for (refinery_tag, overflow_count) in &saturation.overflow_at_refineries {
        if let Some(workers) = bot.scvs.mining_distribution.get_mut(refinery_tag) {
            for _ in 0..*overflow_count {
                if let Some(worker_tag) = workers.pop() {
                    workers_to_make_idle.push(worker_tag);
                }
            }
        }
    }

    for worker_tag in workers_to_make_idle {
		if let Some(worker_unit) = bot.units.my.workers.iter().find_tag(worker_tag) {
			bot.scvs.current_idle_workers.push(worker_unit.clone());
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
