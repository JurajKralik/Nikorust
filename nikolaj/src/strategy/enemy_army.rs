use crate::Nikolaj;
use rust_sc2::prelude::*;

use crate::strategy::unit_snapshot::UnitSnapshot;

#[derive(Default, Clone)]
pub struct EnemyArmySnapshot {
    pub units: Vec<UnitSnapshot>
}

impl EnemyArmySnapshot {
    pub fn get_supply_by_type(&self, unit_type: UnitTypeId) -> usize {
        self.units.iter().filter(|unit| unit.type_id == unit_type).map(|unit| unit.supply).sum()
    }
    pub fn get_army_supply(&self) -> usize {
        self.units.iter().map(|unit| unit.supply).sum()
    }
}

pub fn refresh_enemy_army_snapshot(bot: &mut Nikolaj) {
    let visible_enemies = get_visible_enemies(bot);
    let appended_snapshot = get_appended_enemy_army_snapshot(bot, visible_enemies);
    bot.strategy_data.enemy_army.units = appended_snapshot;
    delete_outdated_enemy_snapshots(bot);
    let dead_units = bot.state.observation.raw.dead_units.clone();
    delete_dead_enemy_snapshots(bot, dead_units);
}
fn get_visible_enemies(bot: &Nikolaj) -> Vec<UnitSnapshot> {
    let enemies = bot.units.enemy.units.clone();
    let mut currently_visible_army: Vec<UnitSnapshot> = Vec::new();
    for enemy in enemies {
        if !enemy.can_attack() || enemy.type_id().is_worker() {
            continue;
        }
        let health = (enemy.health() + enemy.shield()) as f32;
        let supply = enemy.supply_cost() as usize;
        let snapshot = UnitSnapshot {
            id: enemy.tag(),
            type_id: enemy.type_id(),
            position: enemy.position(),
            health: health,
            supply: supply,
            last_seen: bot.time,
            is_snapshot: false,
        };
        currently_visible_army.push(snapshot);
    }
    currently_visible_army
}
fn get_appended_enemy_army_snapshot(
    bot: &mut Nikolaj,
    visible_enemies: Vec<UnitSnapshot>,
) -> Vec<UnitSnapshot> {
    let mut current_snapshot: Vec<UnitSnapshot> = bot.strategy_data.enemy_army.units.clone();
    // Mark all as snapshot
    for current_enemy in current_snapshot.iter_mut() {
        current_enemy.is_snapshot = true;
    }
    // Update or add
    for visible in visible_enemies {
        if let Some(existing) = current_snapshot.iter_mut().find(|e| e.id == visible.id) {
            existing.position = visible.position;
            existing.health = visible.health;
            existing.last_seen = bot.time;
            existing.supply = visible.supply;
        } else {
            current_snapshot.push(visible.clone());
        }
    }
    current_snapshot
}
fn delete_outdated_enemy_snapshots(bot: &mut Nikolaj) {
    let current_time = bot.time;
    bot.strategy_data
        .enemy_army
        .units
        .retain(|snapshot| current_time - snapshot.last_seen <= 120.0);
}
fn delete_dead_enemy_snapshots(bot: &mut Nikolaj, dead_units: Vec<u64>) {
    bot.strategy_data
        .enemy_army
        .units
        .retain(|snapshot| !dead_units.contains(&snapshot.id));
}