use rust_sc2::prelude::*;


#[derive(Default)]
pub struct MapManager {
    pub choke_points: Vec<Point2>,
    pub tank_positions: Vec<Point2>,
}