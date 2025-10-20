use rust_sc2::prelude::*;


#[derive(Default)]
pub struct MapManager {
    pub choke_points: Vec<Choke>,
    pub tank_positions: Vec<Point2>,
}