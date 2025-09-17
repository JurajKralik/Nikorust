use crate::Nikolaj;
use rust_sc2::prelude::*;
use crate::units::surroundings::*;


pub fn marine_control(bot: &mut Nikolaj, unit: &Unit) {
    let surroundings = get_surroundings_info(bot, unit);
}