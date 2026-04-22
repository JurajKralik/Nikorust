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
    pub fn get_tank_positions_for_base(&self, base_position: Point2) -> Vec<TacticalPosition> {
        self.choke_points.iter()
            .find(|choke| choke.base_positions.contains(&base_position))
            .map(|choke| choke.tank_positions.clone())
            .unwrap_or_default()
    }
    pub fn get_tank_position_by_tag(&self, unit_tag: u64) -> Option<Point2> {
        for choke in &self.choke_points {
            for tank_pos in &choke.tank_positions {
                if tank_pos.occupied_by == Some(unit_tag) {
                    return Some(tank_pos.position);
                }
            }
        }
        None
    }
    pub fn mark_position_occupied(&mut self, position: Point2, unit_tag: u64) {
        for choke in &mut self.choke_points {
            for tank_pos in &mut choke.tank_positions {
                if tank_pos.position == position {
                    tank_pos.occupied_by = Some(unit_tag);
                    return;
                }
            }
        }
    }
}

#[derive(Default, Clone)]
pub struct ChokePoint {
    pub position: Point2,
    pub main_line: (Point2, Point2),
    pub base_positions: Vec<Point2>,
    pub tank_positions: Vec<TacticalPosition>,
    pub bio_positions: Vec<TacticalPosition>,
    pub bunker_position: Option<Point2>,
}

impl ChokePoint {
    pub fn new(position: Point2, main_line_raw: ((f32, f32), (f32, f32))) -> Self {
        Self {
            position,
            main_line: (Point2::new(main_line_raw.0 .0, main_line_raw.0 .1), Point2::new(main_line_raw.1 .0, main_line_raw.1 .1)),
            base_positions: Vec::new(),
            tank_positions: Vec::new(),
            bio_positions: Vec::new(),
            bunker_position: None,
        }
    }
}

#[derive(Default, Clone)]
pub struct TacticalPosition {
    pub position: Point2,
    pub occupied_by: Option<u64>,
    pub crowd_level: f32,
}

impl From<TacticalPosition> for Point2 {
    fn from(tp: TacticalPosition) -> Point2 {
        tp.position
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
    check_bio_positions(bot);
    check_bunker_positions(bot);
    compute_crowd_levels(bot);
    cleanup_tactical_positions(bot);
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
    const SIEGE_TANK_RANGE: f32 = 12.8;
    for i in 0..bot.map_manager.choke_points.len() {
        if bot.map_manager.choke_points[i].base_positions.is_empty() {
            continue;
        }
        if bot.map_manager.choke_points[i].tank_positions.is_empty() {
            let new_positions = compute_positions_for_choke(&bot.map_manager.choke_points[i], SIEGE_TANK_RANGE);
            let pathable_positions: Vec<TacticalPosition> = new_positions
                .into_iter()
                .filter(|&pos| bot.is_pathable(pos))
                .map(|pos| TacticalPosition { position: pos, occupied_by: None, crowd_level: 0.0 })
                .collect();
            bot.map_manager.choke_points[i].tank_positions = pathable_positions;
        }
    }
}


fn check_bio_positions(bot: &mut Nikolaj) {
    const BIO_RANGE: f32 = 6.0;
    for i in 0..bot.map_manager.choke_points.len() {
        if bot.map_manager.choke_points[i].base_positions.is_empty() {
            continue;
        }
        if bot.map_manager.choke_points[i].bio_positions.is_empty() {
            let new_positions = compute_positions_for_choke(&bot.map_manager.choke_points[i], BIO_RANGE);
            let pathable_positions: Vec<TacticalPosition> = new_positions
                .into_iter()
                .filter(|&pos| bot.is_pathable(pos))
                .map(|pos| TacticalPosition { position: pos, occupied_by: None, crowd_level: 0.0 })
                .collect();
            bot.map_manager.choke_points[i].bio_positions = pathable_positions;
        }
    }
}


fn check_bunker_positions(bot: &mut Nikolaj) {
    const BIO_IN_BUNKER_RANGE: f32 = 7.0;
    for i in 0..bot.map_manager.choke_points.len() {
        if bot.map_manager.choke_points[i].base_positions.is_empty() {
            continue;
        }
        let base_pos = bot.map_manager.choke_points[i].base_positions[0];
        if bot.map_manager.choke_points[i].bunker_position.is_none() {
            let bunker_pos = bot.map_manager.choke_points[i].position.towards(base_pos, BIO_IN_BUNKER_RANGE);
            if bot.can_place(UnitTypeId::Bunker, bunker_pos) {
                bot.map_manager.choke_points[i].bunker_position = Some(bunker_pos);
            }
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


pub fn compute_crowd_levels(bot: &mut Nikolaj) {
    for choke in &mut bot.map_manager.choke_points {
        compute_crowd_levels_for_group(&mut choke.tank_positions);
        compute_crowd_levels_for_group(&mut choke.bio_positions);
    }
}


fn compute_crowd_levels_for_group(positions: &mut Vec<TacticalPosition>) {
    let occupied: Vec<Point2> = positions.iter()
        .filter(|p| p.occupied_by.is_some())
        .map(|p| p.position)
        .collect();

    for pos in positions.iter_mut() {
        pos.crowd_level = occupied.iter()
            .filter(|&&occ| occ != pos.position)
            .map(|&occ| {
                let dist = pos.position.distance(occ);
                if dist > 0.0 { 1.0 / dist } else { f32::MAX }
            })
            .sum();
    }
}


fn cleanup_tactical_positions(bot: &mut Nikolaj) {
    if bot.strategy_data.attack {
        for choke in &mut bot.map_manager.choke_points {
            for pos in &mut choke.tank_positions {
                pos.occupied_by = None;
            }
            for pos in &mut choke.bio_positions {
                pos.occupied_by = None;
            }
        }
    }
    let living_tags: Vec<u64> = bot.units.my.units.iter().map(|u| u.tag()).collect();
    for choke in &mut bot.map_manager.choke_points {
        for pos in &mut choke.tank_positions {
            if let Some(tag) = pos.occupied_by {
                if !living_tags.contains(&tag) {
                    pos.occupied_by = None;
                }
            }
        }
        for pos in &mut choke.bio_positions {
            if let Some(tag) = pos.occupied_by {
                if !living_tags.contains(&tag) {
                    pos.occupied_by = None;
                }
            }
        }
    }
}