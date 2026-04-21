use rust_sc2::prelude::*;
use crate::Nikolaj;
use std::collections::HashMap;


#[derive(Default)]
pub struct MapManager {
    pub choke_points: Vec<Point2>,
    pub main_path: Vec<Point2>,
    pub base_choke_points: HashMap<u64, Point2>,
    //pub tank_positions: Vec<Point2>,
}

pub fn init_choke_points(bot: &mut Nikolaj) {
    let chokes_raw = bot.get_chokes_lazy().clone();
    let chokes_centers: Vec<Point2> = chokes_raw.iter().map(|choke| choke.center_p2()).collect();
    bot.map_manager.choke_points = chokes_centers;
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

pub fn map_manager_step(bot: &mut Nikolaj) {
    check_base_choke_points(bot);
}

pub fn check_base_choke_points(bot: &mut Nikolaj) {
    for base in bot.units.my.townhalls.ready() {
        let base_tag = base.tag();
        if !bot.map_manager.base_choke_points.contains_key(&base_tag) {
            if let Some(choke_point) = find_base_choke_point(bot, base) {
                bot.map_manager.base_choke_points.insert(base_tag, choke_point);
            }
        }
    }
}


fn find_base_choke_point(bot: &mut Nikolaj, base: Unit) -> Option<Point2> {
    let base_position = base.position();
    if let Some(path_to_enemy_base) = bot.get_path(base_position, bot.enemy_start, PathfindingUnitType::Ground, false, false) {
        if let Some(fifth_point) = path_to_enemy_base.0.iter().nth(5).cloned() {
            let closest_choke_to_fifth_point = bot.map_manager.choke_points.iter()
                .min_by(|a, b| {
                    let dist_a = a.distance(fifth_point);
                    let dist_b = b.distance(fifth_point);
                    dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
                });
            if let Some(closest_choke) = closest_choke_to_fifth_point {
                if closest_choke.distance(base_position) < 25.0 {
                    return Some(*closest_choke);
                }
            }
        }
    }
    None
}