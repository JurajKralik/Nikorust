use rust_sc2::prelude::*;
use crate::Nikolaj;
use std::f32::consts::PI;


#[derive(Default)]
pub struct MapManager {
    pub main_path: Vec<Point2>,
    pub choke_points: Vec<ChokePoint>
}

impl MapManager {
    pub fn get_choke_point_by_base_position(&self, base_position: Point2) -> Option<ChokePoint> {
        self.choke_points.iter()
            .find(|choke| choke.base_positions.contains(&base_position))
            .cloned()
    }
    pub fn get_closest_choke_point(&self, position: Point2) -> Option<&ChokePoint> {
        self.choke_points.iter()
            .min_by(|a, b| {
                a.position.distance(position)
                    .partial_cmp(&b.position.distance(position))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }
}

#[derive(Default, Clone)]
pub struct ChokePoint {
    pub position: Point2,
    pub main_line: (Point2, Point2),
    pub base_positions: Vec<Point2>,
    pub tank_positions: Vec<Point2>,
    pub bio_positions: Vec<Point2>,
    pub pathable_only: bool
}

impl ChokePoint {
    pub fn new(position: Point2, main_line_raw: ((f32, f32), (f32, f32))) -> Self {
        Self {
            position,
            main_line: (Point2::new(main_line_raw.0 .0, main_line_raw.0 .1), Point2::new(main_line_raw.1 .0, main_line_raw.1 .1)),
            base_positions: Vec::new(),
            tank_positions: Vec::new(),
            bio_positions: Vec::new(),
            pathable_only: true
        }
    }
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
    for choke in chokes_raw {
        let center = choke.center();
        let position = Point2::new(center.0, center.1);
        bot.map_manager.choke_points.push(ChokePoint::new(position, choke.main_line));
    }
}


pub fn map_manager_step(bot: &mut Nikolaj) {
    check_base_choke_points(bot);
    check_tank_positions(bot);
}

fn check_base_choke_points(bot: &mut Nikolaj) {
    let base_positions: Vec<Point2> = bot.units.my.townhalls.ready()
        .iter()
        .map(|b| b.position())
        .collect();

    for base_position in base_positions {
        if bot.map_manager.get_choke_point_by_base_position(base_position).is_none() {
            let choke_pos = get_choke_point_for_base(bot, base_position).map(|c| c.position);
            if let Some(pos) = choke_pos {
                if let Some(choke_mut) = bot.map_manager.choke_points.iter_mut().find(|c| c.position == pos) {
                    choke_mut.base_positions.push(base_position);
                }
            }
        }
    }
}


fn get_choke_point_for_base(bot: &mut Nikolaj, base_position: Point2) -> Option<&ChokePoint> {
    const MIN_DISTANCE_TO_CHOKE: f32 = 25.0;
    let enemy_start = bot.enemy_start;
    let fifth_point = bot
        .get_path(base_position, enemy_start, PathfindingUnitType::Ground, false, false)
        .and_then(|path| path.0.into_iter().nth(5))?;

    bot.map_manager.get_closest_choke_point(fifth_point)
        .filter(|choke| choke.position.distance(base_position) < MIN_DISTANCE_TO_CHOKE)
}


fn check_tank_positions(bot: &mut Nikolaj) {
    for i in 0..bot.map_manager.choke_points.len() {
        if bot.map_manager.choke_points[i].base_positions.is_empty() {
            continue;
        }
        if bot.map_manager.choke_points[i].tank_positions.is_empty() {
            let new_positions = compute_positions_for_choke(&bot.map_manager.choke_points[i], 12.5);
            let pathable_positions: Vec<Point2> = new_positions
                .into_iter()
                .filter(|&pos| bot.is_pathable(pos))
                .collect();
            bot.map_manager.choke_points[i].tank_positions = pathable_positions;
        }
    }
}


fn compute_positions_for_choke(choke: &ChokePoint, range: f32) -> Vec<Point2> {
    let angle_step = 0.5_f32 / range;
    let base_position = choke.base_positions[0];

    let dx = base_position.x - choke.position.x;
    let dy = base_position.y - choke.position.y;
    let center_angle = dy.atan2(dx);

    let half_arc = PI / 4.0;
    let start_angle = center_angle - half_arc;
    let end_angle = center_angle + half_arc;

    let mut angle = start_angle;
    let mut positions = Vec::new();
    while angle <= end_angle + 1e-5 {
        let pos = Point2::new(
            choke.position.x + range * angle.cos(),
            choke.position.y + range * angle.sin(),
        );
        positions.push(pos);
        angle += angle_step;
    }
    positions
}