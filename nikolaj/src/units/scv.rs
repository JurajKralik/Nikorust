use crate::Nikolaj;
use std::collections::HashMap;
use rust_sc2::prelude::*;

pub mod helpers;

pub fn scv_tick(bot: &mut Nikolaj) {
    helpers::housekeeping::reset_frame(bot);
    helpers::discovery::init_resources(bot);
    helpers::repair::update_targets(bot);
    helpers::allocation::refresh_allocations(bot);
    helpers::allocation::rebalance(bot);      // uses saturation internally
    helpers::speedmine::tick(bot);
    helpers::building::finish_builds(bot);
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