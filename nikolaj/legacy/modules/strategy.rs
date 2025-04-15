use crate::Nikolaj;
use rust_sc2::prelude::*;
use crate::params::*;

pub fn units_memory(bot: &mut Nikolaj) {
    let mut clean_templar = false;

    //add units to memory
    for enemy in bot.units.enemy.units.clone() {
        if MEMORY_IGNORETYPES.contains(&enemy.type_id()) {
            continue;
        }
        if bot.enemy_units_memory.contains_tag(enemy.tag()) {
            if enemy.type_id() == bot.enemy_units_memory.get(enemy.tag()).unwrap().type_id() {
                continue;
            } else {
                //delete if same tag, but different unit - morph, merge etc.
                bot.enemy_units_memory.remove(enemy.tag());
            }
        }

        //add type
        if !bot
            .enemy_unit_types_memory
            .contains_key(&enemy.type_id().clone())
        {
            bot.enemy_unit_types_memory
                .insert(enemy.type_id().clone(), 1);
        //+1 to type
        } else {
            bot.enemy_unit_types_memory.insert(
                enemy.type_id().clone(),
                bot.enemy_unit_types_memory[&enemy.type_id().clone()] + 1,
            );
        }
        //if Archon found, delete all templars
        if enemy.type_id() == UnitTypeId::Archon {
            clean_templar = true;
        }
        bot.enemy_units_memory.push(enemy.clone());
    }
    if clean_templar {
        for enemy in bot.units.enemy.units.clone() {
            if enemy.type_id() == UnitTypeId::HighTemplar
                || enemy.type_id() == UnitTypeId::DarkTemplar
            {
                bot.enemy_units_memory.remove(enemy.tag());
            }
        }
    }
    //remove units that died this frame from memory
    for enemy in bot.state.observation.raw.dead_units.clone() {
        if bot.enemy_units_memory.contains_tag(enemy) {
            bot.enemy_units_memory.remove(enemy);
        }
    }

    //add structures to memory
    for structure in bot.units.enemy.structures.clone() {
        if bot.enemy_structures_memory.contains_tag(structure.tag()) {
            continue;
        }
        bot.enemy_structures_memory.push(structure.clone());
        //types
        if !bot
            .enemy_structure_types_memory
            .contains_key(&structure.type_id().clone())
        {
            bot.enemy_structure_types_memory
                .insert(structure.type_id().clone(), 1);
        } else {
            bot.enemy_structure_types_memory.insert(
                structure.type_id().clone(),
                bot.enemy_structure_types_memory[&structure.type_id().clone()] + 1,
            );
        }
    }
    //my units
    for my_unit in bot.units.my.units.clone() {
        if MEMORY_IGNORETYPES.contains(&my_unit.type_id()) {
            continue;
        }
        if bot.my_units_memory.contains_tag(my_unit.tag()) {
            if my_unit.type_id() == bot.my_units_memory.get(my_unit.tag()).unwrap().type_id() {
                continue;
            } else {
                bot.my_units_memory.remove(my_unit.tag());
            }
        }
        //types
        if !bot
            .my_unit_types_memory
            .contains_key(&my_unit.type_id().clone())
        {
            bot.my_unit_types_memory
                .insert(my_unit.type_id().clone(), 1);
        } else {
            bot.my_unit_types_memory.insert(
                my_unit.type_id().clone(),
                bot.my_unit_types_memory[&my_unit.type_id().clone()] + 1,
            );
        }
        bot.my_units_memory.push(my_unit.clone());
    }
    //my structures
    for my_structure in bot.units.my.structures.clone() {
        if bot.my_structures_memory.contains_tag(my_structure.tag()) {
            continue;
        }
        bot.my_structures_memory.push(my_structure.clone());
        //types
        if !bot
            .my_structure_types_memory
            .contains_key(&my_structure.type_id().clone())
        {
            bot.my_structure_types_memory
                .insert(my_structure.type_id().clone(), 1);
        } else {
            bot.my_structure_types_memory.insert(
                my_structure.type_id().clone(),
                bot.my_structure_types_memory[&my_structure.type_id().clone()] + 1,
            );
        }
    }
}

