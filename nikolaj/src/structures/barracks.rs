use crate::helpers::construction::*;
use crate::Nikolaj;
use rust_sc2::prelude::*;

pub fn construct_barracks(bot: &mut Nikolaj) {
    if bot.already_pending(UnitTypeId::Barracks) > 0 {
        return;
    }
    if !bot.can_afford(UnitTypeId::Barracks, false) {
        return;
    }
    if bot.structure_count(UnitTypeId::SupplyDepot)
        + bot.structure_count(UnitTypeId::SupplyDepotLowered)
        == 0
    {
        return;
    }

    let amount = bot.structure_count(UnitTypeId::Barracks)
        + bot.structure_count(UnitTypeId::BarracksFlying)
        + bot.already_pending(UnitTypeId::Barracks);

    // Conditions
    // Max 4 barracks
    if amount >= 4 {
        return;
    }

    // Barracks flying
    if !bot
        .units
        .my
        .structures
        .of_type(UnitTypeId::BarracksFlying)
        .is_empty()
    {
        return;
    }

    // Barracks production ongoing
    if !bot
        .units
        .my
        .structures
        .of_type(UnitTypeId::Barracks)
        .idle()
        .is_empty()
    {
        return;
    }

    // Factory and Starport first
    if bot.structure_count(UnitTypeId::Barracks) > 0
        && bot.structure_count(UnitTypeId::Factory) + bot.structure_count(UnitTypeId::FactoryFlying)
            == 0
        && bot.structure_count(UnitTypeId::Starport)
            + bot.structure_count(UnitTypeId::StarportFlying)
            == 0
        && bot.minerals < 300
    {
        return;
    }

    // Positioning
    // Middle ramp
    if let Some(barracks_position) = bot.ramps.my.barracks_in_middle() {
        if bot.can_place(UnitTypeId::Barracks, barracks_position) {
            build(bot, barracks_position, UnitTypeId::Barracks);
            return;
        }
    }

    // On grid
    if let Some(position) = get_placement_on_grid(bot) {
        build(bot, position, UnitTypeId::Barracks);
        return;
    }

    // Random position
    for base in bot.units.my.townhalls.clone() {
        let position = base.position().towards(bot.enemy_start, 4.0);
        if bot.can_place(UnitTypeId::Barracks, position) {
            build(bot, position, UnitTypeId::Barracks);
            return;
        }
    }
}

pub fn barracks_control(bot: &mut Nikolaj) {
    for barracks in bot.units.my.structures.of_type(UnitTypeId::Barracks).idle() {
        if let Some(unit_type) = bot.barracks_priority {
            // Tech lab for Marauder and Ghost
            if [UnitTypeId::Marauder, UnitTypeId::Ghost].contains(&unit_type) {
                // Build tech lab
                if !barracks.has_techlab(){
                    let addon_position = barracks.position().clone().offset(2.5, -0.5);
                    if bot.can_place(UnitTypeId::SupplyDepot, addon_position) {
                        barracks.command(AbilityId::BuildTechLabBarracks, Target::None, false);
                    } else {
                        barracks.command(AbilityId::LiftBarracks, Target::None, false);
                    }
                // Train
                } else {
                    barracks.train(unit_type, false);
                }
            } else {
                // Train
                barracks.train(unit_type, false);
            }
        // No priority
        } else {
            break;
        }
    }

    // Land barracks
    for barracks in bot.units.my.structures.of_type(UnitTypeId::BarracksFlying).idle() {
        // On grid
        if let Some(position) = get_placement_on_grid(bot) {
            barracks.command(AbilityId::LandBarracks, Target::Pos(position), false);
        }
        // Random position
        for base in bot.units.my.townhalls.clone() {
            let position = base.position().towards(bot.enemy_start, 4.0);
            if bot.can_place(UnitTypeId::Barracks, position) {
                barracks.command(AbilityId::LandBarracks, Target::Pos(position), false);
            }
        }
    }
}