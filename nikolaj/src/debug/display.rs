use crate::Nikolaj;
use crate::units::helpers::workers_assignments::*;
use rust_sc2::prelude::*;
use std::collections::HashMap;
use crate::debug::types::*;

pub fn debug_show_bases(bot: &mut Nikolaj) {
    if !bot.debugger.displaying_bases {
        return;
    }
    let bases = bot.worker_allocator.bases.clone();
    for base_tag in bases {
        if let Some(base) = bot.units.my.structures.iter().find_tag(base_tag) {
            let position = base.position();
            let size = 2.5;
            let color = "yellow";
            bot.debug_cube(position, size, color);
        } else {
            println!("[DEBUGGER] {} Base with tag {} not found", bot.debugger.time, base_tag);
        }
    }
}

pub fn debug_show_mining(bot: &mut Nikolaj) {
    if !bot.debugger.displaying_mining {
        return;
    }
    let mut mining_list: HashMap<u64, ResourceAllocation> = HashMap::new();
    for (tag, alloc) in &bot.worker_allocator.resources {
        mining_list.insert(*tag, ResourceAllocation {
            resource_tag: alloc.resource_tag,
            worker_role: alloc.worker_role,
            workers: alloc.workers.clone(),
        });
    }

    for (tag, alloc) in mining_list {
        let mut resource_position: Option<Point2> = None;
        let mut resource_color = "";

        if alloc.worker_role == WorkerRole::Mineral {
            if let Some(mineral) = bot.units.mineral_fields.iter().find_tag(tag.clone()) {
                let position = mineral.position();
                let size = 0.5;
                let workers_amount = alloc.workers.len().clone().to_string();
                resource_color = "blue";
                bot.debug_cube(position, size, resource_color);
                bot.debug_text(workers_amount.as_str(), position, "yellow", Some(16));
                resource_position = Some(position.clone());
            } else {
                println!("[DEBUGGER] {} Mineral with tag {} not found", bot.debugger.time, tag);
            }
        } else {
            if let Some(refinery) = bot.units.my.structures.iter().find_tag(tag.clone()) {
                let position = refinery.position();
                let size = 1.5;
                resource_color = "green";
                bot.debug_cube(position, size, resource_color);
                resource_position = Some(position.clone());
            } else {
                println!("[DEBUGGER] {} Refinery with tag {} not found", bot.debugger.time, tag);
            }
        }
        
        if let Some(resource_pos) = resource_position {
            for worker_tag in alloc.workers {
                if let Some(worker) = bot.units.my.workers.iter().find_tag(worker_tag) {
                    let worker_pos = worker.position();
                    let color = "yellow";
                    bot.debug_sphere(worker_pos, 0.3, resource_color);
                    bot.debug_line(worker_pos, resource_pos, color);
                } else {
                    if let Some(role) = bot.worker_allocator.worker_roles.get(&worker_tag) {
                        if role == &WorkerRole::Gas {
                            continue;
                        }
                        println!("[DEBUGGER] {} (1) Worker with tag {}, role {:?} not found", bot.debugger.time, worker_tag, role);
                    } else {
                        println!("[DEBUGGER] {} (1) Worker with tag {}, without role not found", bot.debugger.time, worker_tag);
                    }
                }
            }
        }
    }
}

pub fn debug_show_worker_mining_steps(bot: &mut Nikolaj) {
    if !bot.debugger.displaying_worker_mining_steps {
        return;
    }
    let mut worker_infos = Vec::new();
    for worker_step in &bot.worker_allocator.debugger.workers_current_mining_steps {
        if let Some(role) = bot.worker_allocator.worker_roles.get(&worker_step.tag) {
            if role != &WorkerRole::Mineral {
                continue;
            }
        }
        if let Some(worker) = bot.units.my.workers.iter().find_tag(worker_step.tag) {
            let position = worker.position();
            let color = match worker_step.step {
                WorkersMiningSteps::MineralFarAway => "magenta",
                WorkersMiningSteps::MineralOffsetWalk => "blue",
                WorkersMiningSteps::MineralGather => "green",
                WorkersMiningSteps::BaseFarAway => "red",
                WorkersMiningSteps::BaseOffsetWalk => "orange",
                WorkersMiningSteps::BaseReturn => "yellow",
                WorkersMiningSteps::None => "white",
            };
            let text = format!("{:?}", worker_step.step);
            worker_infos.push((text, position, color));
        } else {
            println!("[DEBUGGER] {} (2) Worker with tag {} not found", bot.debugger.time, worker_step.tag);
        }
    }
    for (text, pos, color) in worker_infos {
        bot.debug_text(&text, pos, color, Some(14));
    }
}

