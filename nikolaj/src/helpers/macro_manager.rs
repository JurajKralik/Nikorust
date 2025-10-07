use rust_sc2::prelude::*;

use crate::Nikolaj;
use crate::structures::command_center::*;
use crate::structures::supply_depots::*;
use crate::structures::barracks::*;
use crate::structures::factory::*;
use crate::structures::refinery::*;
use crate::structures::starport::*;
use crate::structures::addons::*;
use crate::structures::engineering_bay::*;
use crate::structures::armory::*;
use crate::structures::missile_turrets::*;
use crate::structures::bunker::*;
use crate::helpers::build_order::*;
use crate::helpers::construction::*;

pub fn macro_manager_step(bot: &mut Nikolaj) {
    construction_info_step(bot);
    decide_build_strategy(bot);

    finish_constructions_without_worker(bot);
    cancel_constructions_in_danger(bot);
    
    construct_command_center(bot);
    control_command_center(bot);
    construct_refinery(bot);
    construct_supply_depot(bot);
    control_supply_depot(bot);
    
    control_barracks(bot);
    control_factory(bot);
    control_starport(bot);
    construct_barracks(bot);
    construct_factory(bot);
    construct_starport(bot);

    construct_bunker(bot);
    control_bunker(bot);
    construct_missile_turrets(bot);
    construct_engineering_bay(bot);
    control_engineering_bay(bot);
    construct_armory(bot);
    control_armory(bot);
    control_addons(bot);
}

#[derive(Debug, Clone)]
pub struct MacroManager {
    pub expand_priority: bool,
    pub barracks_priority: Option<UnitTypeId>,
    pub factory_priority: Option<UnitTypeId>,
    pub starport_priority: Option<UnitTypeId>,
    pub starter_reaper: bool,
    pub starter_banshee: bool,
}

impl Default for MacroManager {
    fn default() -> Self {
        Self {
            expand_priority: false,
            barracks_priority: None,
            factory_priority: None,
            starport_priority: None,
            starter_reaper: true,
            starter_banshee: true,
        }
    }
}