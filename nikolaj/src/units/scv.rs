use crate::Nikolaj;
use rust_sc2::{prelude::*, units};
use rust_sc2::units::AllUnits;
use std::collections::{HashMap, HashSet};


pub fn scv_step(bot: &mut Nikolaj) {
    bot.worker_allocator.bases = get_mining_bases(&bot.units);
    bot.worker_allocator.update_resources(collect_valid_resource_tags(&bot.units));
    bot.worker_allocator.update_repair_targets(&bot.units.clone(), collect_damaged_targets(&bot.units));
    bot.worker_allocator.assign_repairmen(&bot.units.clone());
}

#[derive(Debug, Default)]
pub struct WorkerAllocator {
    pub bases: Vec<u64>,
    pub repair: HashMap<u64, RepairAllocation>,
    pub resources: HashMap<u64, ResourceAllocation>,
    pub worker_roles: HashMap<u64, WorkerRole>,
}

impl WorkerAllocator {
    fn update_resources(
        &mut self,
        valid_resources: (HashSet<u64>, HashSet<u64>),
    ) {
        let (valid_minerals, valid_refineries) = valid_resources;
        let valid_all: HashSet<u64> = valid_minerals.union(&valid_refineries).cloned().collect();

        self.resources.retain(|tag, _| valid_all.contains(tag));

        for tag in valid_minerals {
            self.resources.entry(tag).or_insert(ResourceAllocation {
                resource_tag: tag,
                worker_role: WorkerRole::Mineral,
                workers: Vec::new(),
            });
        }

        for tag in valid_refineries {
            self.resources.entry(tag).or_insert(ResourceAllocation {
                resource_tag: tag,
                worker_role: WorkerRole::Gas,
                workers: Vec::new(),
            });
        }
    }
    fn update_repair_targets(
        &mut self,
        units: &AllUnits,
        damaged_targets: HashMap<u64, RepairAllocation>,
    ) {
        let valid_tags: HashSet<u64> = damaged_targets.keys().cloned().collect();

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

fn collect_damaged_targets(units: &AllUnits) -> HashMap<u64, RepairAllocation> {
    let mut damaged_targets = HashMap::new();
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
        if health_percentage < 0.5 {
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
