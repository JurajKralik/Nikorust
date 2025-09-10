use crate::Nikolaj;
use std::collections::HashMap;
use rust_sc2::prelude::*;

pub mod helpers;

pub fn scv_step(bot: &mut Nikolaj) {
    helpers::cleanup::reset_frame(bot);
    helpers::discovery::init_resources(bot);
    helpers::repair::repair(bot); // TODO
    helpers::allocation::refresh_allocations(bot); // TODO
    helpers::allocation::rebalance(bot); // TODO
    helpers::building::finish_buildings(bot); // TODO
    helpers::speedmine::tick(bot); // TODO
}


#[derive(Default)]
pub struct ScvControl {
    pub mining_distribution: HashMap<u64, Vec<u64>>,
    pub repair_list: HashMap<u64, Vec<u64>>,
    pub current_refineries: Units,
    pub current_mineral_fields: Units,
    pub current_gas_workers: Units,
    pub current_mineral_workers: Units,
    pub current_idle_workers: Units,
}