pub fn debug_show_worker_roles(bot: &mut Nikolaj) {
    if !bot.debugger.displaying_worker_roles {
        return;
    }
    let mut worker_infos = Vec::new();
    for worker in bot.units.my.workers.iter() {
        let position = worker.position();
        let tag = worker.tag();
        if let Some(role) = bot.worker_allocator.worker_roles.get(&tag) {
            let text = match role {
                WorkerRole::Idle => "Idle",
                WorkerRole::Mineral => "Mineral",
                WorkerRole::Gas => "Gas",
                WorkerRole::Busy => "Busy",
                WorkerRole::Repair => "Repair",
            };
            let color = match role {
                WorkerRole::Idle => "red",
                WorkerRole::Mineral => "blue",
                WorkerRole::Gas => "green",
                WorkerRole::Busy => "yellow",
                WorkerRole::Repair => "white",
            };
            worker_infos.push((text, position, color));
        } else {
            println!("[DEBUGGER] {} Worker role for tag {} not found", bot.debugger.time, tag);
        }
    }
    for (_text, pos, color) in worker_infos {
        bot.debug_sphere(pos, 0.5, color);
    }
}

pub fn debug_show_repair(bot: &mut Nikolaj) {
    if !bot.debugger.displaying_repair {
        return;
    }
    let mut repair_list: HashMap<u64, RepairAllocation> = HashMap::new();
    for (tag, alloc) in &bot.worker_allocator.repair {
        repair_list.insert(*tag, RepairAllocation {
            tag: alloc.tag,
            workers: alloc.workers.clone(),
            is_structure: alloc.is_structure,
            max_workers: alloc.max_workers,
        });
    }

    for (tag, alloc) in repair_list {
        let mut target_position: Option<Point2> = None;
        let mut target_color = "";

        if alloc.is_structure {
            if let Some(structure) = bot.units.my.structures.iter().find_tag(tag.clone()) {
                let position = structure.position();
                let size = 2.0;
                target_color = "white";
                bot.debug_cube(position, size, target_color);
                target_position = Some(position.clone());
            } else {
                println!("[DEBUGGER] {} Structure with tag {} not found", bot.debugger.time, tag);
            }
        } else {
            if let Some(unit) = bot.units.my.units.iter().find_tag(tag.clone()) {
                let position = unit.position();
                let size = 1.0;
                target_color = "white";
                bot.debug_cube(position, size, target_color);
                target_position = Some(position.clone());
            } else {
                println!("[DEBUGGER] {} Unit with tag {} not found", bot.debugger.time, tag);
            }
        }
        
        if let Some(target_pos) = target_position {
            for worker_tag in alloc.workers {
                if let Some(worker) = bot.units.my.workers.iter().find_tag(worker_tag) {
                    let worker_pos = worker.position();
                    let color = "white";
                    bot.debug_sphere(worker_pos, 0.5, target_color);
                    bot.debug_line(worker_pos, target_pos, color);
                } else {
                    println!("[DEBUGGER] {} (3) Worker with tag {} not found", bot.debugger.time, worker_tag);
                }
            }
        }
    }
}

