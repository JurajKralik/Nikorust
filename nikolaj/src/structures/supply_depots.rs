use crate::helpers::construction::*;
use crate::Nikolaj;
use rust_sc2::prelude::*;
use rust_sc2::ramp::Ramp;

pub fn construct_supply_depots(bot: &mut Nikolaj) {
    if !check_depots_needed(bot) {
        return;
    }

    let position = get_next_depot_position(bot);
    let position = match position {
        Target::None => return,
        Target::Pos(pos) => pos, 
        Target::Tag(_) => return,
    };
    build(bot, position, UnitTypeId::SupplyDepot);
}

fn check_depots_needed(bot: &mut Nikolaj) -> bool {
    let pending = bot.already_pending(UnitTypeId::SupplyDepot);
    let supply_left = bot.supply_left;
    let supply_cap = bot.supply_cap;
    let supply_used = bot.supply_used;

    // Under construction
    for under_construction in bot.construction_info.under_construction.iter() {
        if under_construction.structure == UnitTypeId::SupplyDepot {
            return false;
        }
    }
    // Stop
    if supply_cap > 199 || bot.units.my.townhalls.is_empty() {
        return false;
    }
    // Classic
    if supply_left < 6 && pending == 0 {
        return true;
    }
    // Supply block close
    if supply_left < 6 && pending < 2 && supply_used > 30 {
        return true;
    }
    // Supply block too close
    if supply_used > 45 && supply_left < 3 && pending < 3 {
        return true;
    }
    // Lategame
    if supply_cap > 40 && supply_left < 8 && pending == 0 {
        return true;
    }
    // Close middle ramp
    if let Some(depot_in_middle) = bot.ramps.my.depot_in_middle() {
        if bot
            .units
            .my
            .structures
            .closer(1.5, depot_in_middle)
            .is_empty()
            && bot
                .units
                .my
                .structures
                .of_type(UnitTypeId::Barracks)
                .closer(4.0, depot_in_middle)
                .is_empty()
            && !bot
                .units
                .my
                .structures
                .of_type_including_alias(UnitTypeId::Barracks)
                .is_empty()
            && pending == 0
        {
            return true;
        }
    }
    false
}

fn get_next_depot_position(bot: &mut Nikolaj) -> Target {
    // Ramp corners
    if let Some(depots) = bot.ramps.my.corner_depots() {
        for depot in depots {
            if bot.units.my.structures.closer(2.0, depot).is_empty() {
                return Target::Pos(depot);
            }
        }
    }
    // Ramp middle
    if let Some(depot) = bot.ramps.my.depot_in_middle() {
        if bot.units.my.structures.closer(2.0, depot).is_empty() {
            return Target::Pos(depot);
        }
    }
    // Base walls
    for base in bot.units.my.townhalls.clone() {
        let base_position = base.position().clone();
        // First base ignore
        if base_position.distance(bot.start_location) < 3.0 {
            continue;
        }
        
        // Towards closest ramp
        // Get closest ramp
        let mut closest_ramp: Option<Ramp> = None;
        let mut closest_distance = 9999.0;
        for ramp in bot.ramps.all.clone() {
            if let Some(ramp_top) = ramp.top_center() {
                let distance = base_position.distance(ramp_top);
                if distance < closest_distance {
                    closest_distance = distance;
                    closest_ramp = Some(ramp.clone());
                }
            }
        }
        // Find placement
        if let Some(ramp) = closest_ramp {
            if let Some(ramp_pos) = ramp.top_center(){
                let wall_pos = base_position.towards(Point2 { x: ramp_pos.0 as f32, y: ramp_pos.1 as f32 }, 7.0);
                if bot.units.my.structures.of_type_including_alias(UnitTypeId::SupplyDepot).closer(10.0, wall_pos).len() < 5 {
                    if let Some(depot) =
                        bot.find_placement(UnitTypeId::SupplyDepot, wall_pos, Default::default())
                    {
                        return Target::Pos(depot);
                    }
                }
            }
        }
        
        // Towards map center
        let base_depots_pos = base_position.towards(bot.game_info.map_center, 7.0);
        if let Some(depot) =
            bot.find_placement(UnitTypeId::SupplyDepot, base_depots_pos, Default::default())
        {
            return Target::Pos(depot);
        }
    }
    // Main base mineral line
    if let Some(main_base) = bot.units.my.townhalls.closest(bot.start_location) {
        let position = main_base.position().towards(bot.game_info.map_center, -8.0);
        if let Some(depot) =
            bot.find_placement(UnitTypeId::SupplyDepot, position, Default::default())
        {
            return Target::Pos(depot);
        }
    }
    Target::None
}

pub fn supply_depots_control(bot: &mut Nikolaj) {
    if bot.units.my.structures.of_type_including_alias(UnitTypeId::SupplyDepot).is_empty() {
        return;
    }
    // Closed
    for depot in bot.units.my.structures.of_type(UnitTypeId::SupplyDepot).clone() {
        let mut open = true;
        for unit in bot.units.enemy.units.closer(8.0, depot.position()) {
            if !unit.is_flying() {
                open = false;
                break;
            }
        }
        if !open {
            continue;
        }
        depot.command(AbilityId::MorphSupplyDepotLower, Target::None, false);
    }

    // Opened
    for depot in bot.units.my.structures.of_type(UnitTypeId::SupplyDepotLowered).clone() {
        for unit in bot.units.enemy.units.closer(8.0, depot.position()) {
            if !unit.is_flying() {
                depot.command(AbilityId::MorphSupplyDepotRaise, Target::None, false);
                break;
            }
        }
    }
}