pub fn cheese_detection(bot: &mut Nikolaj) {
    //worker rush
    if bot.time < 200.0 || bot.worker_rush {
        let mut enemy_workers = 0;
        for enemy in bot.units.enemy.units.clone() {
            if enemy.position().distance(bot.enemy_start)
                > enemy.position().distance(bot.start_location)
            {
                if enemy.is_worker() {
                    enemy_workers += 1;
                }
            }
            //start
            if enemy_workers > 4 {
                println!("Worker rush detected {:#?}", bot.time);
                bot.worker_rush = true;
                break;
            }
        }
        //end
        if enemy_workers == 0 {
            if bot.worker_rush {
                println!("Worker rush ended {:#?}", bot.time);
            }
            bot.worker_rush = false;
        }
    }
    //contain rush
    if bot.time < 60.0 * 5.0 || bot.contain_rush {
        if !bot.units.enemy.structures.is_empty() {
            for structure in bot.units.enemy.structures.clone() {
                if structure.position().distance(bot.start_location)
                    < structure.position().distance(bot.enemy_start)
                {
                    if !bot.contain_rush {
                        if structure.type_id() == UnitTypeId::CommandCenter
                            || structure.type_id() == UnitTypeId::CommandCenterFlying
                            || structure.type_id() == UnitTypeId::PlanetaryFortress
                        {
                            println!("Planetary fortress rush detected {:#?}", bot.time);
                        } else if structure.type_id() == UnitTypeId::PhotonCannon {
                            println!("Canon rush detected {:#?}", bot.time);
                        } else {
                            println!("Contain rush detected {:#?}", bot.time);
                        }
                        bot.contain_rush = true;
                    }
                    break;
                }
            }
        } else {
            if bot.contain_rush {
                println!("Contain rush ended {:#?}", bot.time);
            }
            bot.contain_rush = false;
        }
    }
    //ramp depot block
    if let Some(ramp_blocker) = bot.ramp_blocker {
        //end
        if !bot.units.enemy.units.contains_tag(ramp_blocker) {
            bot.ramp_blocker = None;
            bot.ramp_blocker_timer = 0;
        } else {
            if bot
                .units
                .enemy
                .units
                .get(ramp_blocker)
                .unwrap()
                .distance(bot.ramps.my.depot_in_middle().unwrap())
                > 12.0
            {
                bot.ramp_blocker = None;
                bot.ramp_blocker_timer = 0;
            }
        }
    } else if bot
        //ramp not closed yet
        .units
        .my
        .structures
        .of_type(UnitTypeId::SupplyDepot)
        .len()
        + bot
            .units
            .my
            .structures
            .of_type(UnitTypeId::SupplyDepotLowered)
            .len()
        < 2
    {
        //start
        let mut possible_block = false;
        if let Some(corner_depots) = bot.ramps.my.corner_depots() {
            for pos in corner_depots {
                if let Some(closest) = bot.units.enemy.units.closest(pos) {
                    if closest.distance(pos) < 2.0 {
                        if bot.ramp_blocker_timer > 20 {
                            bot.ramp_blocker = Some(closest.tag());
                            println!("Ramp block detected {:#?}", bot.time);
                        }
                        possible_block = true;
                    }
                }
            }
        }
        if possible_block {
            bot.ramp_blocker_timer += 1;
        } else {
            bot.ramp_blocker_timer = 0;
        }
    }
    //flooding
    if bot.time < 60.0 * 5.0 || bot.flooding {
        if bot.units.enemy.units.closer(30.0, bot.idle_point).len() > 5 && !bot.flooding {
            println!("Flooding detected {:#?}", bot.time);
            bot.flooding = true;
        } else if bot.flooding
            && (bot.units.my.townhalls.len() > 1
                || !bot
                    .units
                    .my
                    .structures
                    .of_type(UnitTypeId::Bunker)
                    .ready()
                    .is_empty())
        {
            println!("Flooding ended {:#?}", bot.time);
            bot.flooding = false;
        }
    }
}

