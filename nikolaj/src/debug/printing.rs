use crate::Nikolaj;
use crate::units::helpers::workers_assignments::WorkerRole;
use std::collections::HashMap;

pub fn debug_print_repair(bot: &mut Nikolaj) {
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

pub fn debug_print_resource_assignments(bot: &mut Nikolaj) {
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

pub fn debug_print_combat_info(bot: &mut Nikolaj) {
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

pub fn debug_print_build_order(bot: &mut Nikolaj) {
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

pub fn debug_resource_assignments_checks(bot: &mut Nikolaj) {
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

pub fn debug_print_full_construction_info(bot: &mut Nikolaj) {
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

pub fn debug_print_enemy_army_snapshot(bot: &mut Nikolaj) {
    if !bot.debugger.printing_enemy_army_snapshot {
        return;
    }
    let mut enemy_army_snapshot = bot.strategy_data.enemy_army.units.clone();
    if enemy_army_snapshot.is_empty() {
        return;
    }
    enemy_army_snapshot.sort_by_key(|u| u.type_id as u32);

    println!("--- Enemy Army Snapshot ---");
    for unit in enemy_army_snapshot {
        println!("Type: {:?}, Tag: {}, Snapshot: {}, At: {}", unit.type_id, unit.id, unit.is_snapshot, unit.last_seen);
    }
    println!("---------------------------");
}
