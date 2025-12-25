use crate::Nikolaj;

use crate::strategy::unit_snapshot::UnitSnapshot;


#[derive(Default, Clone)]
pub struct MyArmySnapshot {
    pub units: Vec<UnitSnapshot>
}

pub fn refresh_my_army_snapshot(bot: &mut Nikolaj) {
    delete_missing_units(bot);
    add_new_units(bot);
    detect_health_changes_on_cloaked_units(bot);
    update_health(bot);
}

fn delete_missing_units(bot: &mut Nikolaj) {
    let mut current_snapshot: Vec<UnitSnapshot> = bot.strategy_data.my_army.units.clone();
    current_snapshot.retain(|snapshot_unit| {
        bot.units.my.units.clone().iter().any(|my_unit| my_unit.tag() == snapshot_unit.id())
    });
    bot.strategy_data.my_army.units = current_snapshot;
}

fn add_new_units(bot: &mut Nikolaj) {
    let my_units = bot.units.my.units.clone();
    let mut new_units: Vec<UnitSnapshot> = Vec::new();

    for my_unit in my_units {
        if !bot.strategy_data.my_army.units.iter().any(|snapshot_unit| snapshot_unit.id() == my_unit.tag()) {
            new_units.push(UnitSnapshot::from_unit(my_unit, bot.time));
        }
    }

    bot.strategy_data.my_army.units.extend(new_units);
}

fn detect_health_changes_on_cloaked_units(bot: &mut Nikolaj) {
    let my_units = bot.units.my.units.clone();

    for snapshot_unit in bot.strategy_data.my_army.units.iter_mut() {
        if let Some(current_unit) = my_units.iter().find(|my_unit| my_unit.tag() == snapshot_unit.id()) {
            if !current_unit.is_cloaked() {
                continue;
            }
            let current_health = (current_unit.health() + current_unit.shield()) as f32;
            let health_changed = (current_health - snapshot_unit.health()).abs() > f32::EPSILON;
            if !health_changed {
                continue;
            }
            for enemy_unit in bot.units.enemy.units.closer(15.0, current_unit.position()) {
                if enemy_unit.is_detector() {
                    continue;
                }
            }
            bot.combat_info.detected = true;
            bot.combat_info.detected_at = bot.time;
            return;
        }
    }
}

fn update_health(bot: &mut Nikolaj) {
    let my_units = bot.units.my.units.clone();
    let current_time = bot.time;

    for snapshot_unit in bot.strategy_data.my_army.units.iter_mut() {
        if let Some(current_unit) = my_units.iter().find(|my_unit| my_unit.tag() == snapshot_unit.id()) {
            snapshot_unit.unit = current_unit.clone();
            snapshot_unit.last_seen = current_time;
        }
    }
}
