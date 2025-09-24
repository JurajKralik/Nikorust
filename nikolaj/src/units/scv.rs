use crate::{debug::{print_new_bases_assignments, NikolajDebugger}, Nikolaj};
use rust_sc2::prelude::*;
use rust_sc2::units::AllUnits;
use std::collections::{HashMap, HashSet};
use crate::units::helpers::workers_assignments::*;


pub fn scv_step(bot: &mut Nikolaj) {
    // Worker rush
    if bot.strategy_data.enemy_worker_rush {
        counter_worker_rush(bot); // TODO: implement
        return;
    }

    // Bases
    let current_bases = bot.worker_allocator.bases.clone();
    bot.worker_allocator.bases = get_mining_bases(&bot.units);
    if bot.debugger.printing_bases_assignments {
        print_new_bases_assignments(&current_bases, &bot.worker_allocator.bases.clone());
    }
    
    // Repair
    let bases_tags = bot.worker_allocator.bases.iter().clone();
    let damaged_targets = collect_damaged_targets(&bot.units, bases_tags);
    bot.worker_allocator.update_repair_targets(&bot.units.clone(), damaged_targets);
    bot.worker_allocator.assign_repairmen(&bot.units.clone());

    // Ramp blocking
    if bot.strategy_data.enemy_ramp_blocking {
        counter_ramp_blocking(bot); // TODO: implement
    }

    // Resources
    bot.worker_allocator.update_idle_workers(&bot.units.clone());
    let valid_resources_tags = collect_valid_resource_tags(&bot.units.clone());
    bot.worker_allocator.update_resources(valid_resources_tags, &bot.units.clone());
    bot.worker_allocator.update_saturation();
    bot.worker_allocator.assign_resource_workers(&bot.units.clone());
    bot.worker_allocator.workers_movement(&bot.units.clone());
}

#[derive(Debug, Default)]
pub struct WorkerAllocator {
    pub debugger: NikolajDebugger,
    pub bases: Vec<u64>,
    pub repair: HashMap<u64, RepairAllocation>,
    pub resources: HashMap<u64, ResourceAllocation>,
    pub worker_roles: HashMap<u64, WorkerRole>,
    pub saturation: ResourceSaturation,
}

impl WorkerAllocator {
    fn set_worker_role(&mut self, worker_tag: u64, new_role: WorkerRole) {
        let printing =self.debugger.printing_workers_assignments;
        match self.worker_roles.insert(worker_tag, new_role) {
            Some(old_role) if old_role != new_role => {
                if printing {
                    println!(
                        "[ALLOCATOR] Worker {} changed role {:?} -> {:?}",
                        worker_tag, old_role, new_role
                    );
                }
            }
            None => {
                if printing {
                    println!(
                        "[ALLOCATOR] Worker {} assigned initial role {:?}",
                        worker_tag, new_role
                    );
                }
            }
            _ => {}
        }
    }

    fn add_resource(&mut self, tag: u64, role: WorkerRole) {
        let printing = self.debugger.printing_resources_assignments;
        if !self.resources.contains_key(&tag) {
            if printing {
                println!(
                    "[ALLOCATOR] Added {:?} resource {}",
                    role, tag
                );
            }
            self.resources.insert(
                tag,
                ResourceAllocation {
                    resource_tag: tag,
                    worker_role: role,
                    workers: Vec::new(),
                },
            );
        }
    }

    fn remove_resource(&mut self, tag: u64) {
        let printing = self.debugger.printing_resources_assignments;
        if self.resources.remove(&tag).is_some() {
            if printing {
                println!("[ALLOCATOR] Removed resource {}", tag);
            }
        }
    }

    fn add_repair_target(&mut self, tag: u64, alloc: RepairAllocation) {
        if !self.repair.contains_key(&tag) {
            if self.debugger.printing_repair_targets_assignments {
                println!(
                    "[ALLOCATOR] Added repair target {} (max_workers = {})",
                    tag, alloc.max_workers
                );
            }
            self.repair.insert(tag, alloc);
        }
    }