pub fn enemy_macro_strategy(bot: &mut Nikolaj) {
    for enemy in bot.enemy_units_memory.clone() {
        //cloak
        if !bot.enemy_cloaking && CLOAK_AND_BURROW.contains(&enemy.type_id()) {
            bot.enemy_cloaking = true;
            println!(
                "Cloaking unit {:#?} detected {:#?}",
                enemy.type_id(),
                bot.time
            );
        }
        //fliers
        if !bot.enemy_fliers
            && ((enemy.type_id() == UnitTypeId::Colossus || enemy.is_flying())
                && !FLIERS_IGNORE.contains(&enemy.type_id()))
        {
            bot.enemy_fliers = true;
            println!(
                "Flying unit {:#?} detected {:#?}",
                enemy.type_id(),
                bot.time
            );
        }
        //heavy fliers
        if !bot.enemy_heavy_fliers && HEAVY_FLIERS.contains(&enemy.type_id()) {
            bot.enemy_heavy_fliers = true;
            bot.enemy_fliers = true;
            println!(
                "Heavy flying unit {:#?} detected {:#?}",
                enemy.type_id(),
                bot.time
            );
        }
    }
    for enemy_type in bot.enemy_structure_types_memory.keys() {
        //cloak
        if !bot.enemy_cloaking && CLOAK_STRUCTURES.contains(enemy_type) {
            bot.enemy_cloaking = true;
            println!(
                "Cloaking structure {:#?} detected {:#?}",
                enemy_type, bot.time
            );
        }
        //fliers
        if !bot.enemy_fliers && FLYING_PRODUCTION_STRUCTURES.contains(enemy_type) {
            bot.enemy_fliers = true;
            println!(
                "Flying production structure {:#?} detected {:#?}",
                enemy_type, bot.time
            );
        }
        if !bot.enemy_heavy_fliers && HEAVY_FLYING_PRODUCTION_STRUCTURES.contains(enemy_type) {
            bot.enemy_heavy_fliers = true;
            bot.enemy_fliers = true;
            println!(
                "Heavy flying production structure {:#?} detected {:#?}",
                enemy_type, bot.time
            );
        }
    }
}

pub fn set_idle_point(bot: &mut Nikolaj) {
    //no base
    if bot.units.my.townhalls.is_empty() {
        if let Some(structure) = bot.units.my.structures.closest(bot.start_location) {
            bot.idle_point = structure.position();
            return;
        }
        //main base
        if bot.units.my.townhalls.ready().len() == 1 {
            if let (Some(main_base), Some(barracks_in_middle)) = (
                bot.units.my.townhalls.closest(bot.start_location),
                bot.ramps.my.barracks_in_middle(),
            ) {
                if main_base.distance(bot.start_location) < 2.0 {
                    bot.idle_point = barracks_in_middle.towards(main_base.position(), 5.0);
                    return;
                }
            }
        }
        //most frontal base
        if !bot.bases.is_empty() {
            let mut front_base: Option<Unit> = None;
            let mut front_base_distance = 0.0;
            bot.bases.sort();

            for base in bot.bases.clone() {
                if let Some(structure) = bot.units.my.structures.get(base) {
                    let base_distance = structure.distance(bot.enemy_start);
                    //first base
                    if front_base.is_none() {
                        front_base = Some(structure.clone());
                        front_base_distance = base_distance;
                        continue;
                    }
                    //base closest to enemy base. -15 for back third base
                    if base_distance - 15.0 < front_base_distance {
                        front_base = Some(structure.clone());
                        front_base_distance = base_distance;
                    }
                }
            }

            if front_base.is_some() {
                bot.idle_point = front_base.unwrap().position().towards(bot.enemy_start, 5.0);
                return;
            }
        }
        bot.idle_point = bot.start_location;
    }
}

