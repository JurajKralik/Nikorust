
use crate::Nikolaj;
use rust_sc2::prelude::*;
use crate::helpers::construction::*;


pub fn distribute_workers(bot: &mut Nikolaj) {
	init_mineral_fields_and_refineries(bot);
	println!("{:?}", bot.mining_distribution);
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
	bot.mining_distribution.retain(|tag, _| valid_tags.contains(tag));

	// Add missing valid tags
	for tag in &valid_tags {
		bot.mining_distribution.entry(*tag).or_insert_with(Vec::new);
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