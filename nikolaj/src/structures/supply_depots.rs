use crate::helpers::construction::*;
use crate::Nikolaj;
use rust_sc2::prelude::*;

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
    let builder = get_builder(bot, Target::Pos(position));
    if let Some(worker) = builder {
        worker.build(UnitTypeId::SupplyDepot, position, false);
    }
}

fn check_depots_needed(bot: &mut Nikolaj) -> bool {
    let pending = bot.already_pending(UnitTypeId::SupplyDepot);
    let supply_left = bot.supply_left;
    let supply_cap = bot.supply_cap;
    let supply_used = bot.supply_used;

    //stop
    if supply_cap > 199 || bot.units.my.townhalls.is_empty() {
        return false;
    }
    //classic
    if supply_left < 6 && pending == 0 {
        return true;
    }
    //supply block close
    if supply_left < 6 && pending < 2 && supply_used > 30 {
        return true;
    }
    //supply block too close
    if supply_used > 45 && supply_left < 3 && pending < 3 {
        return true;
    }
    //lategame
    if supply_cap > 40 && supply_left < 8 && pending == 0 {
        return true;
    }
    //close middle ramp
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
    if let Some(depots) = bot.ramps.my.corner_depots() {
        for depot in depots {
            if bot.units.my.structures.closer(2.0, depot).is_empty() {
                return Target::Pos(depot);
            }
        }
    }
    if let Some(depot) = bot.ramps.my.depot_in_middle() {
        if bot.units.my.structures.closer(2.0, depot).is_empty() {
            return Target::Pos(depot);
        }
    }
    for base in bot.units.my.townhalls.clone() {
        let base_depots_pos = base.position().towards(bot.enemy_start, -10.0);
        if let Some(depot) =
            bot.find_placement(UnitTypeId::SupplyDepot, base_depots_pos, Default::default())
        {
            return Target::Pos(depot);
        }
    }
    Target::None
}
