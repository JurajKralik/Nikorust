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
    Flee = 2
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
        for threat_info in &self.threats {
            if &threat_info.unit_type == unit_type {
                return threat_info.threat_level;
            }
        }

        ThreatLevel::None
    }
    pub fn compare_threat_levels(&self, unit_a: Option<Unit>, unit_b: Unit) -> Option<Unit> {
        let threat_level_b = self.get_threat_level(&unit_b.type_id());
        if threat_level_b == ThreatLevel::None {
            return unit_a;
        }
        if let Some(unit_a) = unit_a {
            let threat_level_a = self.get_threat_level(&unit_a.type_id());
            if threat_level_a == threat_level_b {
                return None;
            } else if threat_level_a > threat_level_b {
                return Some(unit_a);
            }
            return Some(unit_a);
        }
        None
    }
}

pub fn get_threat_levels(
    unit_type: &UnitTypeId) -> ThreatLevels {
    THREAT_LEVELS.get(unit_type).clone()
}