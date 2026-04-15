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
    let already_pending = bot.already_pending(UnitTypeId::Bunker) > 0;
    let under_construction = bot.construction_info.is_under_construction(UnitTypeId::Bunker);
    let savings_for_expansion = bot.macro_manager.expand_priority && bot.get_unit_cost(UnitTypeId::Bunker).minerals > bot.minerals.saturating_sub(400);
    let cannot_afford = !bot.can_afford(UnitTypeId::Bunker, false);
    let no_barracks = bot.units.my.structures.of_type_including_alias(UnitTypeId::Barracks).ready().is_empty();
    let already_have_bunker = bot.structure_count(UnitTypeId::Bunker) + bot.already_pending(UnitTypeId::Bunker) > 0;
    let not_needed_for_one_base = bot.units.my.townhalls.ready().len() > 1 || bot.strategy_data.enemy_flooding;

    if already_pending 
        || under_construction
        || savings_for_expansion
        || cannot_afford
        || no_barracks
        || already_have_bunker
        || not_needed_for_one_base
    {
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
    for bunker in bot.units.my.structures.of_type(UnitTypeId::Bunker).ready() {
        set_rally_point(bot, &bunker);
        if in_danger(bot, &bunker) {
            request_bio(bot, &bunker);
        } else {
            bot.combat_info.bunker_requests.remove(&bunker.tag());
        }
    }
}


fn set_rally_point(bot: &Nikolaj, bunker: &Unit) {
    if !bunker.rally_targets().is_empty() {
        return;
    }

    if let Some(base) = bot.units.my.townhalls.closest(bunker.position()) {
        bunker.smart(Target::Pos(base.position()), false);
    }
}

const BIO: &[UnitTypeId] = &[
    UnitTypeId::Marine,
    UnitTypeId::Marauder,
    // UnitTypeId::Reaper,
    UnitTypeId::Ghost
];


fn in_danger(bot: &Nikolaj, bunker: &Unit) -> bool {
    let nearby_enemies = bot.units.enemy.units.in_real_range(bunker, 10.0);
    !nearby_enemies.is_empty()
}

fn request_bio(bot: &mut Nikolaj, bunker: &Unit) {
    let mut cargo_left = bunker.cargo_left().clone();

    if cargo_left == 0 {
        return;
    }

    let mut bunker_requests = bot.combat_info.get_bunker_requests_for_bunker(bunker.tag()).unwrap_or_default();
    let mut dead_requests = Vec::new();
    for request in bunker_requests.iter() {
        if let None = bot.units.my.units.iter().find_tag(*request) {
            dead_requests.push(*request);
        }
    }
    for dead in dead_requests.iter() {
        bunker_requests.retain(|&tag| tag != *dead);
    }
    
    let mut nearby_bio: Vec<_> = bot.units.my.units
        .of_types(&BIO)
        .ready()
        .iter()
        .filter(|unit| unit.distance(bunker.position()) < 15.0)
        .cloned()
        .collect();
        
    nearby_bio.sort_by(|a, b| {
        let dist_a = bunker.position().distance(a.position());
        let dist_b = bunker.position().distance(b.position());
        dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
    });
    
    for unit in nearby_bio {
        if cargo_left == 0 {
            break;
        }
        if unit.cargo_size() > cargo_left {
            continue;
        }        

        cargo_left -= unit.cargo_size();
        if bunker_requests.contains(&unit.tag()) {
            continue;
        }
        bunker_requests.push(unit.tag());
    }
}