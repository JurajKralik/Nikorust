use crate::Nikolaj;
use rust_sc2::prelude::*;

pub mod points;
pub mod unit_snapshot;
pub mod enemy_army;
pub mod my_army;
pub mod offensive;
pub mod enemy_readings;

use enemy_army::EnemyArmySnapshot;
use my_army::MyArmySnapshot;


pub fn strategy_step(bot: &mut Nikolaj) {
    points::refresh_points(bot);
    enemy_army::refresh_enemy_army_snapshot(bot);
    my_army::refresh_my_army_snapshot(bot);
    offensive::decide_offensive(bot);
    enemy_readings::read_enemy_strategy(bot);
}

#[derive(Default)]
pub struct StrategyData {
    pub enemy_army: EnemyArmySnapshot,
    pub my_army: MyArmySnapshot,
    pub idle_point: Point2,
    pub defense_point: Point2,
    pub attack_point: Point2,
    pub army_center: Point2,
    pub harass_points: Vec<Point2>,
    pub repair_points: Vec<Point2>,
    pub defend: bool,
    pub attack: bool,
    pub enemy_cloaking: bool,
    pub enemy_flooding: bool,
    pub enemy_worker_rush: bool,
    pub enemy_worker_rush_time: f32,
    pub enemy_ramp_blocking: bool,
    pub enemy_ramp_blocking_steps: usize,
    pub enemy_ramp_blocking_time: f32,
    pub enemy_flying_units: bool,
}

impl StrategyData {
    pub fn get_enemy_army_supply(&self) -> usize {
        self.enemy_army.get_army_supply()
    }
}
