use crate::Nikolaj;
use rust_sc2::prelude::*;


pub fn hellion_control(bot: &mut Nikolaj, unit: &Unit) {
    unit.attack(Target::Pos(bot.strategy_data.army_center), false);
}