use crate::Nikolaj;
use rust_sc2::prelude::*;
use crate::units::helpers::workers_assignments::*;
use std::{collections::HashMap,};


#[derive(Debug, Clone)]
pub struct NikolajDebugger{
    pub time: f32,
    pub printing_full_resource_assignments: bool,
    pub printing_bases_assignments: bool,
    pub printing_workers_assignments: bool,
    pub printing_resources_assignments: bool,
    pub printing_full_repair_assignments: bool,
    pub printing_repair_targets_assignments: bool,
    pub printing_construction: bool,
    pub printing_full_construction_info: bool,
    pub printing_combat_info: bool,
    pub printing_build_order: bool,
    pub printing_research: bool,
    pub displaying_worker_roles: bool,
    pub displaying_worker_mining_steps: bool,
    pub displaying_bases: bool,
    pub displaying_repair: bool,
    pub displaying_mining: bool,
    pub displaying_strategy_points: bool,
    pub displaying_selected: bool,
    pub displaying_heatmaps: bool,
    pub run_resource_assignments_checks: bool,
    pub workers_current_mining_steps: Vec<WorkersCurrentMiningStep>,
}
impl Default for NikolajDebugger {
    fn default() -> Self {
        #[cfg(feature = "wine_sc2")]
        {
            Self {
                time: 0.0,
                printing_full_resource_assignments: false,
                printing_bases_assignments: false,
                printing_workers_assignments: false,
                printing_resources_assignments: false,
                printing_full_repair_assignments: false,
                printing_repair_targets_assignments: false,
                printing_construction: true,
                printing_full_construction_info: false,
                printing_combat_info: false,
                printing_build_order: false,
                printing_research: false,
                displaying_worker_roles: false,
                displaying_worker_mining_steps: false,
                displaying_bases: false,
                displaying_repair: false,
                displaying_mining: true,
                displaying_strategy_points: false,
                displaying_selected: true,
                displaying_heatmaps: true,
                run_resource_assignments_checks: false,
                workers_current_mining_steps: vec![],
            }
        }
        #[cfg(not(feature = "wine_sc2"))]
        {
            Self {
                time: 0.0,
                printing_full_resource_assignments: false,
                printing_bases_assignments: false,
                printing_workers_assignments: false,
                printing_resources_assignments: false,
                printing_full_repair_assignments: false,
                printing_repair_targets_assignments: false,
                printing_construction: true,
                printing_full_construction_info: false,
                printing_combat_info: false,
                printing_build_order: false,
                printing_research: false,
                displaying_worker_roles: false,
                displaying_worker_mining_steps: false,
                displaying_bases: false,
                displaying_repair: false,
                displaying_mining: false,
                displaying_strategy_points: false,
                displaying_selected: true,
                displaying_heatmaps: false,
                run_resource_assignments_checks: false,
                workers_current_mining_steps: vec![],
            }
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
    debug_print_repair(bot);
    debug_show_worker_roles(bot);
    debug_show_strategy_points(bot);
    debug_print_resource_assignments(bot);
    debug_show_worker_mining_steps(bot);
    debug_print_combat_info(bot);
    debug_print_build_order(bot);
    debug_resource_assignments_checks(bot);
    debug_display_selected(bot);
    debug_print_full_construction_info(bot);
    debug_show_heatmaps(bot);
}
// Debugging
fn debug_show_bases(
    bot: &mut Nikolaj
) {
    if !bot.debugger.displaying_bases {
        return;
    }
    // Bases
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

fn debug_show_mining(
    bot: &mut Nikolaj
) {
    if !bot.debugger.displaying_mining {
        return;
    }
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
                let workers_amount = alloc.workers.len().clone().to_string();
                resource_color = "blue";
                bot.debug_cube(position, size, resource_color);

                bot.debug_text(workers_amount.as_str(), position, "yellow", Some(16));
                resource_position = Some(position.clone());
            } else {
                println!("[DEBUGGER] {} Mineral with tag {} not found", bot.debugger.time, tag);
            }
        } else {
            if let Some(refinery) = bot.units.my.structures.iter().find_tag(tag.clone()){
                let position = refinery.position();
                let size = 1.5;
                resource_color = "green";
                bot.debug_cube(position, size, resource_color);
                resource_position = Some(position.clone());
            } else {
                println!("[DEBUGGER] {} Refinery with tag {} not found", bot.debugger.time, tag);
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
                        println!("[DEBUGGER] {} (1) Worker with tag {}, role {:?} not found", bot.debugger.time, worker_tag, role);
                    } else {
                        println!("[DEBUGGER] {} (1) Worker with tag {}, without role not found", bot.debugger.time, worker_tag);
                    }
                }
            }
        }
    }
}

fn debug_show_worker_mining_steps(
    bot: &mut Nikolaj
) {
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

fn debug_show_worker_roles(
    bot: &mut Nikolaj
) {
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
        // bot.debug_text(text, pos, color, Some(14));
    }
}

