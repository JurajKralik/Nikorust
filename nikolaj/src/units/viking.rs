use crate::Nikolaj;
use rust_sc2::prelude::*;
use crate::units::helpers::combat_movement::*;


pub fn viking_control(bot: &mut Nikolaj, unit: &Unit) {
    attack_no_spam(unit, Target::Pos(bot.strategy_data.army_center));
}