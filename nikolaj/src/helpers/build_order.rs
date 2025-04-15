use crate::Nikolaj;
use rust_sc2::prelude::*;


pub fn decide_strategy(bot: &mut Nikolaj) {
    check_starters(bot);
    // Zerg
    if bot.enemy_race == Race::Zerg {
        // Barracks
        if !bot.starter_reaper {
            let marines = bot.unit_count(UnitTypeId::Marine) + bot.already_pending(UnitTypeId::Marine);
            let marauders = (bot.unit_count(UnitTypeId::Marauder) + bot.already_pending(UnitTypeId::Marauder)) * 2;
            if marines <= 4 {
                bot.barracks_priority = Some(UnitTypeId::Marine);
            } else {
                if marauders <= 2 {
                    bot.barracks_priority = Some(UnitTypeId::Marauder);
                }
            }
        }
    } else if bot.enemy_race == Race::Protoss {

    } else if bot.enemy_race == Race::Terran {

    } else {

    }
}

fn check_starters(bot: &mut Nikolaj) {
    if bot.starter_reaper {
        if bot.unit_count(UnitTypeId::Reaper) > 0 || bot.already_pending(UnitTypeId::Reaper) > 0 {
            bot.starter_reaper = false;
        } else {
            bot.barracks_priority = Some(UnitTypeId::Reaper);
        }
    }

    if bot.starter_banshee {
        if bot.unit_count(UnitTypeId::Banshee) > 0 || bot.already_pending(UnitTypeId::Banshee) > 0 {
            bot.starter_banshee = false;
        } else {
            bot.starport_priority = Some(UnitTypeId::Banshee);
        }
    }
}