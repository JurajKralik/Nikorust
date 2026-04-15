use crate::Nikolaj;
use rust_sc2::prelude::*;

use crate::strategy::unit_snapshot::UnitSnapshot;


const FADE_OUT_TIME: f32 = 45.0;
const COMBAT_RELEVANT_TIME: f32 = 8.0;

#[derive(Default, Clone)]
pub struct EnemyArmySnapshot {
    pub debugging: bool,
    pub units: Vec<UnitSnapshot>
}

impl EnemyArmySnapshot {
    pub fn get_supply_by_type(&self, unit_type: UnitTypeId) -> usize {
        self.units.iter().filter(|unit| unit.type_id() == unit_type).map(|unit| unit.supply()).sum()
    }

    pub fn get_army_supply(&self) -> usize {
        self.units.iter().filter(|unit| !unit.is_ignored_unit).map(|unit| unit.supply()).sum()
    }

    fn mark_all_as_snapshot(&mut self) {
        for unit in self.units.iter_mut() {
            unit.is_snapshot = true;
        }
    }

    fn append_snapshot(&mut self, enemies: Vec<UnitSnapshot>) {
        for unit in enemies {
            if let Some(existing) = self.units.iter_mut().find(|e| e.tag() == unit.tag()) {
                existing.unit = unit.unit.clone();
                existing.last_seen = unit.last_seen;
                existing.is_snapshot = false;
            } else {
                if self.debugging {
                    println!("New enemy unit spotted: {:?}", unit.unit.type_id());
                }
                self.units.push(unit.clone());
            }
        }
    }

    fn delete_outdated_enemy_snapshots(&mut self, current_time: f32) {
        let mut outdated_units = Vec::new();
        for snapshot in self.units.iter_mut() {
            if current_time - snapshot.last_seen <= FADE_OUT_TIME {
                continue;
            }
            if self.debugging {
                println!("Enemy unit faded out: {:?}", snapshot.unit.type_id());
            }
            outdated_units.push(snapshot.tag());
        }
        self.units.retain(|snapshot| !outdated_units.contains(&snapshot.tag()));
    }

    fn delete_dead_enemy_snapshots(&mut self, dead_units: Vec<u64>) {
        if self.debugging {
            for dead_unit in &dead_units {
                println!("Enemy unit died: {:?}", dead_unit);
            }
        }
        self.units.retain(|snapshot| !dead_units.contains(&snapshot.tag()));
    }

    fn update_combat_relevance(&mut self, current_time: f32) {
        for snapshot in self.units.iter_mut() {
            if snapshot.is_snapshot && (current_time - snapshot.last_seen) > COMBAT_RELEVANT_TIME {
                snapshot.is_position_still_relevant = false;
            }
        }
    }

}

pub fn refresh_enemy_army_snapshot(bot: &mut Nikolaj) {
    let current_time = bot.time.clone();
    let dead_units = bot.state.observation.raw.dead_units.clone();
    let currently_visible_enemies = bot.units.enemy.units.clone().iter().map(|unit| UnitSnapshot::from_unit(unit.clone(), current_time)).collect();
    let enemy_army_snapshot = &mut bot.strategy_data.enemy_army;

    enemy_army_snapshot.debugging = bot.debugger.printing_enemy_army_changes;

    enemy_army_snapshot.mark_all_as_snapshot();
    enemy_army_snapshot.append_snapshot(currently_visible_enemies);
    enemy_army_snapshot.delete_outdated_enemy_snapshots(current_time);
    enemy_army_snapshot.delete_dead_enemy_snapshots(dead_units);
    enemy_army_snapshot.update_combat_relevance(current_time);
}

