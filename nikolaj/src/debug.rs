use crate::Nikolaj;
use rust_sc2::prelude::*;
use crate::units::scv::*;
use std::collections::HashMap;

// Debugging
pub fn debug_show_bases(
    bot: &mut Nikolaj
) {

    // Bases
    let bases = bot.worker_allocator.bases.clone();
    for base_tag in bases {
        if let Some(base) = bot.units.my.structures.iter().find_tag(base_tag) {
            let position = base.position();
            let size = 2.5;
            let color = "yellow";
            bot.debug_cube(position, size, color);
        } else {
            println!("Debugger: Base with tag {} not found", base_tag);
        }
    }
}

pub fn debug_show_mining(
    bot: &mut Nikolaj
) {
    // Clone list
    let mut mining_list:HashMap<u64, ResourceAllocation> = HashMap::new();
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

        // Show resource
        if alloc.worker_role == WorkerRole::Mineral {
            if let Some(mineral) = bot.units.mineral_fields.iter().find_tag(tag.clone()){
                let position = mineral.position();
                let size = 0.5;
                resource_color = "blue";
                bot.debug_cube(position, size, resource_color);
                resource_position = Some(position.clone());
            } else {
                println!("Debugger: Mineral with tag {} not found", tag);
            }
        } else {
            if let Some(refinery) = bot.units.my.structures.iter().find_tag(tag.clone()){
                let position = refinery.position();
                let size = 1.5;
                resource_color = "green";
                bot.debug_cube(position, size, resource_color);
                resource_position = Some(position.clone());
            } else {
                println!("Debugger: Refinery with tag {} not found", tag);
            }
        }
        // Show workers
        if let Some(resource_pos) = resource_position {
            for worker_tag in alloc.workers {
                if let Some(worker) = bot.units.my.workers.iter().find_tag(worker_tag) {
                    let worker_pos = worker.position();
                    let color = "yellow";
                    bot.debug_sphere(worker_pos, 0.3, resource_color);
                    bot.debug_line(worker_pos, resource_pos, color);
                } else {
                    println!("Debugger: Worker with tag {} not found", worker_tag);
                }
            }
        }
    }
}

pub fn debug_show_worker_roles(
    bot: &mut Nikolaj
) {
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
            println!("Debugger: Worker role for tag {} not found", tag);
        }
    }
    for (text, pos, color) in worker_infos {
        bot.debug_sphere(pos, 0.5, color);
        bot.debug_text(text, pos, color, Some(1));
    }
}

pub fn debug_show_repair(
    bot: &mut Nikolaj
) {
    // Clone list
    let mut repair_list:HashMap<u64, RepairAllocation> = HashMap::new();
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

        // Show target
        if alloc.is_structure {
            if let Some(structure) = bot.units.my.structures.iter().find_tag(tag.clone()){
                let position = structure.position();
                let size = 2.0;
                target_color = "white";
                bot.debug_cube(position, size, target_color);
                target_position = Some(position.clone());
            } else {
                println!("Debugger: Structure with tag {} not found", tag);
            }
        } else {
            if let Some(unit) = bot.units.my.units.iter().find_tag(tag.clone()){
                let position = unit.position();
                let size = 1.0;
                target_color = "white";
                bot.debug_cube(position, size, target_color);
                target_position = Some(position.clone());
            } else {
                println!("Debugger: Unit with tag {} not found", tag);
            }
        }
        // Show workers
        if let Some(target_pos) = target_position {
            for worker_tag in alloc.workers {
                if let Some(worker) = bot.units.my.workers.iter().find_tag(worker_tag) {
                    let worker_pos = worker.position();
                    let color = "white";
                    bot.debug_sphere(worker_pos, 0.5, target_color);
                    bot.debug_line(worker_pos, target_pos, color);
                } else {
                    println!("Debugger: Worker with tag {} not found", worker_tag);
                }
            }
        }
    }
}
