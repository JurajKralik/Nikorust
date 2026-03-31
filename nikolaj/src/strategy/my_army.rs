use crate::Nikolaj;
use rust_sc2::prelude::*;

use crate::strategy::unit_snapshot::UnitSnapshot;


// Purpose : Checking health changes for detection of cloaked units
#[derive(Default, Clone)]
pub struct MyArmySnapshot {
    pub debugging: bool,
    pub units: Vec<UnitSnapshot>
}

impl MyArmySnapshot {
    fn delete_missing_units(&mut self, current_units: Units) {
        let mut ids_to_remove = Vec::new();
        for snapshot_unit in self.units.iter() {
            if !current_units.iter().any(|my_unit| my_unit.tag() == snapshot_unit.tag()) {
                if self.debugging {
                    println!("My unit died: {:?}", snapshot_unit.unit.type_id());
                }
                ids_to_remove.push(snapshot_unit.tag());
            }
        }
        self.units.retain(|unit| !ids_to_remove.contains(&unit.tag()));
    }

    fn append_snapshot(&mut self, current_units: Vec<UnitSnapshot>) {
        for unit in current_units {
            if let Some(existing) = self.units.iter_mut().find(|u| u.tag() == unit.tag()) {
                existing.unit = unit.unit.clone();
                existing.last_seen = unit.last_seen;
            } else {
                if self.debugging {
                    println!("New my unit: {:?}", unit.unit.type_id());
                }
                self.units.push(unit.clone());
            }
        }
    }

    fn detect_health_changes_on_cloaked_units(&mut self, my_units: Units, enemy_units: Units) -> bool {
        for unit in my_units.iter() {
            if !unit.is_cloaked() {
                continue;
            };
            if let Some(snapshot_unit) = self.units.iter().find(|u| u.tag() == unit.tag()) {
                let current_health = (unit.health() + unit.shield()) as f32;
                let health_changed = (current_health - snapshot_unit.health()).abs() > f32::EPSILON;
                if !health_changed {
                    continue;
                }
                for enemy_unit in enemy_units.iter().closer(15.0, unit.position()) {
                    if enemy_unit.is_detector() {
                        continue;
                    }
                }
                if self.debugging {
                    println!("Health change detected on cloaked unit: {:?}", snapshot_unit.unit.type_id());
                }
                return true;
            }
        }
        false
    }
    
    fn update_health(&mut self, my_units: Units) {
        for snapshot_unit in self.units.iter_mut() {
            if let Some(current_unit) = my_units.iter().find(|my_unit| my_unit.tag() == snapshot_unit.tag()) {
                snapshot_unit.unit = current_unit.clone();
            }
        }
    }
}


pub fn refresh_my_army_snapshot(bot: &mut Nikolaj) {
    let my_units = bot.units.my.units.clone();
    let enemy_units = bot.units.enemy.units.clone();
    let current_time = bot.time;
    let my_army_snapshot = &mut bot.strategy_data.my_army;

    my_army_snapshot.debugging = bot.debugger.printing_my_army_changes;

    my_army_snapshot.delete_missing_units(my_units.clone());
    my_army_snapshot.append_snapshot(my_units.clone().into_iter().map(|unit| UnitSnapshot::from_unit(unit, current_time)).collect());
    let detected = my_army_snapshot.detect_health_changes_on_cloaked_units(my_units.clone(), enemy_units);
    if detected {
        bot.combat_info.detected = true;
        bot.combat_info.detected_at = current_time;
    }
    my_army_snapshot.update_health(my_units);

}