    fn remove_repair_target(&mut self, tag: u64) {
        if self.repair.remove(&tag).is_some() {
            if self.debugger.printing_repair_targets_assignments {
                println!("[ALLOCATOR] Removed repair target {}", tag);
            }
        }
    }

    fn update_repair_targets(
        &mut self,
        units: &AllUnits,
        damaged_targets: HashMap<u64, RepairAllocation>,
    ) {
        let valid_tags: HashSet<u64> = damaged_targets.keys().cloned().collect();
        let bases_tags = self.bases.iter().clone();
        let bases = units.my.structures.find_tags(bases_tags);

        let mut invalid_tags: Vec<u64> = Vec::new();
        let mut workers_to_idle: Vec<u64> = Vec::new();

        for (tag, alloc) in self.repair.iter_mut() {
            if valid_tags.contains(tag) {
                continue;
            }
            let target: Unit;
            if alloc.is_structure {
                target = units.my.structures.iter().find_tag(*tag).unwrap().clone();
            } else {
                target = units.my.units.iter().find_tag(*tag).unwrap().clone();
                let closest_base_distance = bases.closest_distance(target.position()).unwrap_or(0.0);
                if closest_base_distance > 20.0 {
                    invalid_tags.push(*tag);
                    continue;
                }
            }
            let health_percentage = target.health_percentage().unwrap_or(1.0);
            if health_percentage < 1.0 {
                continue;
            }
            let mut safe = true;
            for enemy in units.enemy.units.closer(target.sight_range(), target.position()) {
                if enemy.can_attack_ground() {
                    safe = false;
                    break;
                }
            }
            if !safe {
                continue;
            }
            invalid_tags.push(*tag);
            for worker_tag in alloc.workers.clone() {
                workers_to_idle.push(worker_tag);
            }
        }

        for worker_tag in workers_to_idle {
            self.set_worker_role(worker_tag, WorkerRole::Idle);
        }

        let workers_tags = units.my.workers.iter().map(|w| w.tag()).collect::<HashSet<u64>>();
        for tag in invalid_tags {
            if let Some(alloc) = self.repair.get(&tag) {
                for worker_tag in alloc.workers.clone() {
                    if !workers_tags.contains(&worker_tag) {
                        if self.debugger.printing_workers_assignments {
                            println!(
                                "[ALLOCATOR] Worker {} dead. Removed from repair target {}",
                                worker_tag, tag
                            );
                        }
                        self.worker_roles.remove(&worker_tag);
                    } else {
                        if self.debugger.printing_workers_assignments {
                            println!(
                                "[ALLOCATOR] Worker {} set to Idle. Removed from repair target {}",
                                worker_tag, tag
                            );
                        }
                        self.set_worker_role(worker_tag, WorkerRole::Idle);
                    }
                }
            }
            if self.debugger.printing_repair_targets_assignments {
                println!("[ALLOCATOR] Removed repair target {}", tag);
            }
            self.remove_repair_target(tag);
        }

        for (tag, alloc) in damaged_targets {
            self.add_repair_target(tag, alloc);
        }
    }

