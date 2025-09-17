use crate::Nikolaj;
use rust_sc2::prelude::*;

pub fn decide_build_strategy(bot: &mut Nikolaj) {
    check_starters(bot);
    decide_barracks(bot);
    decide_factory(bot);
    decide_starport(bot);
}

fn decide_barracks(bot: &mut Nikolaj) {
    // Starter Reaper
    if bot.starter_reaper {
        bot.barracks_priority = Some(UnitTypeId::Reaper);
        return;
    }

    let marines = bot.unit_count(UnitTypeId::Marine) + bot.already_pending(UnitTypeId::Marine);
    let marauders =
        bot.unit_count(UnitTypeId::Marauder) + bot.already_pending(UnitTypeId::Marauder);

    // Zerg (Marine priority) TODO: add marauders against roaches
    if bot.enemy_race == Race::Zerg {
        if marines <= 4 {
            bot.barracks_priority = Some(UnitTypeId::Marine);
        } else if marauders <= 2 {
            bot.barracks_priority = Some(UnitTypeId::Marauder);
        } else if marines <= 8 {
            bot.barracks_priority = Some(UnitTypeId::Marine);
        } else {
            bot.barracks_priority = None;
        }
    // Protoss (50/50)
    } else if bot.enemy_race == Race::Protoss {
        if marines >= 8 && marauders >= 8 {
            bot.barracks_priority = None;
        } else if marines <= marauders {
            bot.barracks_priority = Some(UnitTypeId::Marine);
        } else {
            bot.barracks_priority = Some(UnitTypeId::Marauder);
        }
    // Terran (few marines)
    } else if bot.enemy_race == Race::Terran {
        if marines <= 6 {
            bot.barracks_priority = Some(UnitTypeId::Marine);
        } else {
            bot.barracks_priority = None;
        }
    // Random
    } else {
        bot.barracks_priority = Some(UnitTypeId::Marine);
    }
}

fn decide_factory(bot: &mut Nikolaj) {
    let tanks = bot.unit_count(UnitTypeId::SiegeTank)
        + bot.unit_count(UnitTypeId::SiegeTankSieged)
        + bot.already_pending(UnitTypeId::SiegeTank);
    let vikings = bot.unit_count(UnitTypeId::VikingAssault)
        + bot.already_pending(UnitTypeId::VikingAssault)
        + bot.unit_count(UnitTypeId::VikingFighter)
        + bot.already_pending(UnitTypeId::VikingFighter);
    let cyclones = bot.unit_count(UnitTypeId::Cyclone)
        + bot.already_pending(UnitTypeId::Cyclone);
    let widow_mines = bot.unit_count(UnitTypeId::WidowMine)
        + bot.already_pending(UnitTypeId::WidowMine)
        + bot.unit_count(UnitTypeId::WidowMineBurrowed);

    // First Tanks
    if tanks < 4 {
        bot.factory_priority = Some(UnitTypeId::SiegeTank);
    // Some Cyclones
    } else if cyclones < 2 {
        bot.factory_priority = Some(UnitTypeId::Cyclone);
    // Viking/Tank army
    } else if vikings + 2 > tanks {
        bot.factory_priority = Some(UnitTypeId::SiegeTank);
    // Support Mines
    } else if widow_mines < 2 {
        bot.factory_priority = Some(UnitTypeId::WidowMine);
    // Fill up Cyclones
    } else if cyclones < 6 {
        bot.factory_priority = Some(UnitTypeId::Cyclone);
    // Wait for Vikings
    } else {
        bot.factory_priority = None;
    }
}

fn decide_starport(bot: &mut Nikolaj) {
    // Starter Banshee
    if bot.starter_banshee {
        bot.starport_priority = Some(UnitTypeId::Banshee);
        return;
    }

    let medivacs = bot.unit_count(UnitTypeId::Medivac)
        + bot.already_pending(UnitTypeId::Medivac);
    let banshees = bot.unit_count(UnitTypeId::Banshee)
        + bot.already_pending(UnitTypeId::Banshee);
    let ravens = bot.unit_count(UnitTypeId::Raven)
        + bot.already_pending(UnitTypeId::Raven);
    let tanks = bot.unit_count(UnitTypeId::SiegeTank)
        + bot.unit_count(UnitTypeId::SiegeTankSieged)
        + bot.already_pending(UnitTypeId::SiegeTank);
    let vikings = bot.unit_count(UnitTypeId::VikingAssault)
        + bot.already_pending(UnitTypeId::VikingAssault)
        + bot.unit_count(UnitTypeId::VikingFighter)
        + bot.already_pending(UnitTypeId::VikingFighter);
    let bio = bot.unit_count(UnitTypeId::Marine) 
        + bot.unit_count(UnitTypeId::Marauder)
        + bot.already_pending(UnitTypeId::Marine)
        + bot.already_pending(UnitTypeId::Marauder);
    
    // Detection
    if bot.strategy_data.enemy_cloaking && ravens == 0 {
        bot.starport_priority = Some(UnitTypeId::Raven);
    // Bio squads
    } else if bio > 4 && ((bio + 1) / 4) as usize > medivacs {
        bot.starport_priority = Some(UnitTypeId::Medivac);
    // Air control
    } else if tanks + 1 > vikings {
        bot.starport_priority = Some(UnitTypeId::VikingAssault);
    // Banshee harass
    } else if banshees < 4 {
        bot.starport_priority = Some(UnitTypeId::Banshee);
    } else {
        bot.starport_priority = None;
    }
}
    

fn check_starters(bot: &mut Nikolaj) {
    if bot.starter_reaper {
        if bot.unit_count(UnitTypeId::Reaper) > 0 || bot.already_pending(UnitTypeId::Reaper) > 0 {
            bot.starter_reaper = false;
        }
    }

    if bot.starter_banshee {
        if bot.unit_count(UnitTypeId::Banshee) > 0 || bot.already_pending(UnitTypeId::Banshee) > 0 {
            bot.starter_banshee = false;
        }
    }
}
