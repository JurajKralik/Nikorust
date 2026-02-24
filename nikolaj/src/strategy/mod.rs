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
    check_army_leader(bot);
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
    pub army_leader_tag: u64,
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

pub fn check_army_leader(bot: &mut Nikolaj) {
    let army_leader_tag = bot.strategy_data.army_leader_tag;
    
    if bot.units.my.units.contains_tag(army_leader_tag) {
        return;
    }

    let army_types = Vec::<UnitTypeId>::from([
        UnitTypeId::Marine,
        UnitTypeId::Marauder,
        UnitTypeId::SiegeTank,
        UnitTypeId::SiegeTankSieged,
        UnitTypeId::Thor
    ]);

    let my_army = bot.units.my.units.of_types(&army_types);
    
    let army_center = my_army.center();
    let army_center = match army_center {
        None => return,
        Some(pos) => pos,
    };

    let army_leader = my_army.closest(army_center);
    if let Some(army_leader) = army_leader {
        bot.strategy_data.army_leader_tag = army_leader.tag();
    }

}