    fn assign_repairmen(&mut self, units: &AllUnits) {
        let workers = units.my.workers.ready().clone();
        let worker_tags = workers.iter().map(|w| w.tag()).collect::<HashSet<u64>>();

        let mut workers_to_assign: Vec<u64> = Vec::new();
        let mut repair_orders: Vec<(u64, u64)> = Vec::new(); 

        for (tag, alloc) in self.repair.iter_mut() {
            let mut current_workers = alloc.workers.clone();
            let max_workers = alloc.max_workers;
            let target: Unit = if alloc.is_structure {
                units.my.structures.iter().find_tag(*tag).unwrap().clone()
            } else {
                units.my.units.iter().find_tag(*tag).unwrap().clone()
            };

            for worker in current_workers.clone() {
                if !worker_tags.contains(&worker) {
                    current_workers.retain(|w| *w != worker);
                }
            }

            if current_workers.len() < max_workers {
                let workers_sorted = workers.iter().sort_by_distance(target.clone());
                for worker in workers_sorted {
                    let worker_tag = worker.tag();
                    if current_workers.len() >= max_workers {
                        break;
                    }
                    if worker.distance(target.clone()) > 25.0 {
                        break;
                    }
                    if let Some(role) = self.worker_roles.get(&worker_tag) {
                        if *role == WorkerRole::Busy || *role == WorkerRole::Repair {
                            continue;
                        }
                    }
                    current_workers.push(worker_tag);
                }
            }

            alloc.workers = current_workers.clone();

            for worker_tag in current_workers {
                let worker = units.my.workers.iter().find_tag(worker_tag).unwrap().clone();
                if worker.is_repairing() {
                    continue;
                }
                workers_to_assign.push(worker_tag);
                repair_orders.push((worker_tag, *tag));
            }
        }

        for worker_tag in workers_to_assign {
            if self.debugger.printing_workers_assignments {
                println!("[ALLOCATOR] Worker {} assigned to Repair", worker_tag);
            }
            self.set_worker_role(worker_tag, WorkerRole::Repair);
        }
        for (worker_tag, target_tag) in repair_orders {
            if let Some(worker) = units.my.workers.iter().find_tag(worker_tag) {
                worker.repair(target_tag, false);
            }
        }
    }


    fn update_idle_workers(&mut self, units: &AllUnits) {
        for worker in units.my.workers.ready().clone() {
            let worker_tag = worker.tag();
            if !self.worker_roles.contains_key(&worker_tag) {
                if self.debugger.printing_workers_assignments {
                    println!("New worker without role detected: {}", worker_tag);
                }
                self.set_worker_role(worker_tag, WorkerRole::Idle);
            } else if self.worker_roles.get(&worker_tag).unwrap() == &WorkerRole::Busy {
                if worker.is_idle() || worker.is_gathering() || worker.is_repairing() {
                    if self.debugger.printing_workers_assignments {
                        println!("Worker {} finished task. Set to Idle", worker_tag);
                    }
                    self.set_worker_role(worker_tag, WorkerRole::Idle);
                }
            }
        }
    }

    fn update_resources(
        &mut self,
        valid_minerals_and_refineries: (HashSet<u64>, HashSet<u64>),
        units: &AllUnits,
    ) {
        let (valid_minerals, valid_refineries) = valid_minerals_and_refineries;
        let valid_resources: HashSet<u64> = valid_minerals.union(&valid_refineries).cloned().collect();
        self.remove_invalid_resources(&valid_resources, units);
        for tag in valid_minerals {
            self.add_resource(tag, WorkerRole::Mineral);
        }
        for tag in valid_refineries {
            self.add_resource(tag, WorkerRole::Gas);
        }
    }

    fn remove_invalid_resources(&mut self, valid_resources: &HashSet<u64>, units: &AllUnits) {
        let worker_tags = units.my.workers.iter().map(|w| w.tag()).collect::<HashSet<u64>>();

        let mut invalid_resources_tags: Vec<u64> = Vec::new();
        let mut workers_to_idle: Vec<u64> = Vec::new();

        for (tag, alloc) in self.resources.iter() {
            if !valid_resources.contains(tag) {
                invalid_resources_tags.push(*tag);

                for worker_tag in alloc.workers.clone() {
                    if !worker_tags.contains(&worker_tag) {
                        self.worker_roles.remove(&worker_tag);
                    } else {
                        workers_to_idle.push(worker_tag);
                    }
                }
            }
        }
        for worker_tag in workers_to_idle {
            if self.debugger.printing_workers_assignments {
                println!(
                    "[ALLOCATOR] Worker {} set to Idle. Removed from resource",
                    worker_tag
                );
            }
            self.set_worker_role(worker_tag, WorkerRole::Idle);
        }

        for tag in invalid_resources_tags {
            if self.debugger.printing_full_resource_assignments {
                println!("[ALLOCATOR] Removed resource {}", tag);
            }
            self.remove_resource(tag);
        }
    }


