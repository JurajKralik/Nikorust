use rust_sc2::prelude::*;
use std::collections::HashMap;

use crate::consts::*;

pub struct ThreatLevelsList {
    pub list: HashMap<UnitTypeId, ThreatLevels>,
}

#[derive(Clone)]
pub struct ThreatLevels {
    pub threats: Vec<ThreatLevelInfo>,
}

#[derive(Clone)]
pub struct ThreatLevelInfo {
    pub unit_type: UnitTypeId,
    pub threat_level: ThreatLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ThreatLevel {
    None = 0,
    Danger = 1,
    Countered = 2,
    Flee = 3
}

impl ThreatLevelsList {
    pub fn get(&self, unit_type: &UnitTypeId) -> ThreatLevels {
        if let Some(levels) = self.list.get(unit_type) {
            return levels.clone();
        }
        ThreatLevels::default()
    }
}

impl Default for ThreatLevels {
    fn default() -> Self {
        ThreatLevels {
            threats: vec![],
        }
    }
}

impl ThreatLevels {
    pub fn get_threat_level(&self, unit_type: &UnitTypeId) -> ThreatLevel {
        let mut max_threat = ThreatLevel::None;
        
        for threat_info in &self.threats {
            if &threat_info.unit_type == unit_type {
                if threat_info.threat_level > max_threat {
                    max_threat = threat_info.threat_level;
                }
            }
        }

        max_threat
    }

    pub fn get_higher_threat_unit(&self, current_unit: Option<Unit>, new_unit: Unit) -> Option<Unit> {
        let new_threat_level = self.get_threat_level(&new_unit.type_id());

        if new_threat_level == ThreatLevel::None {
            return current_unit;
        }

        if let Some(current_unit) = current_unit {
            let current_threat_level = self.get_threat_level(&current_unit.type_id());
            if new_threat_level > current_threat_level {
                return Some(new_unit);
            }
            return Some(current_unit);
        }
        Some(new_unit)
    }
}

pub fn get_threat_levels(
    unit_type: &UnitTypeId) -> ThreatLevels {
    THREAT_LEVELS.get(unit_type).clone()
}