use crate::Nikolaj;
use rust_sc2::prelude::*;
use crate::units::helpers::workers_assignments::*;
use std::{collections::HashMap,};


#[derive(Debug, Clone)]
pub struct NikolajDebugger{
    pub printing_full_resource_assignments: bool,
    pub printing_bases_assignments: bool,
    pub printing_workers_assignments: bool,
    pub printing_resources_assignments: bool,
    pub printing_repair_targets_assignments: bool,
    pub printing_construction: bool,
    pub displaying_worker_mining_steps: bool,
    pub workers_current_mining_steps: Vec<WorkersCurrentMiningStep>,
}
impl Default for NikolajDebugger {
    fn default() -> Self {
        Self {
            printing_full_resource_assignments: false,
            printing_bases_assignments: true,
            printing_workers_assignments: true,
            printing_resources_assignments: true,
            printing_repair_targets_assignments: true,
            printing_construction: true,
            displaying_worker_mining_steps: true,
            workers_current_mining_steps: vec![],
        }
    }
}
impl NikolajDebugger {
    pub fn add_mining_step(&mut self, tag: u64, step: WorkersMiningSteps) {
        if let Some(existing) = self.workers_current_mining_steps.iter_mut().find(|w| w.tag == tag) {
            existing.step = step;
        } else {
            self.workers_current_mining_steps.push(WorkersCurrentMiningStep {
                tag,
                step,
            });
        }
    }
}

pub fn debug_step(
    bot: &mut Nikolaj
) {
    debug_show_bases(bot);
    debug_show_mining(bot);
    debug_show_repair(bot);
    debug_show_worker_roles(bot);
    debug_show_strategy_points(bot);
    if bot.debugger.printing_full_resource_assignments {
        debug_print_resource_assignments(bot);
    }
    if bot.debugger.displaying_worker_mining_steps {
        debug_show_worker_mining_steps(bot);
    }
}
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
                    if let Some(role) = bot.worker_allocator.worker_roles.get(&worker_tag) {
                        if role == &WorkerRole::Gas {
                            continue;
                        }

                        println!("Debugger: (1) Worker with tag {}, role {:?} not found", worker_tag, role);
                    }
                    println!("Debugger: (1) Worker with tag {}, without role not found", worker_tag);
                }
            }
        }
    }
}

fn debug_show_worker_mining_steps(
    bot: &mut Nikolaj
) {
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
            println!("Debugger: (2) Worker with tag {} not found", worker_step.tag);
        }
    }
    for (text, pos, color) in worker_infos {
        bot.debug_text(&text, pos, color, Some(14));
    }
}

pub fn debug_show_worker_roles(
    bot: &mut Nikolaj
) {
    let mut worker_infos = Vec::new();
    for worker in bot.units.my.workers.iter() {
        if let Some(role) = bot.worker_allocator.worker_roles.get(&worker.tag()) {
            if role != &WorkerRole::Mineral {
                continue;
            }
        }
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
    for (_text, pos, color) in worker_infos {
        bot.debug_sphere(pos, 0.5, color);
        // bot.debug_text(text, pos, color, Some(14));
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
                    println!("Debugger: (3) Worker with tag {} not found", worker_tag);
                }
            }
        }
    }
}

pub fn debug_show_strategy_points(
    bot: &mut Nikolaj
) {
    // Idle point
    let idle_point = bot.strategy_data.idle_point;
    bot.debug_sphere(idle_point, 0.5, "white");
    bot.debug_text("IDLE", idle_point, "white", Some(14));

    // Defense point
    let defense_point = bot.strategy_data.defense_point;
    bot.debug_sphere(defense_point, 0.5, "yellow");
    bot.debug_text("DEFENSE", defense_point, "yellow", Some(1));

    // Attack point
    let attack_point = bot.strategy_data.attack_point;
    bot.debug_sphere(attack_point, 0.5, "red");
    bot.debug_text("ATTACK", attack_point, "red", Some(14));

    // Harass points
    let harass_points = bot.strategy_data.harass_points.clone();
    for (i, point) in harass_points.iter().enumerate() {
        bot.debug_sphere(*point, 0.5, "blue");
        bot.debug_text(&format!("HARASS {}", i + 1), *point, "blue", Some(14));
    }
    // Repair points
    let repair_points = bot.strategy_data.repair_points.clone();
    for (i, point) in repair_points.iter().enumerate() {
        bot.debug_sphere(*point, 0.5, "green");
        bot.debug_text(&format!("REPAIR {}", i + 1), *point, "green", Some(14));
    }
    
}

pub fn debug_print_resource_assignments(
    bot: &mut Nikolaj
) {
    println!("--- Resource Assignments ---");
    for (tag, alloc) in &bot.worker_allocator.resources {
        let workers: Vec<String> = alloc.workers.iter().map(|w| w.to_string()).collect();
        println!(
            "Resource Tag: {}, Role: {:?}, Workers: [{}]", 
            tag, 
            alloc.worker_role, 
            workers.join(", ")
        );
    }
    println!("----------------------------");
}

pub fn print_new_bases_assignments(old_bases: &Vec<u64>, new_bases: &Vec<u64>) {
    for base in new_bases {
        if !old_bases.contains(base) {
            println!("Debugger: New base added with tag {}", base);
        }
    }
    for base in old_bases {
        if !new_bases.contains(base) {
            println!("Debugger: Base removed with tag {}", base);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WorkersMiningSteps {
    MineralFarAway,
    MineralOffsetWalk,
    MineralGather,
    BaseFarAway,
    BaseOffsetWalk,
    BaseReturn,
    None
}

#[derive(Debug, Clone)]
pub struct WorkersCurrentMiningStep {
    pub tag: u64,
    pub step: WorkersMiningSteps,
}