use std::any::Any;

use crate::Nikolaj;
use rust_sc2::{bot, prelude::*};

pub(crate) fn units_memory(bot: &mut Nikolaj) {
    const MEMORY_IGNORETYPES: &'static [UnitTypeId] = &[
        UnitTypeId::SCV,
        UnitTypeId::Drone,
        UnitTypeId::DroneBurrowed,
        UnitTypeId::Probe,
        UnitTypeId::AdeptPhaseShift,
        UnitTypeId::Observer,
        UnitTypeId::Overlord,
        UnitTypeId::OverlordTransport,
        UnitTypeId::Overseer,
        UnitTypeId::OverlordCocoon,
        UnitTypeId::Larva,
        UnitTypeId::Egg,
        UnitTypeId::BroodLordCocoon,
        UnitTypeId::BanelingCocoon,
        UnitTypeId::LurkerMPEgg,
        UnitTypeId::Changeling,
        UnitTypeId::Broodling,
        UnitTypeId::LocustMP,
        UnitTypeId::LocustMPFlying,
        UnitTypeId::MULE,
        UnitTypeId::ChangelingMarine,
        UnitTypeId::ChangelingMarineShield,
    ];
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

pub(crate) fn get_enemy_army_supply(bot: &mut Nikolaj) -> f32 {
    let mut army_supply = 0.0;
    for unit in bot.enemy_units_memory.clone() {
        army_supply += unit.supply_cost();
    }
    army_supply
}

pub(crate) fn cheese_detection(bot: &mut Nikolaj) {
    //worker rush
    if bot.time < 200.0 || bot.worker_rush {
        let mut enemy_workers = 0;
        for enemy in bot.units.enemy.units.clone() {
            if enemy.position().distance(bot.enemy_start)
                < enemy.position().distance(bot.start_location)
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
    if bot.time < 350.0 || bot.contain_rush {
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
            if bot.units.enemy.units.get(ramp_blocker).unwrap().distance(bot.ramps.my.depot_in_middle().unwrap()) > 12.0 {
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
            if !bot.units.enemy.units.is_empty() {
                for pos in corner_depots {
                    if bot.units.enemy.units.closest(pos) {
                        
                    }
                }                
            }            
        }
    }
}