fn debug_show_repair(
    bot: &mut Nikolaj
) {
    if !bot.debugger.displaying_repair {
        return;
    }
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
                println!("[DEBUGGER] {} Structure with tag {} not found", bot.debugger.time, tag);
            }
        } else {
            if let Some(unit) = bot.units.my.units.iter().find_tag(tag.clone()){
                let position = unit.position();
                let size = 1.0;
                target_color = "white";
                bot.debug_cube(position, size, target_color);
                target_position = Some(position.clone());
            } else {
                println!("[DEBUGGER] {} Unit with tag {} not found", bot.debugger.time, tag);
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
                    println!("[DEBUGGER] {} (3) Worker with tag {} not found", bot.debugger.time, worker_tag);
                }
            }
        }
    }
}

fn debug_print_repair(
    bot: &mut Nikolaj
) {
    if !bot.debugger.printing_full_repair_assignments {
        return;
    }
    if bot.worker_allocator.repair.is_empty() {
        return;
    }

    println!("--- Repair Assignments ---");
    for (tag, alloc) in &bot.worker_allocator.repair {
        let workers: Vec<String> = alloc.workers.iter().map(|w| w.to_string()).collect();
        println!(
            "Repair Target Tag: {}, Is Structure: {}, Max Workers: {}, Workers: [{}]", 
            tag, 
            alloc.is_structure, 
            alloc.max_workers, 
            workers.join(", ")
        );
    }
    println!("--------------------------");
}

fn debug_show_strategy_points(
    bot: &mut Nikolaj
) {
    if !bot.debugger.displaying_strategy_points {
        return;
    }
    // Idle point
    let idle_point = bot.strategy_data.idle_point;
    bot.debug_sphere(idle_point, 0.5, "white");
    bot.debug_text("IDLE", idle_point, "white", Some(14));

    // Defense point
    let defense_point = bot.strategy_data.defense_point;
    bot.debug_sphere(defense_point, 0.5, "yellow");
    bot.debug_text("DEFENSE", defense_point, "yellow", Some(14));

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

fn debug_print_resource_assignments(
    bot: &mut Nikolaj
) {
    if !bot.debugger.printing_full_resource_assignments {
        return;
    }

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

pub fn print_new_bases_assignments(old_bases: &Vec<u64>, new_bases: &Vec<u64>, time: f32) {
    for base in new_bases {
        if !old_bases.contains(base) {
            println!("[DEBUGGER] {} New base added with tag {}", time, base);
        }
    }
    for base in old_bases {
        if !new_bases.contains(base) {
            println!("[DEBUGGER] {} Base removed with tag {}", time, base);
        }
    }
}

fn debug_print_combat_info(
    bot: &mut Nikolaj
) {
    if !bot.debugger.printing_combat_info {
        return;
    }
    if bot.combat_info.unsiege_timer.is_empty() {
        return;
    }
    println!("--- Combat Info ---");
    for timer in &bot.combat_info.unsiege_timer {
        println!(
            "Unit Tag: {}, Unsiege in: {:.2} seconds", 
            timer.tag, 
            timer.unsiege_in
        );
    }
    println!("-------------------");
}

fn debug_print_build_order(
    bot: &mut Nikolaj
) {
    if !bot.debugger.printing_build_order {
        return;
    }
    println!("--- Build Order ---");
    if let Some(ref priority) = bot.macro_manager.barracks_priority {
        println!("Barracks Priority: {:?}", priority);
    } else {
        println!("Barracks Priority: None");
    }
    if let Some(ref priority) = bot.macro_manager.factory_priority {
        println!("Factory Priority: {:?}", priority);
    } else {
        println!("Factory Priority: None");
    }
    if let Some(ref priority) = bot.macro_manager.starport_priority {
        println!("Starport Priority: {:?}", priority);
    } else {
        println!("Starport Priority: None");
    }
    println!("-------------------");
}

fn debug_resource_assignments_checks(bot: &mut Nikolaj) {
    if !bot.debugger.run_resource_assignments_checks {
        return;
    }
    let mut used_workers: HashMap<u64, WorkerRole> = HashMap::new();
    for (_, alloc) in &bot.worker_allocator.resources {
        for worker_tag in &alloc.workers {
            if let Some(existing_role) = used_workers.get(worker_tag) {
                println!("[DEBUGGER] {} Worker with tag {} assigned to multiple resources: {:?} and {:?}", bot.debugger.time, worker_tag, existing_role, alloc.worker_role);
            } else {
                used_workers.insert(*worker_tag, alloc.worker_role.clone());
            }
        }
    }
}

fn debug_display_selected(bot: &mut Nikolaj) {
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

fn debug_print_full_construction_info(bot: &mut Nikolaj) {
    if !bot.debugger.printing_full_construction_info {
        return;
    }
    if bot.construction_info.under_construction.is_empty() {
        return;
    }
    println!("--- Construction Info ---");
    for construction in &bot.construction_info.under_construction {
        println!(
            "Structure: {:?}, Worker: {}, Position: {:?}, Started at: {:.2}",
            construction.structure,
            construction.worker_tag,
            construction.position,
            construction.time_started
        );
    }
    println!("-------------------------");
}

fn debug_show_heatmaps(bot: &mut Nikolaj) {
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