use crate::Nikolaj;
use rust_sc2::prelude::*;
use crate::consts::*;


pub fn read_enemy_strategy(bot: &mut Nikolaj) {
    // Worker rush and ramp blocking
    if bot.time < 60.0 * 3.0 {
        detect_enemy_worker_rush(bot);
        if bot.time - bot.strategy_data.enemy_worker_rush_time > 5.0 {
            bot.strategy_data.enemy_worker_rush = false;
        }
        detect_enemy_ramp_blocking(bot);
    }

    detect_enemy_flooding(bot);
    detect_cloaking_enemy(bot);
    detect_flying_enemy(bot);
}
fn detect_enemy_worker_rush(bot: &mut Nikolaj) {
    let enemy_workers = bot.units.enemy.workers.clone();
    let mut offensive_workers = 0;
    for worker in enemy_workers {
        let distance_to_my_base = worker.position().distance(bot.start_location);
        let distance_to_enemy_base = worker.position().distance(bot.enemy_start);
        if distance_to_my_base < distance_to_enemy_base {
            offensive_workers += 1;
        }
    }
    if offensive_workers >= 4 {
        bot.strategy_data.enemy_worker_rush_time = bot.time;
        if !bot.strategy_data.enemy_worker_rush {
            println!(
                "Strategy: Detected enemy worker rush at time {:.1}",
                bot.time
            );
        }
        bot.strategy_data.enemy_worker_rush = true;
    }
}
fn detect_enemy_ramp_blocking(bot: &mut Nikolaj) {
    let enemy_workers = bot.units.enemy.workers.clone();
    let block_positions = bot.ramps.my.corner_depots();
    let ramp_center = bot.ramps.my.barracks_in_middle();
    if bot.strategy_data.enemy_ramp_blocking {
        if enemy_workers
            .closer(10.0, ramp_center.unwrap_or(Point2 { x: 0.0, y: 0.0 }))
            .is_empty()
        {
            bot.strategy_data.enemy_ramp_blocking = false;
            bot.strategy_data.enemy_ramp_blocking_steps = 0;
            println!(
                "Strategy: Enemy ramp blocking ended at time {:.1}",
                bot.time
            );
        }
    }
    let mut blocking = false;
    if let Some(positions) = block_positions {
        for position in positions {
            if !enemy_workers.closer(1.0, position).is_empty() {
                blocking = true;
                continue;
            }
        }
    }
    if blocking {
        bot.strategy_data.enemy_ramp_blocking_steps += 1;
        bot.strategy_data.enemy_ramp_blocking_time = bot.time;
    }
    if bot.time - bot.strategy_data.enemy_ramp_blocking_time > 3.0 {
        bot.strategy_data.enemy_ramp_blocking_steps = 0;
    }
    if bot.strategy_data.enemy_ramp_blocking_steps > 8 && !bot.strategy_data.enemy_ramp_blocking {
        bot.strategy_data.enemy_ramp_blocking = true;
        println!(
            "Strategy: Detected enemy ramp blocking at time {:.1}",
            bot.time
        );
    }
}
fn detect_enemy_flooding(bot: &mut Nikolaj) {
    if bot.time > 60.0 * 5.0 {
        bot.strategy_data.enemy_flooding = false;
        return;
    }

    let enemy_units = bot.units.enemy.units.clone();
    let mut offensive_units = 0;
    for enemy in enemy_units {
        if !enemy.can_attack() || enemy.type_id().is_worker() {
            continue;
        }
        let distance_to_my_base = enemy.position().distance(bot.start_location);
        let distance_to_enemy_base = enemy.position().distance(bot.enemy_start);
        if distance_to_my_base < distance_to_enemy_base + 10.0 {
            offensive_units += 1;
        }
    }
    if offensive_units >= 5 && !bot.strategy_data.enemy_flooding {
        println!("Strategy: Detected enemy flooding at time {:.1}", bot.time);
        bot.strategy_data.enemy_flooding = true;
    }
}
fn detect_cloaking_enemy(bot: &mut Nikolaj) {
    if bot.strategy_data.enemy_cloaking {
        return;
    }

    let enemy_units = bot.strategy_data.enemy_army.clone();
    let cloaking_units = [
        UnitTypeId::Banshee,
        UnitTypeId::Ghost,
        UnitTypeId::WidowMine,
        UnitTypeId::WidowMineBurrowed,
        UnitTypeId::Roach,
        UnitTypeId::RoachBurrowed,
        UnitTypeId::DarkTemplar,
        UnitTypeId::Mothership,
        UnitTypeId::LurkerMP,
        UnitTypeId::LurkerMPBurrowed,
        UnitTypeId::LurkerMPEgg,
    ];
    for enemy in enemy_units.units {
        let enemy_type = enemy.type_id;
        if cloaking_units.contains(&enemy_type) {
            if !bot.strategy_data.enemy_cloaking {
                println!(
                    "Strategy: Detected enemy cloaking unit {:?} at time {:.1}",
                    enemy_type,
                    bot.time
                );
            }
            bot.strategy_data.enemy_cloaking = true;
            return;
        }
    }
    let enemy_structures = bot.units.enemy.structures.clone();
    let cloaking_structures = [
        UnitTypeId::DarkShrine,
        UnitTypeId::LurkerDenMP,
        UnitTypeId::GhostAcademy,
        UnitTypeId::StarportTechLab,
    ];
    for structure in enemy_structures {
        let structure_type = structure.type_id();
        if cloaking_structures.contains(&structure_type) {
            if !bot.strategy_data.enemy_cloaking {
                println!(
                    "Strategy: Detected enemy cloaking structure {:?} at time {:.1}",
                    structure_type,
                    bot.time
                );
            }
            bot.strategy_data.enemy_cloaking = true;
            return;
        }
    }
}

fn detect_flying_enemy(bot: &mut Nikolaj) {
    if bot.strategy_data.enemy_flying_units {
        return;
    }

    let enemy_units = bot.strategy_data.enemy_army.units.clone();
    for enemy in enemy_units {
        let enemy_type = enemy.type_id;
        if FLYING_UNITS.contains(&enemy_type) {
            if !bot.strategy_data.enemy_flying_units {
                println!(
                    "Strategy: Detected enemy flying unit {:?} at time {:.1}",
                    enemy_type,
                    bot.time
                );
            }
            bot.strategy_data.enemy_flying_units = true;
            return;
        }
    }
}

