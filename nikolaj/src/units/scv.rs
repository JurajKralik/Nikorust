use crate::Nikolaj;
use rust_sc2::prelude::*;
use crate::helpers::construction::*;


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
pub fn distribute_workers(bot: &mut Nikolaj) {
	const DISTRIBUTION_DELAY: u32 = 8;

	if bot.units.my.workers.is_empty() {
		return;
	}

	let mut idle_workers = bot.units.my.workers.idle();

	// Skip if distribution delay not yet passed and no idle workers
	let game_loop = bot.state.observation.game_loop();
	if idle_workers.is_empty() && bot.last_loop_distributed + DISTRIBUTION_DELAY > game_loop {
		return;
	}
	bot.last_loop_distributed = game_loop;

	let mineral_fields = &bot.units.mineral_fields;
	let bases = bot.units.my.townhalls.ready();
	if mineral_fields.is_empty() || bases.is_empty() {
		return;
	}

	let mut deficit_mineral_bases = Units::new();
	let mut deficit_geysers = Units::new();

	distribute_mineral_deficits(bot, &bases, mineral_fields, &mut idle_workers, &mut deficit_mineral_bases);
	distribute_gas_deficits(bot, &bases, &mut idle_workers, &mut deficit_geysers);

	let surplus_workers = idle_workers.len() > deficit_mineral_bases.len() + deficit_geysers.len();
	let fallback_minerals = if surplus_workers {
		let nearby = mineral_fields.filter(|m| bases.iter().any(|b| b.is_closer(11.0, *m)));
		if nearby.is_empty() { None } else { Some(nearby) }
	} else {
		None
	};

	assign_idle_workers(idle_workers, &mut deficit_geysers, &mut deficit_mineral_bases, mineral_fields, fallback_minerals);
}

fn distribute_mineral_deficits(
	bot: &Nikolaj,
	bases: &Units,
	mineral_fields: &Units,
	idle_workers: &mut Units,
	deficit_list: &mut Units,
) {
	for base in bases {
		let current = base.assigned_harvesters().unwrap_or(0);
		let ideal = base.ideal_harvesters().unwrap_or(0);
		match current.cmp(&ideal) {
			std::cmp::Ordering::Less => {
				for _ in 0..(ideal - current) {
					deficit_list.push(base.clone());
				}
			}
			std::cmp::Ordering::Greater => {
				let local_tags: Vec<u64> = mineral_fields
					.iter()
					.closer(11.0, base)
					.map(|m| m.tag())
					.collect();
				let surplus = current - ideal;
				let reclaimed: Vec<_> = bot
					.units
					.my
					.workers
					.filter(|u| {
						u.target_tag().map_or(false, |t| {
							local_tags.contains(&t)
								|| (u.is_carrying_minerals() && t == base.tag())
						})
					})
					.iter()
					.take(surplus as usize)
					.cloned()
					.collect();
				for worker in reclaimed {
                    idle_workers.push(worker);
                }
			}
			_ => {}
		}
	}
}

fn distribute_gas_deficits(
	bot: &Nikolaj,
	bases: &Units,
	idle_workers: &mut Units,
	deficit_list: &mut Units,
) {
	for gas in bot.units.my.gas_buildings.iter().ready().filter(|g| g.vespene_contents().unwrap_or(0) > 0) {
		let current = gas.assigned_harvesters().unwrap_or(0);
		let ideal = gas.ideal_harvesters().unwrap_or(0);
		match current.cmp(&ideal) {
			std::cmp::Ordering::Less => {
				for _ in 0..(ideal - current) {
					deficit_list.push(gas.clone());
				}
			}
			std::cmp::Ordering::Greater => {
				let surplus = current - ideal;
				let reclaimed: Vec<_> = bot
					.units
					.my
					.workers
					.filter(|u| {
						u.target_tag().map_or(false, |t| {
							t == gas.tag()
								|| (u.is_carrying_vespene()
									&& t == bases.closest(gas).map(|b| b.tag()).unwrap_or(0))
						})
					})
					.iter()
					.take(surplus as usize)
					.cloned()
					.collect();
				for worker in reclaimed {
                    idle_workers.push(worker);
                }
			}
			_ => {}
		}
	}
}

fn assign_idle_workers(
	idle_workers: Units,
	deficit_geysers: &mut Units,
	deficit_minerals: &mut Units,
	mineral_fields: &Units,
	fallback_minerals: Option<Units>,
) {
	for worker in &idle_workers {
		if let Some(closest_gas) = deficit_geysers.closest(worker) {
			let tag = closest_gas.tag();
			deficit_geysers.remove(tag);
			worker.gather(tag, false);
		} else if let Some(closest_base) = deficit_minerals.closest(worker) {
			if let Some(best_mineral) = mineral_fields
				.closer(11.0, closest_base)
				.iter()
				.max_by_key(|m| m.mineral_contents().unwrap_or(0))
			{
				worker.gather(best_mineral.tag(), false);
			}
			deficit_minerals.remove(closest_base.tag());
		} else if worker.is_idle() {
			if let Some(fallback) = fallback_minerals.as_ref().and_then(|ms| ms.closest(worker)) {
				worker.gather(fallback.tag(), false);
			}
		}
	}
}
