use crate::Nikolaj;
use crate::units::helpers::surroundings::*;
use crate::units::helpers::combat_movement::*;
use rust_sc2::prelude::*;


pub fn widowmine_control(bot: &mut Nikolaj, unit: &Unit) {
    let surroundings = get_surroundings_info(bot, unit, SurroundingsOptions::default());
    if surroundings.best_target_in_range.is_some() || surroundings.better_target_off_range.is_some() {
        siege_up(bot, unit);
    } else {
        unsiege(bot, unit);
    }
    attack_no_spam(unit, Target::Pos(bot.strategy_data.army_center));
}