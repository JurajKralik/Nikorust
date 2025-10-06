use crate::Nikolaj;
use crate::units::helpers::surroundings::*;
use crate::units::helpers::combat_movement::*;
use crate::units::helpers::threat_detection::*;
use rust_sc2::prelude::*;


pub fn tank_control(bot: &mut Nikolaj, unit: &Unit) {
    let surroundings = get_surroundings_info(bot, unit);
    let in_danger = surroundings.clone().threat_level > ThreatLevel::None;

    if in_danger && surroundings.best_target_in_range.is_none() && surroundings.better_target_off_range.is_none() && surroundings.closest_structure.is_none() {
        force_unsiege(bot, unit);
        return;
    }
    if surroundings.best_target_in_range.is_some() || surroundings.better_target_off_range.is_some() || surroundings.closest_structure.is_some() {
        siege_up(bot, unit);
    } else {
        unsiege(bot, unit);
    }
    attack_no_spam(unit, Target::Pos(bot.strategy_data.army_center));
}