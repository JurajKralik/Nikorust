use crate::Nikolaj;
use rust_sc2::prelude::*;
use rust_sc2::units::AllUnits;
use std::collections::{HashMap, HashSet};


pub fn scv_step(bot: &mut Nikolaj) {
    bot.worker_allocator.bases = get_mining_bases(&bot.units);
    bot.worker_allocator.update_resources(collect_valid_resource_tags(&bot.units));
}

#[derive(Debug, Default)]
pub struct WorkerAllocator {
    pub bases: Vec<u64>,
    pub repair: HashMap<u64, RepairAllocation>,
    pub resources: HashMap<u64, ResourceAllocation>,
    pub worker_roles: HashMap<u64, WorkerRole>,
}

impl WorkerAllocator {
    pub fn update_resources(
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