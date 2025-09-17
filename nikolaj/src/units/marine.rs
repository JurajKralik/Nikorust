use crate::Nikolaj;
use crate::units::helpers::combat_movement::*;
use crate::units::helpers::surroundings::*;
use rust_sc2::prelude::*;


pub fn marine_control(bot: &mut Nikolaj, unit: &Unit) {
    let surroundings = get_surroundings_info(bot, unit);
    let low_health = unit.health_percentage().unwrap_or(1.0) < 0.4;

    if unit.weapon_cooldown().unwrap_or(0.0) < 0.2 {
        if let Some(target) = surroundings.clone().best_target_in_range {
            unit.attack(Target::Tag(target.tag()), false);
        } else if let Some(target) = surroundings.clone().better_target_off_range {
            if !low_health || surroundings.clone().closest_threat.is_none() {
                unit.attack(Target::Tag(target.tag()), false);
            } else {
                flee(bot, unit, surroundings.clone());
            }
        } else if bot.strategy_data.defend {

        }
    }
}

// Helpers
fn flee(bot: &mut Nikolaj, unit: &Unit, surroundings: SurroundingsInfo) {
    let units = bot.units.clone();
    if flee_to_bunker(units.clone(), unit) {
        return;
    }
    if flee_to_medivac(units.clone(), unit) {
        return;
    }
    if flee_from_threat(bot, unit, surroundings.closest_threat) {
        return;
    }
    let idle_point = bot.strategy_data.idle_point;
    unit.move_to(Target::Pos(idle_point), false);
}
