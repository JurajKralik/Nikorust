use crate::Nikolaj;
use rust_sc2::prelude::*;
use std::cmp::Ordering;


pub fn distribute_workers(bot: &mut Nikolaj) {
    const DISTRIBUTION_DELAY: u32 = 8;

    if bot.units.my.workers.is_empty() {
        return;
    }
    let mut idle_workers = bot.units.my.workers.idle();

    // Check distribution delay if there aren't any idle workers
    let game_loop = bot.state.observation.game_loop();
    let last_loop = &mut bot.last_loop_distributed;
    if idle_workers.is_empty() && *last_loop + DISTRIBUTION_DELAY > game_loop {
        return;
    }
    *last_loop = game_loop;

    // Distribute
    let mineral_fields = &bot.units.mineral_fields;
    if mineral_fields.is_empty() {
        return;
    }
    let bases = bot.units.my.townhalls.ready();
    if bases.is_empty() {
        return;
    }

    let mut deficit_minings = Units::new();
    let mut deficit_geysers = Units::new();

    // Distributing mineral workers
    for base in &bases {
        match base.assigned_harvesters().cmp(&base.ideal_harvesters()) {
            Ordering::Less => (0..(base.ideal_harvesters().unwrap()
                - base.assigned_harvesters().unwrap()))
                .for_each(|_| {
                    deficit_minings.push(base.clone());
                }),
            Ordering::Greater => {
                let local_minerals = mineral_fields
                    .iter()
                    .closer(11.0, base)
                    .map(|m| m.tag())
                    .collect::<Vec<u64>>();

                idle_workers.extend(
                    bot.units
                        .my
                        .workers
                        .filter(|u| {
                            u.target_tag().map_or(false, |target_tag| {
                                local_minerals.contains(&target_tag)
                                    || (u.is_carrying_minerals() && target_tag == base.tag())
                            })
                        })
                        .iter()
                        .take(
                            (base.assigned_harvesters().unwrap() - base.ideal_harvesters().unwrap())
                                as usize,
                        )
                        .cloned(),
                );
            }
            _ => {}
        }
    }

    // Distributing gas workers
    bot.units
        .my
        .gas_buildings
        .iter()
        .ready()
        .filter(|g| g.vespene_contents().map_or(false, |vespene| vespene > 0))
        .for_each(
            |gas| match gas.assigned_harvesters().cmp(&gas.ideal_harvesters()) {
                Ordering::Less => (0..(gas.ideal_harvesters().unwrap()
                    - gas.assigned_harvesters().unwrap()))
                    .for_each(|_| {
                        deficit_geysers.push(gas.clone());
                    }),
                Ordering::Greater => {
                    idle_workers.extend(
                        bot.units
                            .my
                            .workers
                            .filter(|u| {
                                u.target_tag().map_or(false, |target_tag| {
                                    target_tag == gas.tag()
                                        || (u.is_carrying_vespene()
                                            && target_tag == bases.closest(gas).unwrap().tag())
                                })
                            })
                            .iter()
                            .take(
                                (gas.assigned_harvesters().unwrap() - gas.ideal_harvesters().unwrap())
                                    as usize,
                            )
                            .cloned(),
                    );
                }
                _ => {}
            },
        );

    // Distributing idle workers
    let minerals_near_base = if idle_workers.len() > deficit_minings.len() + deficit_geysers.len() {
        let minerals = mineral_fields.filter(|m| bases.iter().any(|base| base.is_closer(11.0, *m)));
        if minerals.is_empty() {
            None
        } else {
            Some(minerals)
        }
    } else {
        None
    };

    for u in &idle_workers {
        if let Some(closest) = deficit_geysers.closest(u) {
            let tag = closest.tag();
            deficit_geysers.remove(tag);
            u.gather(tag, false);
        } else if let Some(closest) = deficit_minings.closest(u) {
            u.gather(
                mineral_fields
                    .closer(11.0, closest)
                    .max(|m| m.mineral_contents().unwrap_or(0))
                    .unwrap()
                    .tag(),
                false,
            );
            let tag = closest.tag();
            deficit_minings.remove(tag);
        } else if u.is_idle() {
            if let Some(mineral) = minerals_near_base.as_ref().and_then(|ms| ms.closest(u)) {
                u.gather(mineral.tag(), false);
            }
        }
    }
}