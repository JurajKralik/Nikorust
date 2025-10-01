#![allow(dead_code)]

use rust_sc2::prelude::*;
use std::collections::HashMap;

use crate::consts::*;

pub struct TargetingPrioritiesList {
    pub list: HashMap<UnitTypeId, TargetingPriorities>,
}

#[derive(Clone)]
pub struct TargetingPriorities {
    pub priorities: Vec<TargetPriorityInfo>,
}

#[derive(Clone)]
pub struct TargetPriorityInfo {
    pub unit_type: UnitTypeId,
    pub priority_level: PriorityLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PriorityLevel {
    Ignore = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    VeryHigh = 4,
    Max = 5,
}

impl TargetingPrioritiesList {
    pub fn get(&self, unit_type: &UnitTypeId) -> TargetingPriorities {
        if let Some(priorities) = self.list.get(unit_type) {
            return priorities.clone();
        }
        TargetingPriorities::default()
    }
}

impl Default for TargetingPriorities {
    fn default() -> Self {
        TargetingPriorities {
            priorities: vec![],
        }
    }
}

impl TargetingPriorities {
    pub fn get_priority_level(&self, unit_type: &UnitTypeId) -> PriorityLevel {
        if IGNORE_UNITS.contains(unit_type) {
            return PriorityLevel::Ignore;
        }

        for priority_info in &self.priorities {
            if &priority_info.unit_type == unit_type {
                return priority_info.priority_level;
            }
        }

        PriorityLevel::Medium
    }
    pub fn compare_priority(&self, unit_a: Unit, unit_b: Unit) -> Option<u64> {
        let priority_a = self.get_priority_level(&unit_a.type_id());
        let priority_b = self.get_priority_level(&unit_b.type_id());
        if priority_a == priority_b {
            return None;
        } else if priority_a > priority_b {
            return Some(unit_a.tag());
        }
        Some(unit_b.tag())
    }
}

pub fn get_targeting_priorities(
    unit_type: &UnitTypeId) -> TargetingPriorities {
    TARGETING_PRIORITIES.get(unit_type)
}