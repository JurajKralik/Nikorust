use crate::helpers::construction::*;
use crate::Nikolaj;
use rust_sc2::prelude::*;


pub fn construct_bunker(bot: &mut Nikolaj) {
    if !should_try_build_bunker(bot) {
        return;
    }

    if let Some(pos) = find_bunker_placement(bot) {
        build(bot, pos, UnitTypeId::Bunker);
    }
}

fn should_try_build_bunker(bot: &Nikolaj) -> bool {
    // Basics
    // Don't build if one is already in progress
    if bot.already_pending(UnitTypeId::Bunker) > 0 {
        return false;
    }
    // Under construction
    for under_construction in bot.construction_info.under_construction.iter() {
        if under_construction.structure == UnitTypeId::Bunker {
            return false;
        }
    }
    // Savings for expansion
    if bot.macro_manager.expand_priority && bot.get_unit_cost(UnitTypeId::Bunker).minerals > bot.minerals.saturating_sub(400) {
        return false;
    }
    // Can't afford it
    if !bot.can_afford(UnitTypeId::Bunker, false) {
        return false;
    }
    // Needs at least one barracks
    let barracks = bot.units.my.structures.of_type_including_alias(UnitTypeId::Barracks).ready();
    if barracks.is_empty() {
        return false;
    }

    // Additional conditions
    // Max 1 bunker
    let bunker_total = bot.structure_count(UnitTypeId::Bunker)
        + bot.already_pending(UnitTypeId::Bunker);
    if bunker_total > 0 {
        return false;
    }
    // One base defense
    if bot.units.my.townhalls.ready().len() == 1 && !bot.strategy_data.enemy_flooding {
        return false;
    }
    true
}

fn find_bunker_placement(bot: &Nikolaj) -> Option<Point2> {
    let townhall_count = bot.units.my.townhalls.len();
    if townhall_count == 1 {
        if let Some(townhall) = bot.units.my.townhalls.first() {
            if townhall.distance(bot.start_location) < 1.0 {
                let off_position = bot.ramps.my.barracks_in_middle().unwrap_or(townhall.position().towards(bot.enemy_start, 12.0));
                let position = off_position.towards(townhall.position(), 5.0);
                return Some(position);
            }
        }
    } else if townhall_count == 2 {
        let latest_townhall = bot.units.my.townhalls.iter().max_by_key(|th| th.tag());
        if let Some(townhall) = latest_townhall {
            let position = townhall.position().towards(bot.game_info.map_center, 5.0);
            return Some(position);
        }
    }
    None
}

pub fn control_bunker(bot: &mut Nikolaj) {
    for bunker in bot.units.my.structures.of_type(UnitTypeId::Bunker).ready().clone() {
        // Rally
        if bunker.rally_targets().is_empty() {
            if let Some(base) = bot.units.my.townhalls.closest(bunker.position()) {
                bunker.smart(Target::Pos(base.position()), false);
            }
        }
        // Unload
        if bunker.cargo_space_taken() > 0 && bot.strategy_data.attack && !bot.strategy_data.defend{
            bunker.use_ability(AbilityId::UnloadAllBunker, false);
        }
    }
}