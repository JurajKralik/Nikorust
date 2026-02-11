use rust_sc2::prelude::*;

use crate::consts::UNITS_PRIORITY_IGNORE;


#[derive(Clone)]
pub struct UnitSnapshot {
    pub unit: Unit,
    pub last_seen: f32,
    pub is_snapshot: bool,
    pub is_combat_relevant: bool,
    pub is_combat_unit: bool,
}

impl UnitSnapshot {
    pub fn from_unit(unit: Unit, last_seen: f32) -> Self {
        let is_combat_unit = !UNITS_PRIORITY_IGNORE.contains(&unit.type_id());
        UnitSnapshot {
            unit,
            last_seen,
            is_snapshot: false,
            is_combat_relevant: true,
            is_combat_unit
        }
    }
    
    pub fn id(&self) -> u64 {
        self.unit.tag()
    }
    
    pub fn type_id(&self) -> UnitTypeId {
        self.unit.type_id()
    }
        
    pub fn health(&self) -> f32 {
        (self.unit.health() + self.unit.shield()) as f32
    }
    
    pub fn position(&self) -> Point2 {
        self.unit.position()
    }

    pub fn supply(&self) -> usize {
        self.unit.supply_cost() as usize
    }
}