use crate::Nikolaj;
use rust_sc2::prelude::*;
use rust_sc2::units::AllUnits;
use std::collections::{HashMap, HashSet};


pub fn scv_step(bot: &mut Nikolaj) {
    // Worker rush
    if bot.enemy_worker_rush {
        counter_worker_rush(bot); // TODO: implement
        return;
    }

    // Bases
    bot.worker_allocator.bases = get_mining_bases(&bot.units);
    
    // Repair
    let bases_tags = bot.worker_allocator.bases.iter().clone();
    let damaged_targets = collect_damaged_targets(&bot.units, bases_tags);
    bot.worker_allocator.update_repair_targets(&bot.units.clone(), damaged_targets);
    bot.worker_allocator.assign_repairmen(&bot.units.clone());

    // Ramp blocking
    if bot.enemy_ramp_blocking {
        counter_ramp_blocking(bot); // TODO: implement
    }

    // Resources
    bot.worker_allocator.update_idle_workers(&bot.units.clone());
    let valid_resources_tags = collect_valid_resource_tags(&bot.units.clone());
    bot.worker_allocator.update_resources(valid_resources_tags, &bot.units.clone());

}

#[derive(Debug, Default)]
pub struct WorkerAllocator {
    pub bases: Vec<u64>,
    pub repair: HashMap<u64, RepairAllocation>,
    pub resources: HashMap<u64, ResourceAllocation>,
    pub worker_roles: HashMap<u64, WorkerRole>,
}

impl WorkerAllocator {
    fn update_repair_targets(
        &mut self,
        units: &AllUnits,
        damaged_targets: HashMap<u64, RepairAllocation>,
    ) {
        let valid_tags: HashSet<u64> = damaged_targets.keys().cloned().collect();
        let bases_tags = self.bases.iter().clone();
        let bases = units.my.structures.find_tags(bases_tags);

        // Check existing repair targets
        let mut invalid_tags: Vec<u64> = Vec::new();
        for (tag, alloc) in self.repair.iter_mut() {
            if valid_tags.contains(tag) {
                continue;
            }
            let target: Unit;
            if alloc.is_structure {
                target = units.my.structures.iter().find_tag(tag.clone()).unwrap().clone();
            } else {
                target = units.my.units.iter().find_tag(tag.clone()).unwrap().clone();
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
                if self.worker_roles.contains_key(&worker_tag) {
                    self.worker_roles.insert(worker_tag, WorkerRole::Idle);
                }
            }
        }
        for tag in invalid_tags {
            self.repair.remove(&tag);
        }
        // Add new repair targets
        for (tag, alloc) in damaged_targets {
            if self.repair.contains_key(&tag) {
                continue;
            }
            self.repair.insert(tag, alloc);
        }
    }
    fn assign_repairmen(&mut self, units: &AllUnits) {
        let workers = units.my.workers.ready().clone();
        let worker_tags = workers.iter().map(|w| w.tag()).collect::<HashSet<u64>>();
        for (tag, alloc) in self.repair.iter_mut() {
            let mut current_workers = alloc.workers.clone();
            let max_workers = alloc.max_workers;
            let target: Unit;
            if alloc.is_structure {
                target = units.my.structures.iter().find_tag(tag.clone()).unwrap().clone();
            } else {
                target = units.my.units.iter().find_tag(tag.clone()).unwrap().clone();
            }
            // Check if workers are still valid
            for worker in current_workers.clone() {
                if !worker_tags.contains(&worker) {
                    current_workers.retain(|w| *w != worker);
                }
            }
            // Assign new workers
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
                    if self.worker_roles.contains_key(&worker_tag) {
                        let worker_role = self.worker_roles.get(&worker_tag).unwrap_or(&WorkerRole::Idle).clone();
                        if worker_role != WorkerRole::Busy && worker_role != WorkerRole::Repair {
                            continue;
                        }
                    }
                    current_workers.push(worker_tag);
                }
            }
            alloc.workers = current_workers.clone();
            // Send repair commands
            for worker_tag in current_workers {
                let worker = units.my.workers.iter().find_tag(worker_tag).unwrap().clone();
                if worker.is_repairing() {
                    continue;
                }
                self.worker_roles.insert(worker_tag, WorkerRole::Repair);
                worker.repair(tag.clone(), false);
            }
        }
    }
    fn update_idle_workers(&mut self, units: &AllUnits) {
        for worker in units.my.workers.ready().clone() {
            let worker_tag = worker.tag();
            if !self.worker_roles.contains_key(&worker_tag) {
                self.worker_roles.insert(worker_tag, WorkerRole::Idle);
            }
        }
    }
    fn update_resources(
        &mut self,
        valid_minerals_and_refineries: (HashSet<u64>, HashSet<u64>),
        units: &AllUnits
    ) {
        let (valid_minerals, valid_refineries) = valid_minerals_and_refineries;
        let valid_resources: HashSet<u64> = valid_minerals.union(&valid_refineries).cloned().collect();
        self.remove_invalid_resources(&valid_resources.clone(), units);
        self.add_new_resources(&valid_minerals, &valid_refineries);
    }
    fn remove_invalid_resources(&mut self, valid_resources: &HashSet<u64>, units: &AllUnits) {
        let worker_tags = units.my.workers.iter().map(|w| w.tag()).collect::<HashSet<u64>>();
        let mut invalid_resources_tags: Vec<u64> = Vec::new();
        for (tag, alloc) in self.resources.iter_mut() {
            if !valid_resources.contains(tag) {
                invalid_resources_tags.push(*tag);
                for worker_tag in alloc.workers.clone() {
                    if !worker_tags.contains(&worker_tag) {
                        if self.worker_roles.contains_key(&worker_tag) {
                            self.worker_roles.remove(&worker_tag);
                        }
                    } else {
                        self.worker_roles.insert(worker_tag, WorkerRole::Idle);
                    }
                }
            }
        }
        for tag in invalid_resources_tags {
            self.resources.remove(&tag);
        }
    }
    fn add_new_resources(&mut self, valid_minerals: &HashSet<u64>, valid_refineries: &HashSet<u64>) {
        for tag in valid_minerals {
            self.resources.insert(*tag, ResourceAllocation {
                resource_tag: *tag,
                worker_role: WorkerRole::Mineral,
                workers: Vec::new(),
            });
        }
        for tag in valid_refineries {
            self.resources.insert(*tag, ResourceAllocation {
                resource_tag: *tag,
                worker_role: WorkerRole::Gas,
                workers: Vec::new(),
            });
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerRole {
    Mineral,
    Gas,
    Repair,
    Busy,
    Idle
}

#[derive(Debug)]
pub struct RepairAllocation {
    pub tag: u64,
    pub is_structure: bool,
    pub workers: Vec<u64>,
    pub max_workers: usize,
}

#[derive(Debug)]
pub struct ResourceAllocation {
    pub resource_tag: u64,
    pub worker_role: WorkerRole,
    pub workers: Vec<u64>,
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
                | UnitTypeId::MissileTurret => 3,
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
