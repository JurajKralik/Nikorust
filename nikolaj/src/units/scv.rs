use crate::Nikolaj;
use rust_sc2::prelude::*;
use crate::helpers::construction::*;
use std::collections::HashMap;


pub struct ScvControl {
	mining_distribution: HashMap<u64, Vec<u64>>,
	repair_list: HashMap<u64, Vec<u64>>,
	current_refineries: Units,
	current_mineral_fields: Units,
	current_gas_workers: Units,
	current_mineral_workers: Units,
	current_idle_workers: Units
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
			current_idle_workers: Units::default()
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

fn init_mineral_fields_and_refineries(bot: &mut Nikolaj) {
	// Init valid tags
	let mut valid_tags = Vec::new();
	for base in bot.units.my.townhalls.ready().clone() {
		for mineral in bot.units.mineral_fields.closer(10.0, base.position()) {
			valid_tags.push(mineral.tag());
		}
		for refinery in bot.units.my.structures.of_type_including_alias(UnitTypeId::Refinery).closer(10.0, base.position()).ready() {
			if refinery.vespene_contents().unwrap_or(0) > 0 {
				valid_tags.push(refinery.tag());
			}
		}
	}

	// Remove entries not in valid set
	let tags_to_remove: Vec<u64> = bot.scvs.mining_distribution.keys()
		.filter(|tag| !valid_tags.contains(tag))
		.cloned()
		.collect();
	for tag in tags_to_remove {
		println!("[{}]: Removing invalid mining_distribution tag: {}", bot.iteration, tag);
		bot.scvs.mining_distribution.remove(&tag);
	}

	// Add missing valid tags
	for tag in &valid_tags {
		if !bot.scvs.mining_distribution.contains_key(tag) {
			println!("[{}]: Adding new mining_distribution tag: {}", bot.iteration, tag);
			bot.scvs.mining_distribution.insert(*tag, Vec::new());
		}
	}
}

fn init_repair_targets(bot: &mut Nikolaj) {
	// Delete repaired or destroyed structures
	let repair_keys: Vec<u64> = bot.scvs.repair_list.keys().cloned().collect();

	for structure_tag in repair_keys {
		if let Some(structure) = bot.units.my.structures.iter().find_tag(structure_tag.clone()) {
			if structure.health_percentage().unwrap_or(0.0) >= 1.0 {
				bot.scvs.repair_list.remove(&structure_tag);
			}
		}
	}

	// Newly damaged
	for structure in bot.units.my.structures.ready().clone() {
		if structure.health_percentage().unwrap_or(0.0) < 0.8 && !bot.scvs.repair_list.contains_key(&structure.tag()) {
			bot.scvs.repair_list.insert(structure.tag(), Vec::new());
		}
	}
}

fn get_worker_allocations(bot: &mut Nikolaj) {
	// Refresh resources
	for resource_tag in bot.scvs.mining_distribution.keys() {
		if let Some(mineral_field) = bot.units.mineral_fields.iter().find_tag(resource_tag.clone()) {
			bot.scvs.current_mineral_fields.push(mineral_field.clone());
		} else if let Some(refinery) = bot.units.my.structures.iter().find_tag(resource_tag.clone()) {
			bot.scvs.current_refineries.push(refinery.clone());
		} else {
			println!("[{}]: (1) Unknown resource tag: {}", bot.iteration, resource_tag);
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
		if !(bot.scvs.current_gas_workers.contains_tag(worker.tag()) || bot.scvs.current_mineral_workers.contains_tag(worker.tag())) {
			bot.scvs.current_idle_workers.push(worker);
		}
	}
}

fn split_workers(bot: &mut Nikolaj) {
	let saturation = get_saturation_of_resources(bot);

	while !saturation.deficit_at_mineral_fields.is_empty() || !saturation.deficit_at_refineries.is_empty() && !saturation.overflow_at_mineral_fields.is_empty() || !saturation.overflow_at_refineries.is_empty() {
	  



	}

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
		let assigned = bot.scvs.mining_distribution.get(&tag).map_or(0, |v| v.len() as u32);
		if assigned < 2 {
			saturation.deficit_at_mineral_fields.insert(tag, 2 - assigned);
		}
	}

	for refinery in bot.scvs.current_refineries.iter() {
		let tag = refinery.tag();
		let assigned = bot.scvs.mining_distribution.get(&tag).map_or(0, |v| v.len() as u32);
		if assigned < 3 {
			saturation.deficit_at_refineries.insert(tag, 3 - assigned);
		}
	}

	for mineral_field in bot.scvs.current_mineral_fields.iter() {
		let tag = mineral_field.tag();
		let assigned = bot.scvs.mining_distribution.get(&tag).map_or(0, |v| v.len() as u32);
		if assigned > 2 {
			saturation.overflow_at_mineral_fields.insert(tag, assigned - 2);
		}
	}

	for refinery in bot.scvs.current_refineries.iter() {
		let tag = refinery.tag();
		let assigned = bot.scvs.mining_distribution.get(&tag).map_or(0, |v| v.len() as u32);
		if assigned > 3 {
			saturation.overflow_at_refineries.insert(tag, assigned - 3);
		}
	}

	return saturation;
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