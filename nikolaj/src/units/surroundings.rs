use crate::Nikolaj;
use rust_sc2::prelude::*;


pub struct SurroundingsInfo {
    pub current_target: Option<Unit>,
    pub better_target: Option<Unit>,
    pub closest_threat: Option<Unit>,
    pub closest_counter: Option<Unit>,
}

pub fn get_surroundings_info(bot: &mut Nikolaj, unit: &Unit) -> SurroundingsInfo {
    let mut surroundings = SurroundingsInfo {
        current_target: None,
        better_target: None,
        closest_threat: None,
        closest_counter: None,
    };




    surroundings
}