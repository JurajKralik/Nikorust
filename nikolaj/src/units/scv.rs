use rust_sc2::prelude::*;
use std::collections::{HashMap, HashSet};
use rust_sc2::units::AllUnits;
use crate::Nikolaj;


pub fn scv_step(bot: &mut Nikolaj) {
    let (valid_minerals, valid_refineries) = {
        collect_valid_resource_tags(&bot.units)
    };
    bot.worker_allocator.update_resources_from_tags(valid_minerals, valid_refineries);
}

#[derive(Debug, Default)]
pub struct WorkerAllocator {
    pub repair: HashMap<u64, RepairAllocation>,
    pub resources: HashMap<u64, ResourceAllocation>,
    pub worker_roles: HashMap<u64, WorkerRole>,
}

impl WorkerAllocator {
    pub fn new() -> Self {
        Self {
            repair: HashMap::new(),
            resources: HashMap::new(),
            worker_roles: HashMap::new(),
        }
    }
    pub fn update_resources_from_tags(
        &mut self,
        valid_minerals: HashSet<u64>,
        valid_refineries: HashSet<u64>,
    ) {
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

    pub fn assign_repair(&mut self, structure: &Unit, worker: &Unit, max: usize) {
        let entry = self.repair.entry(structure.tag()).or_insert(RepairAllocation {
            structure_tag: structure.tag(),
            workers: Vec::new(),
            max_workers: max,
        });

        if entry.workers.len() < entry.max_workers {
            entry.workers.push(worker.tag());
            self.worker_roles.insert(worker.tag(), WorkerRole::Repair);
        }
    }

    pub fn assign_resource(&mut self, resource: &Unit, worker: &Unit) {
        let entry = self.resources.entry(resource.tag()).or_insert(ResourceAllocation {
            resource_tag: resource.tag(),
            worker_role: if resource.type_id() == UnitTypeId::Refinery || resource.type_id() == UnitTypeId::RefineryRich {
                WorkerRole::Gas
            } else {
                WorkerRole::Mineral
            },
            workers: Vec::new(),
        });

        entry.workers.push(worker.tag());
        self.worker_roles.insert(worker.tag(), if resource.type_id() == UnitTypeId::Refinery || resource.type_id() == UnitTypeId::RefineryRich {
            WorkerRole::Gas
        } else {
            WorkerRole::Mineral
        });
    }

    pub fn free_worker(&mut self, worker_tag: u64) {
        self.worker_roles.remove(&worker_tag);

        for alloc in self.repair.values_mut() {
            alloc.workers.retain(|&w| w != worker_tag);
        }

        for alloc in self.resources.values_mut() {
            alloc.workers.retain(|&w| w != worker_tag);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerRole {
    Mineral,
    Gas,
    Repair,
    Idle,
}

#[derive(Debug)]
pub struct RepairAllocation {
    pub structure_tag: u64,
    pub workers: Vec<u64>,
    pub max_workers: usize,
}

#[derive(Debug)]
pub struct ResourceAllocation {
    pub resource_tag: u64,
    pub worker_role: WorkerRole,
    pub workers: Vec<u64>,
}

impl ResourceAllocation {
    pub fn worker_count(&self) -> usize {
        self.workers.len()
    }
    pub fn is_saturated(&self) -> bool {
        match self.worker_role {
            WorkerRole::Mineral => self.workers.len() >= 2,
            WorkerRole::Gas => self.workers.len() >= 3,
            _ => false,
        }
    }
}

// Helpers
fn collect_valid_resource_tags(units: &AllUnits) -> (HashSet<u64>, HashSet<u64>) {
    const GATHER_RADIUS: f32 = 13.0;
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