pub fn set_main_army_point(bot: &mut Nikolaj) {
    //position of unit closest to the army center
    let all_army = bot.units.my.units.exclude_types(&EXCLUDE_MAIN_ARMY).clone();
    if let Some(army_center) = all_army.center() {
        if let Some(centered) = all_army.closest(army_center) {
            bot.main_army_point = Some(centered.clone().position());
            return;
        }
    }
    //none
    bot.main_army_point = None;
}

pub fn set_defensive_point(bot: &mut Nikolaj) {
    bot.defensive_point = None;

    if let Some(enemy) = bot
        .units
        .enemy
        .units
        .exclude_types(&DEFENSIVE_IGNORETYPES)
        .closest(bot.start_location)
    {
        let mut enemy_base = bot.enemy_start.clone();
        if let Some(closest_structure) = bot.units.enemy.structures.closest(bot.start_location) {
            enemy_base = closest_structure.position();
        }
        if let Some(structure) = bot.units.my.structures.closest(enemy) {
            if enemy.distance(enemy_base) > enemy.distance(structure) * 2.0 {
                bot.defensive_point = Some(enemy.position());
            }
        }
    }
}

fn assemble_offensive(bot: &mut Nikolaj) {
    bot.assembling = bot.time + 15.0;
    let mut enemy_base = bot.enemy_start.clone();
    if let Some(closest_structure) = bot.units.enemy.structures.closest(bot.start_location) {
        enemy_base = closest_structure.position();
    }

    //reassemble
    let all_army = bot.units.my.units.exclude_types(&EXCLUDE_MAIN_ARMY).clone();
    if let (Some(main_army_point), Some(army_center)) = (bot.main_army_point, all_army.center()) {
        if main_army_point.distance(bot.idle_point) < main_army_point.distance(enemy_base) && main_army_point.distance(army_center) > 15.0 {
            bot.offensive_point = Some(main_army_point.clone());
            return;
        }
    }

    //attack enemy base
    bot.offensive_point = Some(enemy_base.clone());
}

fn get_enemy_army_supply(bot: &mut Nikolaj) -> f32 {
    let mut army_supply = 0.0;
    for unit in bot.enemy_units_memory.clone() {
        army_supply += unit.supply_cost();
    }
    army_supply
}

pub fn set_offensive_point(bot: &mut Nikolaj) {
    //defense priority
    if bot.defensive_point.is_some() {
        bot.offensive_point = None;
        return;
    }

    //start push
    if (bot.supply_army > 12 && bot.supply_army as f32 > get_enemy_army_supply(bot))
        || bot.supply_used > 170
        || bot.assembling > bot.time
    {
        assemble_offensive(bot);
        return;
    }

    //keep pushing
    if let (Some(main_army_point), Some(offensive_point)) = (bot.main_army_point, bot.offensive_point) {
        if bot.supply_army > 13 && main_army_point.distance(offensive_point) < main_army_point.distance(bot.idle_point) {
            assemble_offensive(bot);
            return;
        } 
    }
    bot.offensive_point = None;
}

pub fn set_repair_point(bot: &mut Nikolaj) {
    bot.repair_point = bot.start_location;

    let mut maxed_base:Option<Unit> = None;
    let mut max_scvs = 0;

    for base in bot.units.my.townhalls.ready() {
        let scvs = bot.units.my.workers.closer(20.0, base.position()).len();
        //first
        if maxed_base.is_none() {
            maxed_base = Some(base.clone());
            max_scvs = scvs;
        } else if max_scvs < scvs {
            maxed_base = Some(base.clone());
            max_scvs = scvs;
        }
    }
    if let Some(base) = maxed_base {
        if let Some(mineral) = bot.units.mineral_fields.closest(base.position()) {
            if base.clone().distance(mineral) < 9.0 {
                bot.repair_point = base.clone().position().towards(mineral.clone().position(), -8.0);
                return;
            }
        }
        bot.repair_point = base.position().towards(bot.enemy_start, -4.0);
    }
}

pub fn set_harass_point(bot: &mut Nikolaj) {
    bot.harass_point = bot.enemy_start.towards(bot.start_location, -5.0);
}