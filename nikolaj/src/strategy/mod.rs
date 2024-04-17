use crate::Nikolaj;
use rust_sc2::prelude::*;

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
        if !bot.enemy_unit_types_memory.contains_key(&enemy.type_id().clone()) {
            bot.enemy_unit_types_memory.insert(enemy.type_id().clone(), 1);
        //+1 to type
        } else {
            bot.enemy_unit_types_memory.insert(enemy.type_id().clone(), bot.enemy_unit_types_memory[&enemy.type_id().clone()] + 1);
        }
        //if Archon found, delete all templars
        if enemy.type_id() == UnitTypeId::Archon {
            clean_templar = true;
        }
        bot.enemy_units_memory.push(enemy.clone());
    }
    if clean_templar {
        for enemy in bot.units.enemy.units.clone() {
            if enemy.type_id() == UnitTypeId::HighTemplar || enemy.type_id() == UnitTypeId::DarkTemplar{
                bot.enemy_units_memory.remove(enemy.tag());
            }
        }
    }
}
    