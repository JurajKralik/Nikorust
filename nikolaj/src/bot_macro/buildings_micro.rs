use crate::Nikolaj;
use rust_sc2::prelude::*;

pub(crate) fn cancel_buildings(bot: &mut Nikolaj) {
    for building in bot.units.my.structures.not_ready() {
        if let (Some(health_max), Some(health), Some(health_percentage)) = (
            building.health_max(),
            building.health(),
            building.health_percentage(),
        ) {
            if bot.worker_rush
                || (health_max > 700 && health < 50 && building.build_progress() > 0.1)
                || (health_max < 700 && health_percentage < 0.1 && building.build_progress() > 0.2)
            {
                building.cancel_building(false)
            }
        }
    }
}

pub(crate) fn depot_micro(bot: &mut Nikolaj) {
    let safe_dist = 8.0;

    //opened
    for depot in bot
        .units
        .my
        .structures
        .of_type(UnitTypeId::SupplyDepotLowered)
    {
        for unit in bot.units.enemy.units.closer(safe_dist, depot.position()) {
            if !unit.is_flying() {
                depot.command(AbilityId::MorphSupplyDepotLower, Target::None, false);
                break;
            }
        }
    }
    //closed
    for depot in bot
        .units
        .my
        .structures
        .of_type(UnitTypeId::SupplyDepot)
        .ready()
    {
        let mut open = true;
        for unit in bot.units.enemy.units.closer(safe_dist, depot.position()) {
            if !unit.is_flying() {
                open = false;
                break;
            }
        }
        if open {
            depot.command(AbilityId::MorphSupplyDepotRaise, Target::None, false);
        }
    }
}

pub(crate) fn bunker_micro(bot: &mut Nikolaj) {
    for bunker in bot.units.my.structures.of_type(UnitTypeId::Bunker).ready() {
        const PRIORITY_ZERO: &'static [UnitTypeId] = &[
            UnitTypeId::Larva,
            UnitTypeId::Egg,
            UnitTypeId::AdeptPhaseShift,
            UnitTypeId::Interceptor,
            UnitTypeId::Overlord,
            UnitTypeId::OverlordCocoon,
            UnitTypeId::Overseer,
            UnitTypeId::OverseerSiegeMode,
            UnitTypeId::OverlordTransport,
            UnitTypeId::Observer,
            UnitTypeId::ObserverSiegeMode,
            UnitTypeId::Medivac,
            UnitTypeId::Phoenix,
        ];
        let enemies = bot
            .units
            .enemy
            .units
            .exclude_types(&PRIORITY_ZERO)
            .closer(20.0, bunker.position());
        //unload
        if let Some(cargo_space_taken) = bunker.cargo_space_taken() {
            if cargo_space_taken > 0 && enemies.is_empty() && bot.offensive_point.is_some() {
                bunker.command(AbilityId::UnloadAllBunker, Target::None, false);
                continue;
            }
        }
        //targeting
        if !enemies.is_empty() {
            let mut lowest: Option<Unit> = None;

            for enemy in enemies {
                if let Some(target) = lowest.clone() {
                    if let (Some(target_health), Some(enemy_health)) =
                        (target.health(), enemy.health())
                    {
                        if enemy_health < target_health {
                            lowest = Some(target);
                        }
                    }
                }
            }
            if let Some(target) = lowest {
                bunker.attack(Target::Tag(target.tag()), false);
            }
        }
    }
}

pub(crate) fn set_rally_points(bot: &mut Nikolaj) {
    const PRODUCTION: &'static [UnitTypeId] = &[
            UnitTypeId::Barracks,
            UnitTypeId::Factory,
            UnitTypeId::Starport
        ];
    for building in bot.units.my.structures.of_types(&PRODUCTION).ready() {
        if building.rally_targets().is_empty() && !bot.units.my.townhalls.is_empty() {
            if let Some(townhall) = bot.units.my.townhalls.closest(building.position()) {
                building.command(AbilityId::RallyBuilding, Target::Tag(townhall.tag()), false);
            }
        }
    }
}