    fn update_saturation(&mut self) {
        self.saturation = ResourceSaturation {
            mineral_tags_undersaturated: Vec::new(),
            mineral_tags_oversaturated: Vec::new(),
            refinery_tags_undersaturated: Vec::new(),
            refinery_tags_oversaturated: Vec::new(),
        };
        for (resource_tag, allocation) in self.resources.iter() {
            let workers_count = allocation.workers.len();
            match allocation.worker_role {
                WorkerRole::Mineral => {
                    if workers_count < 2 {
                        self.saturation.mineral_tags_undersaturated.push(*resource_tag);
                    } else if workers_count > 2 {
                        self.saturation.mineral_tags_oversaturated.push(*resource_tag);
                    }
                }
                WorkerRole::Gas => {
                    if workers_count < 3 {
                        self.saturation.refinery_tags_undersaturated.push(*resource_tag);
                    } else if workers_count > 3 {
                        self.saturation.refinery_tags_oversaturated.push(*resource_tag);
                    }
                }
                _ => {}
            }
        }
    }

    fn assign_resource_workers(&mut self, units: &AllUnits) {
        let workers = units.my.workers.ready().clone();
        for worker in workers {
            let worker_tag = worker.tag();
            if !self.worker_roles.contains_key(&worker_tag) {
                self.set_worker_role(worker_tag, WorkerRole::Idle);
                println!("New worker without role detected: {}", worker_tag);
            }
            let worker_role = self.worker_roles.get(&worker_tag).unwrap_or(&WorkerRole::Idle).clone();
            let gas_priority = self.get_resource_priority_gas();
            if worker_role == WorkerRole::Idle {
                if self.check_if_resource_assigned(worker_tag) {
                    continue;
                }
                if gas_priority && !self.saturation.refinery_tags_undersaturated.is_empty() {
                    self.assign_worker_to_gas(worker_tag, units);
                } else {
                    self.assign_worker_to_minerals(worker_tag, units);
                }
            } else if worker_role == WorkerRole::Mineral {
            } else if worker_role == WorkerRole::Gas {
            }
            self.update_saturation();
        }
    }

    fn check_if_resource_assigned(&mut self, worker_tag: u64) -> bool {
        for (_resource_tag, allocation) in self.resources.iter() {
            if allocation.workers.contains(&worker_tag) {
                self.set_worker_role(worker_tag, allocation.worker_role.clone());
                return true;
            }
        }
        false
    }

    fn get_resource_priority_gas(&self) -> bool {
        const GAS_PRIORITY_THRESHOLD: f32 = 2.5;
        let mineral_workers = self.worker_roles.values().filter(|&&role| role == WorkerRole::Mineral).count() as f32;
        let gas_workers = self.worker_roles.values().filter(|&&role| role == WorkerRole::Gas).count() as f32;
        if gas_workers == 0.0 {
            return mineral_workers < GAS_PRIORITY_THRESHOLD;
        }
        mineral_workers / gas_workers < GAS_PRIORITY_THRESHOLD
    }

    fn assign_worker_to_minerals(&mut self, worker_tag: u64, units: &AllUnits) {
        let undersaturated_mineral_tags = &self.saturation.mineral_tags_undersaturated;
        let mut minerals: Units = units.mineral_fields.clone();
        let worker = units.my.workers.iter().find_tag(worker_tag).unwrap().clone();

        if !undersaturated_mineral_tags.is_empty() {
            minerals = minerals.find_tags(undersaturated_mineral_tags.iter());
            if let Some(closest_mineral) = minerals.closest(worker.position()) {
                if let Some(allocation) = self.resources.get_mut(&closest_mineral.tag()) {
                    allocation.workers.push(worker_tag);
                    self.set_worker_role(worker_tag, WorkerRole::Mineral);
                    worker.gather(closest_mineral.tag(), false);
                }
            }
        } else {
            minerals.iter().sort_by_distance(worker.position());
            let mut lowest_saturated_count = usize::MAX;
            let mut lowest_saturated_tag: Option<u64> = None;
            for (resource_tag, allocation) in self.resources.iter() {
                if allocation.worker_role != WorkerRole::Mineral {
                    continue;
                }
                let workers_count = allocation.workers.len();
                if workers_count < lowest_saturated_count {
                    lowest_saturated_count = workers_count;
                    lowest_saturated_tag = Some(*resource_tag);
                }
            }
            if let Some(mineral_tag) = lowest_saturated_tag {
                if let Some(allocation) = self.resources.get_mut(&mineral_tag) {
                    allocation.workers.push(worker_tag);
                    self.set_worker_role(worker_tag, WorkerRole::Mineral);
                    worker.gather(mineral_tag, false);
                }
            }
        }
    }