pub fn debug_show_strategy_points(bot: &mut Nikolaj) {
    if !bot.debugger.displaying_strategy_points {
        return;
    }

    let start_point = bot.start_location;
    let idle_point = bot.strategy_data.idle_point;
    let defense_point = bot.strategy_data.defense_point;
    let defense_active = bot.strategy_data.defend;
    let attack_point = bot.strategy_data.attack_point;
    
    // Idle point - main hub
    bot.debug_sphere(idle_point, 0.5, "white");
    bot.debug_text("IDLE", idle_point, "white", Some(14));
    bot.debug_line(start_point, idle_point, "white");

    // Defense point - from idle
    if defense_active{
        bot.debug_sphere(defense_point, 0.5, "yellow");
        bot.debug_text("DEFENSE", defense_point, "yellow", Some(14));
        bot.debug_line_offset(idle_point, defense_point, "yellow", 0.0);
    }

    // Attack point - from idle
    bot.debug_sphere(attack_point, 0.5, "red");
    bot.debug_text("ATTACK", attack_point, "red", Some(14));
    bot.debug_line_offset(idle_point, attack_point, "red", 0.5);

    // Harass points - from idle
    let harass_points = bot.strategy_data.harass_points.clone();
    for (i, point) in harass_points.iter().enumerate() {
        bot.debug_sphere(*point, 0.5, "blue");
        bot.debug_text(&format!("HARASS {}", i + 1), *point, "blue", Some(14));
        bot.debug_line_offset(idle_point, *point, "blue", 1.0 + (i as f32 * 0.5));
    }
    
    // Army center - from idle
    let army_center = bot.strategy_data.army_center;
    bot.debug_sphere(army_center, 0.5, "orange");
    bot.debug_text("ARMY", army_center, "orange", Some(14));
    bot.debug_line_offset(idle_point, army_center, "orange", 2.0);

    // Repair points - standalone
    let repair_points = bot.strategy_data.repair_points.clone();
    for (i, point) in repair_points.iter().enumerate() {
        bot.debug_sphere(*point, 0.5, "green");
        bot.debug_text(&format!("REPAIR {}", i + 1), *point, "green", Some(14));
        bot.debug_line_offset(start_point, *point, "green", 2.5 + (i as f32 * 0.5));
    }
}

pub fn debug_display_selected(bot: &mut Nikolaj) {
    if !bot.debugger.displaying_selected {
        return;
    }
    let mut selected_units: Vec<Unit> = vec![];
    for unit in bot.units.my.all.iter().filter(|u| u.is_selected()) {
        selected_units.push(unit.clone());
    }
    for unit in selected_units {
        let position = unit.position();
        let text = format!("Tag: {}\nType: {:?}", unit.tag(), unit.type_id());
        if let Some(worker_role) = bot.worker_allocator.worker_roles.get(&unit.tag()) {
            let role_text = format!("\nRole: {:?}", worker_role);
            let full_text = format!("{}{}", text, role_text);
            bot.debug_text(&full_text, position, "white", Some(14));
            continue;
        }
        bot.debug_text(&text, position, "white", Some(14));
    }
}

pub fn debug_show_heatmaps(bot: &mut Nikolaj) {
    if !bot.debugger.displaying_heatmaps {
        return;
    }
    let mut selected_units: Vec<Unit> = vec![];
    for unit in bot.units.my.all.iter().filter(|u| u.is_selected()) {
        selected_units.push(unit.clone());
    }
    let mut debug_texts: Vec<(String, Point2, &str, Option<u32>)> = vec![];
    for unit in selected_units {
        if let Some(heatmap) = bot.combat_info.heatmaps.get(&unit.tag()) {
            for point in &heatmap.points {
                let position = point.position;
                let intensity = point.intensity;
                if intensity == 0.0 {
                    continue;
                }
                let color: &str;
                if intensity >= 1000.0 {
                    color = "green";
                } else if intensity > 0.0 {
                    color = "yellow";
                } else {
                    color = "red";
                };
                debug_texts.push((format!("{:.0}", intensity), position, color, Some(10)));
            }
        }
    }
    for (text, position, color, size) in debug_texts {
        bot.debug_text(&text, position, color, size);
    }
}

pub fn debug_show_strategy_monitor(bot: &mut Nikolaj) {
    if !bot.debugger.displaying_strategy_monitor {
        return;
    }
    
    let mut status_texts = Vec::new();
    let mut y_offset = 0;
    let letter_size = 16;
    let offset = letter_size / 2;
    
    if bot.strategy_data.attack {
        status_texts.push(("ATTACK MODE", "red", y_offset));
        y_offset += offset;
    }
    
    if bot.strategy_data.defend {
        status_texts.push(("DEFEND MODE", "yellow", y_offset));
        y_offset += offset;
    }
    
    if bot.strategy_data.enemy_flooding {
        status_texts.push(("ENEMY FLOODING", "orange", y_offset));
        y_offset += offset;
    }
    
    if bot.strategy_data.enemy_worker_rush {
        status_texts.push(("ENEMY WORKER RUSH", "red", y_offset));
        y_offset += offset;
    }
    
    if bot.strategy_data.enemy_ramp_blocking {
        status_texts.push(("ENEMY RAMP BLOCK", "blue", y_offset));
    }
    
    for (text, color, offset) in status_texts {
        let position = Point2::new(0.02, 0.1 + (offset as f32 * 0.03));
        bot.debug_text_screen(text, position, color, Some(letter_size));
    }
}
