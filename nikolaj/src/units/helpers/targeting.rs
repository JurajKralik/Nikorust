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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriorityLevel {
    Ignore,
    Low,
    Medium,
    High,
    VeryHigh,
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
}

pub fn get_targeting_priorities(
    unit_type: &UnitTypeId) -> TargetingPriorities {
    TARGETING_PRIORITIES.get(unit_type)
}