    fn assign_worker_to_gas(&mut self, worker_tag: u64, units: &AllUnits) {
        let refinery_tags = &self.saturation.refinery_tags_undersaturated;
        if refinery_tags.is_empty() {
            return;
        }
        let refineries: Units = units.my.structures.find_tags(refinery_tags.iter());
        let worker = units.my.workers.iter().find_tag(worker_tag).unwrap().clone();
        if let Some(closest_refinery) = refineries.closest(worker.position()) {
            if let Some(allocation) = self.resources.get_mut(&closest_refinery.tag()) {
                allocation.workers.push(worker_tag);
                self.set_worker_role(worker_tag, WorkerRole::Gas);
                worker.gather(closest_refinery.tag(), false);
            }
        }
    }

    fn workers_movement(&self, units: &AllUnits) {
        let workers = units.my.workers.ready().clone();
        for worker in workers {
            let worker_tag = worker.tag();
            if let Some(role) = self.worker_roles.get(&worker_tag) {
                if role == &WorkerRole::Idle {
                    println!("Unassigned worker: {}", worker_tag);
                    continue;
                } else if role == &WorkerRole::Busy {
                    continue;
                } else if role == &WorkerRole::Repair {
                    self.command_repair(worker.clone(), worker_tag, units);
                } else if role == &WorkerRole::Mineral {
                    self.command_gather_mineral(worker.clone(), worker_tag, units);
                } else if role == &WorkerRole::Gas {
                    self.command_gather_gas(worker.clone(), worker_tag, units);
                }
            }
        }
    }
    fn command_repair(&self, worker: Unit, worker_tag: u64, units: &AllUnits) {
        for alloc in self.repair.values() {
            if alloc.workers.contains(&worker_tag) {
                let target_tag = alloc.tag;
                // Already repairing
                if let Some(order) = worker.order() {
                    if order.1 == Target::Tag(target_tag) {
                        return;
                    }
                }
                // Repair
                if let Some(target) = units.my.all.iter().find_tag(target_tag).clone() {
                    if target.health_percentage().unwrap_or(1.0) < 1.0 {
                        worker.repair(target.tag(), false);
                    }
                } else {
                    println!("Repair target with tag {} not found", target_tag);
                }
            }
        }
    }

