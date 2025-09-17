use crate::Nikolaj;
use rust_sc2::prelude::*;
use crate::units::surroundings::*;


pub fn marine_control(bot: &mut Nikolaj, unit: &Unit) {
    let surroundings = get_surroundings_info(bot, unit);
    if let Some(target) = surroundings.best_target_in_range {
        unit.attack(Target::Tag(target.tag()), false);
    }
}