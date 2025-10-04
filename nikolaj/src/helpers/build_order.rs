use crate::Nikolaj;
use rust_sc2::prelude::*;

pub fn decide_build_strategy(bot: &mut Nikolaj) {
    decide_expansion_priority(bot);
    check_starters(bot);
    decide_barracks(bot);
    decide_factory(bot);
    decide_starport(bot);
}

fn decide_expansion_priority(bot: &mut Nikolaj) {
    if bot.worker_allocator.resources.len() < 8 
        && !!bot.strategy_data.enemy_flooding 
        && !bot.strategy_data.enemy_worker_rush {
        bot.macro_manager.expand_priority = true;
    } else {
        bot.macro_manager.expand_priority = false;
    }
}

fn decide_barracks(bot: &mut Nikolaj) {
    // Starter Reaper
    if bot.macro_manager.starter_reaper {
        bot.macro_manager.barracks_priority = Some(UnitTypeId::Reaper);
        return;
    }

    let marines = bot.unit_count(UnitTypeId::Marine) + bot.already_pending(UnitTypeId::Marine);
    let marauders =
        bot.unit_count(UnitTypeId::Marauder) + bot.already_pending(UnitTypeId::Marauder);

    // Zerg (Marine priority) TODO: add marauders against roaches
    if bot.enemy_race == Race::Zerg {
        if marines <= 4 {
            bot.macro_manager.barracks_priority = Some(UnitTypeId::Marine);
        } else if marauders <= 2 {
            bot.macro_manager.barracks_priority = Some(UnitTypeId::Marauder);
        } else if marines <= 8 {
            bot.macro_manager.barracks_priority = Some(UnitTypeId::Marine);
        } else {
            bot.macro_manager.barracks_priority = None;
        }
    // Protoss (50/50)
    } else if bot.enemy_race == Race::Protoss {
        if marines >= 8 && marauders >= 8 {
            bot.macro_manager.barracks_priority = None;
        } else if marines <= marauders {
            bot.macro_manager.barracks_priority = Some(UnitTypeId::Marine);
        } else {
            bot.macro_manager.barracks_priority = Some(UnitTypeId::Marauder);
        }
    // Terran (few marines)
    } else if bot.enemy_race == Race::Terran {
        let start_marines_needed = marines <= 6;
        let mid_marines_needed = bot.minerals > 300 && marines <= 20;
        let late_marines_needed = bot.minerals > 300 && bot.supply_left > 4 && bot.supply_used > 70 && marines <= 40;

        if start_marines_needed || mid_marines_needed || late_marines_needed {
            bot.macro_manager.barracks_priority = Some(UnitTypeId::Marine);
        } else {
            bot.macro_manager.barracks_priority = None;
        }
    // Random
    } else {
        bot.macro_manager.barracks_priority = Some(UnitTypeId::Marine);
    }
}

fn decide_factory(bot: &mut Nikolaj) {
    let tanks = bot.unit_count(UnitTypeId::SiegeTank)
        + bot.unit_count(UnitTypeId::SiegeTankSieged)
        + bot.already_pending(UnitTypeId::SiegeTank);
    let widow_mines = bot.unit_count(UnitTypeId::WidowMine)
        + bot.already_pending(UnitTypeId::WidowMine)
        + bot.unit_count(UnitTypeId::WidowMineBurrowed);
    let tank_cost = bot.get_unit_cost(UnitTypeId::SiegeTank);
    let start_tanks_needed = tanks < 4;
    let too_many_tanks = (tanks as f32 * tank_cost.supply as f32 * 2.5) > (bot.supply_army as f32);
    let under_tank_cap = tanks < 16;

    // First Tanks
    if start_tanks_needed {
        bot.macro_manager.factory_priority = Some(UnitTypeId::SiegeTank);
    // Some Cyclones
    } else if too_many_tanks {
        bot.macro_manager.factory_priority = Some(UnitTypeId::Cyclone);
    // Support Mine
    } else if widow_mines < 1 {
        bot.macro_manager.factory_priority = Some(UnitTypeId::WidowMine);
    // Fill up Tanks
    } else if under_tank_cap {
        bot.macro_manager.factory_priority = Some(UnitTypeId::SiegeTank);
    // Late Game
    } else {
        bot.macro_manager.factory_priority = Some(UnitTypeId::ThorAALance);
    }
}

fn decide_starport(bot: &mut Nikolaj) {
    // Starter Banshee
    if bot.macro_manager.starter_banshee {
        bot.macro_manager.starport_priority = Some(UnitTypeId::Banshee);
        return;
    }

    let medivacs = bot.unit_count(UnitTypeId::Medivac)
        + bot.already_pending(UnitTypeId::Medivac);
    let ravens = bot.unit_count(UnitTypeId::Raven)
        + bot.already_pending(UnitTypeId::Raven);
    let vikings = bot.unit_count(UnitTypeId::VikingAssault)
        + bot.already_pending(UnitTypeId::VikingAssault)
        + bot.unit_count(UnitTypeId::VikingFighter)
        + bot.already_pending(UnitTypeId::VikingFighter);
    let bio = bot.unit_count(UnitTypeId::Marine) 
        + bot.unit_count(UnitTypeId::Marauder)
        + bot.already_pending(UnitTypeId::Marine)
        + bot.already_pending(UnitTypeId::Marauder);
    let viking_cost = bot.get_unit_cost(UnitTypeId::VikingFighter);
    let raven_needed = bot.strategy_data.enemy_cloaking && ravens == 0;
    let first_vikings_needed = vikings < 2;
    let bio_support_needed = bio > 4 && ((bio + 1) / 4) as usize > medivacs;
    let too_many_vikings = (vikings as f32 * viking_cost.supply as f32 * 2.5) > (bot.supply_army as f32);
    
    // Detection
    if raven_needed {
        bot.macro_manager.starport_priority = Some(UnitTypeId::Raven);
    // First Vikings
    } else if first_vikings_needed {
        bot.macro_manager.starport_priority = Some(UnitTypeId::VikingFighter);
    // Bio squads
    } else if bio_support_needed {
        bot.macro_manager.starport_priority = Some(UnitTypeId::Medivac);
    // Banshee harass
    } else if too_many_vikings {
        bot.macro_manager.starport_priority = Some(UnitTypeId::Banshee);
    // Air control
    } else {
        bot.macro_manager.starport_priority = Some(UnitTypeId::VikingFighter);
    }
}
    

fn check_starters(bot: &mut Nikolaj) {
    if bot.macro_manager.starter_reaper {
        if bot.unit_count(UnitTypeId::Reaper) > 0 || bot.already_pending(UnitTypeId::Reaper) > 0 {
            bot.macro_manager.starter_reaper = false;
        }
    }

    if bot.macro_manager.starter_banshee {
        if bot.unit_count(UnitTypeId::Banshee) > 0 || bot.already_pending(UnitTypeId::Banshee) > 0 {
            bot.macro_manager.starter_banshee = false;
        }
    }
}