    fn command_gather_mineral(&self, worker: Unit, worker_tag: u64, units: &AllUnits) {
        const CHECK_OFFSET: f32 = 0.5;
        const COMMAND_OFFSET: f32 = 0.3;
        const MINIMUM_RANGE: f32 = 2.5;

        for alloc in self.resources.values() {
            if alloc.workers.contains(&worker_tag) {
                let target_tag = alloc.resource_tag;
                if let Some(target) = units.mineral_fields.iter().find_tag(target_tag).clone() {
                    if let Some(closest_base) = units.my.townhalls.closest(worker.clone().position()) {
                        // Gather
                        if !worker.clone().is_carrying_resource() {
                            let target_distance = worker.clone().distance(target.position()) + target.radius();
                            // Mineral walk - too far
                            if target_distance > MINIMUM_RANGE {
                                // Antispam - already gathering
                                if let Some(order) = worker.order() {
                                    if order.1 == Target::Tag(target_tag) {
                                        return;
                                    }
                                }
                                worker.gather(target_tag, false);
                            }
                            // Get to position
                            else if target_distance > CHECK_OFFSET {
                                let offset = target.radius() + COMMAND_OFFSET;
                                let mineral_offset_position = target.position().towards(worker.clone().position(), offset);
                                // Antispam - already moving close
                                if let Some(order) = worker.order() {
                                    if let Target::Pos(pos) = order.1 {
                                        if pos.distance(mineral_offset_position) < COMMAND_OFFSET {
                                            return;
                                        }
                                    }
                                }
                                worker.move_to(Target::Pos(mineral_offset_position), false);
                            // Gather
                            } else {
                                // Antispam - already gathering
                                if let Some(order) = worker.order() {
                                    if order.1 == Target::Tag(target_tag) {
                                        return;
                                    }
                                }
                                worker.gather(target_tag, false);
                            }
                        } else {
                            let return_distance = worker.clone().distance(closest_base.position()) + closest_base.radius();
                            // Too far
                            if return_distance > MINIMUM_RANGE {
                                // Antispam - already returning
                                if let Some(order) = worker.order() {
                                    if order.1 == Target::Tag(closest_base.tag()) {
                                        return;
                                    }
                                }
                                worker.smart(Target::Tag(closest_base.tag()), false);
                            // Get to position
                            } else if return_distance > CHECK_OFFSET {
                                let offset = closest_base.radius() + COMMAND_OFFSET;
                                let base_offset_position = closest_base.position().towards(worker.clone().position(), offset);
                                // Antispam - already moving close
                                if let Some(order) = worker.order() {
                                    if let Target::Pos(pos) = order.1 {
                                        if pos.distance(base_offset_position) < COMMAND_OFFSET {
                                            return;
                                        }
                                    }
                                }
                                worker.move_to(Target::Pos(base_offset_position), false);
                            // Return
                            } else {
                                // Antispam - already returning
                                if let Some(order) = worker.order() {
                                    if order.1 == Target::Tag(closest_base.tag()) {
                                        return;
                                    }
                                }
                                worker.smart(Target::Tag(closest_base.tag()), false);
                            }
                        }
                    } else {
                        println!("No base found for worker {}", worker_tag);
                    }
                } else {
                    println!("Mineral with tag {} not found", target_tag);
                }
                return;
            }
        }
        println!("Worker {} not assigned to any mineral", worker_tag);
    }

    fn command_gather_gas(&self, worker: Unit, worker_tag: u64, units: &AllUnits) {
        for alloc in self.resources.values() {
            if alloc.workers.contains(&worker_tag) {
                let target_tag = alloc.resource_tag;
                let target = units.my.structures.iter().find_tag(target_tag).unwrap().clone();
                let closest_base = units.my.townhalls.closest(worker.clone().position());
                if let Some(closest_base) = closest_base {
                    if worker.clone().is_carrying_resource() {
                        // Antispam - already returning
                        if let Some(order) = worker.order() {
                            if order.1 == Target::Tag(closest_base.tag()) {
                                return;
                            }
                        }
                        worker.smart(Target::Tag(closest_base.tag()), false);
                    } else {
                        // Antispam - already gathering
                        if let Some(order) = worker.order() {
                            if order.1 == Target::Tag(target_tag) {
                                return;
                            }
                        }
                        worker.gather(target.tag(), false);
                    }
                } else {
                    println!("No base found for worker {}", worker_tag);
                }
                return;
            }
        }
        println!("Worker {} not assigned to any refinery", worker_tag);
    }

    pub fn get_closest_worker(&mut self, units: &AllUnits, position: Point2) -> Option<u64> {
        let mut closest_worker = self.get_closest_worker_by_role(units, position, WorkerRole::Idle, &self.worker_roles.clone());
        if let Some(tag) = closest_worker {
            self.set_worker_role(tag, WorkerRole::Busy);
            return Some(tag);
        }
        closest_worker = self.get_closest_worker_by_role(units, position, WorkerRole::Mineral, &self.worker_roles.clone());
        if let Some(tag) = closest_worker {
            self.set_worker_role(tag, WorkerRole::Busy);
            return Some(tag);
        }
        closest_worker = self.get_closest_worker_by_role(units, position, WorkerRole::Gas, &self.worker_roles.clone());
        if let Some(tag) = closest_worker {
            self.set_worker_role(tag, WorkerRole::Busy);
            return Some(tag);
        }
        None
    }

