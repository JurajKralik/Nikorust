use rust_sc2::prelude::*;


#[derive(Clone)]
pub struct UnitSnapshot {
    pub id: u64,
    pub type_id: UnitTypeId,
    pub position: Point2,
    pub health: f32,
    pub supply: usize,
    pub last_seen: f32,
    pub is_snapshot: bool,
}

impl UnitSnapshot {
    pub fn from_unit(unit: &Unit) -> Self {
        let health = (unit.health() + unit.shield()) as f32;
        let supply = unit.supply_cost() as usize;
        UnitSnapshot {
            id: unit.tag(),
            type_id: unit.type_id(),
            position: unit.position(),
            health,
            supply,
            last_seen: 0.0,
            is_snapshot: false,
        }
    }
}