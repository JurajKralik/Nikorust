use crate::Nikolaj;
use rust_sc2::prelude::*;


pub fn reaper_control(bot: &mut Nikolaj, unit: &Unit) {
    unit.move_to(Target::Pos(bot.start_center), false);
}