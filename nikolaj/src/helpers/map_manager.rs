use rust_sc2::prelude::*;
use crate::Nikolaj;
use std::collections::HashMap;
use std::f32::consts::PI;


#[derive(Default)]
pub struct MapManager {
    pub main_path: Vec<Point2>,
    pub choke_points: Vec<Point2>,
    pub base_choke_points: HashMap<Point2, Point2>, // base position -> choke point position
    pub tank_positions: Vec<Point2>,
}


pub fn init_map_manager(bot: &mut Nikolaj) {
    init_choke_points(bot);
    init_main_path(bot);
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

pub fn init_choke_points(bot: &mut Nikolaj) {
    let chokes_raw = bot.get_chokes_lazy().clone();
    let chokes_centers: Vec<Point2> = chokes_raw.iter().map(|choke| choke.center_p2()).collect();
    bot.map_manager.choke_points = chokes_centers;
}



pub fn map_manager_step(bot: &mut Nikolaj) {
    check_base_choke_points(bot);
}

pub fn check_base_choke_points(bot: &mut Nikolaj) {
    let bases: Vec<Point2> = bot.units.my.townhalls.ready()
        .iter()
        .map(|b| b.position())
        .collect();

    for base_position in bases {
        if !bot.map_manager.base_choke_points.contains_key(&base_position) {
            if let Some(choke_point) = find_base_choke_point_pos(bot, base_position) {
                bot.map_manager.base_choke_points.insert(base_position, choke_point);
                compute_tank_positions_for_choke(bot, choke_point, base_position);
            }
        }
    }
}


fn find_base_choke_point_pos(bot: &mut Nikolaj, base_position: Point2) -> Option<Point2> {
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


fn compute_tank_positions_for_choke(bot: &mut Nikolaj, choke: Point2, base_position: Point2) {
    const SIEGE_TANK_RANGE: f32 = 12.5;
    let angle_step = 0.5_f32 / SIEGE_TANK_RANGE;

    let dx = base_position.x - choke.x;
    let dy = base_position.y - choke.y;
    let center_angle = dy.atan2(dx);

    let half_arc = PI / 4.0;
    let start_angle = center_angle - half_arc;
    let end_angle = center_angle + half_arc;

    let mut angle = start_angle;
    while angle <= end_angle + 1e-5 {
        let pos = Point2::new(
            choke.x + SIEGE_TANK_RANGE * angle.cos(),
            choke.y + SIEGE_TANK_RANGE * angle.sin(),
        );
        if bot.is_pathable((pos.x as usize, pos.y as usize)) {
            bot.map_manager.tank_positions.push(pos);
        }
        angle += angle_step;
    }
}