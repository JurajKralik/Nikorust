#![allow(dead_code)]

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreatLevel {
    None,
    Danger,
    Flee
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
}

pub fn get_threat_levels(
    unit_type: &UnitTypeId) -> ThreatLevels {
    THREAT_LEVELS.get(unit_type).clone()
}