use rust_sc2::prelude::*;
use crate::Nikolaj;


#[derive(Default)]
pub struct MapManager {
    pub choke_points: Vec<Choke>,
    pub main_path: Vec<Point2>,
    //pub tank_positions: Vec<Point2>,
}

pub fn init_main_path(bot: &mut Nikolaj) {
    let start_location = bot.start_location;
    let enemy_location = bot.enemy_start;
    
    if let Some(path) = bot.get_path(start_location, enemy_location, PathfindingUnitType::Ground, false, false) {
        bot.map_manager.main_path = path.0;
        return;
    }
    println!("Failed to find main path between start and enemy locations.");
}