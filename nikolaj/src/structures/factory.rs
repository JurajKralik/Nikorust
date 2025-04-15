use crate::helpers::construction::*;
use crate::Nikolaj;
use rust_sc2::prelude::*;

pub fn construct_factory(bot: &mut Nikolaj) {
    if bot.already_pending(UnitTypeId::Factory) > 0 {
        return;
    }
    if !bot.can_afford(UnitTypeId::Factory, false) {
        return;
    }
    if bot.structure_count(UnitTypeId::Barracks)
        + bot.structure_count(UnitTypeId::BarracksFlying)
        == 0
    {
        return;
    }

    let amount = bot.structure_count(UnitTypeId::Factory)
        + bot.structure_count(UnitTypeId::FactoryFlying)
        + bot.already_pending(UnitTypeId::Factory);

    // Conditions
    // Max 4 factories
    if amount >= 4 {
        return;
    }

    // Factory flying
    if !bot
        .units
        .my
        .structures
        .of_type(UnitTypeId::FactoryFlying)
        .is_empty()
    {
        return;
    }

    // Factory production ongoing
    if !bot
        .units
        .my
        .structures
        .of_type(UnitTypeId::Factory)
        .idle()
        .is_empty()
    {
        return;
    }

    // Factory and Starport first
    if bot.structure_count(UnitTypeId::Factory) > 0
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
        build(bot, position, UnitTypeId::Factory);
        return;
    }

    // Random position
    for base in bot.units.my.townhalls.clone() {
        let position = base.position().towards(bot.enemy_start, 4.0);
        if bot.can_place(UnitTypeId::Factory, position) {
            build(bot, position, UnitTypeId::Factory);
            return;
        }
    }
}
