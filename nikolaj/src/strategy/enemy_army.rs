use crate::Nikolaj;
use rust_sc2::prelude::*;

use crate::strategy::unit_snapshot::UnitSnapshot;


const FADE_OUT_TIME: f32 = 45.0;
const COMBAT_RELEVANT_TIME: f32 = 8.0;

#[derive(Default, Clone)]
pub struct EnemyArmySnapshot {
    pub units: Vec<UnitSnapshot>
}

impl EnemyArmySnapshot {
    pub fn get_supply_by_type(&self, unit_type: UnitTypeId) -> usize {
        self.units.iter().filter(|unit| unit.type_id() == unit_type).map(|unit| unit.supply()).sum()
    }
    pub fn get_army_supply(&self) -> usize {
        self.units.iter().filter(|unit| unit.is_combat_unit).map(|unit| unit.supply()).sum()
    }
}

pub fn refresh_enemy_army_snapshot(bot: &mut Nikolaj) {
    let visible_enemies = get_visible_enemies(bot);
    let appended_snapshot = get_appended_enemy_army_snapshot(bot, visible_enemies);
    bot.strategy_data.enemy_army.units = appended_snapshot;
    delete_outdated_enemy_snapshots(bot);
    let dead_units = bot.state.observation.raw.dead_units.clone();
    delete_dead_enemy_snapshots(bot, dead_units);
    check_combat_relevance(bot);
}

fn get_visible_enemies(bot: &Nikolaj) -> Vec<UnitSnapshot> {
    let enemies = bot.units.enemy.units.clone();
    let mut currently_visible_army: Vec<UnitSnapshot> = Vec::new();
    for enemy in enemies {
        let snapshot = UnitSnapshot::from_unit(enemy, bot.time);
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
        if let Some(existing) = current_snapshot.iter_mut().find(|e| e.id() == visible.id()) {
            existing.unit = visible.unit.clone();
            existing.last_seen = bot.time;
            existing.is_snapshot = false;
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
        .retain(|snapshot| current_time - snapshot.last_seen <= FADE_OUT_TIME);
}

fn delete_dead_enemy_snapshots(bot: &mut Nikolaj, dead_units: Vec<u64>) {
    bot.strategy_data
        .enemy_army
        .units
        .retain(|snapshot| !dead_units.contains(&snapshot.id()));
}

fn check_combat_relevance(bot: &mut Nikolaj) {
    let current_time = bot.time;
    for snapshot in bot.strategy_data.enemy_army.units.iter_mut() {
        if snapshot.is_snapshot && (current_time - snapshot.last_seen) > COMBAT_RELEVANT_TIME {
            snapshot.is_combat_relevant = false;
        }
    }
}