    fn get_closest_worker_by_role(
        &self, units: &AllUnits, position: Point2, role: WorkerRole, roles: &HashMap<u64, WorkerRole>
    ) -> Option<u64> {
        let workers = &units.my.workers;
        let mut closest_worker_tag: Option<u64> = None;
        let mut closest_distance = f32::MAX;

        for worker in workers {
            if let Some(worker_role) = roles.get(&worker.tag()) {
                if *worker_role == role {
                    let distance = worker.position().distance(position);
                    if distance < closest_distance {
                        closest_distance = distance;
                        closest_worker_tag = Some(worker.tag());
                    }
                }
            }
        }
        closest_worker_tag
    }
}

// Helpers
fn counter_worker_rush(_bot: &mut Nikolaj) {
    return;
}

fn counter_ramp_blocking(_bot: &mut Nikolaj) {
    return;
}

fn get_mining_bases(units: &AllUnits) -> Vec<u64> {
    let mut bases = Vec::new();
    for base in units.my.townhalls.ready().clone() {
        if base.is_flying() {
            continue;
        }
        bases.push(base.tag());
    }
    bases
}
fn collect_valid_resource_tags(units: &AllUnits) -> (HashSet<u64>, HashSet<u64>) {
    const GATHER_RADIUS: f32 = 15.0;
    let mut minerals = HashSet::new();
    let mut refineries = HashSet::new();

    for base in units.my.townhalls.ready().clone() {
        let pos = base.position();

        for mf in units.mineral_fields.closer(GATHER_RADIUS, pos) {
            minerals.insert(mf.tag());
        }

        for rf in units
            .my
            .structures
            .of_type_including_alias(UnitTypeId::Refinery)
            .closer(GATHER_RADIUS, pos)
            .ready()
        {
            if rf.vespene_contents().unwrap_or(0) > 0 {
                refineries.insert(rf.tag());
            }
        }
    }
    (minerals, refineries)
}

fn collect_damaged_targets(units: &AllUnits, bases_tags: std::slice::Iter<u64>) -> HashMap<u64, RepairAllocation> {
    let mut damaged_targets = HashMap::new();
    let bases = units.my.structures.find_tags(bases_tags);

    for structure in units.my.structures.ready().clone() {
        let health_percentage = structure.health_percentage().unwrap_or(1.0);
        if health_percentage < 0.8 {
            let max_workers = match structure.type_id() {
                UnitTypeId::SupplyDepot
                | UnitTypeId::SupplyDepotLowered => 2,
                UnitTypeId::Bunker
                | UnitTypeId::MissileTurret
                | UnitTypeId::CommandCenter
                | UnitTypeId::OrbitalCommand => 3,
                UnitTypeId::PlanetaryFortress => 5,
                _ => 1,
            };
            damaged_targets.insert(
                structure.tag(),
                RepairAllocation {
                    tag: structure.tag(),
                    is_structure: true,
                    workers: Vec::new(),
                    max_workers,
                },
            );
        }
    }
    for unit in units.my.units.ready().clone() {
        let unit_type = unit.type_id();
        if !(unit_type == UnitTypeId::Thor
            || unit_type == UnitTypeId::ThorAALance
            || unit_type == UnitTypeId::ThorAAWeapon
            || unit_type == UnitTypeId::Banshee
            )
        {
            continue;
        }
        let health_percentage = unit.health_percentage().unwrap_or(1.0);
        let closest_base_distance = bases.closest_distance(unit.position()).unwrap_or(0.0);
        if health_percentage < 0.5 && closest_base_distance < 20.0 {
            damaged_targets.insert(
                unit.tag(),
                RepairAllocation {
                    tag: unit.tag(),
                    is_structure: false,
                    workers: Vec::new(),
                    max_workers: 1,
                },
            );
        }
    }
    damaged_targets
}
