use std::collections::HashMap;

use crate::Nikolaj;
use rust_sc2::prelude::*;

pub(crate) fn decide_build_order(bot: &mut Nikolaj) -> Vec<UnitTypeId> {
    const BIO: &'static [UnitTypeId] = &[
        UnitTypeId::Marine,
        UnitTypeId::Marauder,
        UnitTypeId::Reaper,
        UnitTypeId::Ghost,
    ];
    let mut build_order: Vec<UnitTypeId> = vec![];

    if bot.enemy_race == Race::Terran {
        build_order = vec![
            UnitTypeId::SiegeTank,
            UnitTypeId::VikingFighter,
            UnitTypeId::Marine,
        ];
    }

    //add support units
    //raven
    if bot.enemy_cloaking && bot.my_unit_count(UnitTypeId::Raven) == 0 {
        build_order.push(UnitTypeId::Raven);
    }
    //medivac
    let bio_amount = bot.units.my.units.of_types(&BIO).len();
    if bio_amount > 6 && bot.my_unit_count(UnitTypeId::Medivac) * 5 < bio_amount {
        build_order.push(UnitTypeId::Medivac);
    }

    //starters
    for unit in [UnitTypeId::Reaper, UnitTypeId::Banshee].iter() {
        if bot.already_pending(*unit) == 0 && bot.my_units_memory.of_type(*unit).is_empty() {
            build_order.insert(0, *unit);
        }
    }
    build_order
}

pub(crate) fn execute_build_order(bot: &mut Nikolaj) {
    let unit_source: HashMap<UnitTypeId, UnitTypeId> = [
        (UnitTypeId::Marine, UnitTypeId::Barracks),
        (UnitTypeId::Marauder, UnitTypeId::Barracks),
        (UnitTypeId::Reaper, UnitTypeId::Barracks),
        (UnitTypeId::Ghost, UnitTypeId::Barracks),
        (UnitTypeId::Hellion, UnitTypeId::Factory),
        (UnitTypeId::WidowMine, UnitTypeId::Factory),
        (UnitTypeId::Cyclone, UnitTypeId::Factory),
        (UnitTypeId::SiegeTank, UnitTypeId::Factory),
        (UnitTypeId::Thor, UnitTypeId::Factory),
        (UnitTypeId::VikingFighter, UnitTypeId::Starport),
        (UnitTypeId::Banshee, UnitTypeId::Starport),
        (UnitTypeId::Liberator, UnitTypeId::Starport),
        (UnitTypeId::Raven, UnitTypeId::Starport),
        (UnitTypeId::Medivac, UnitTypeId::Starport),
        (UnitTypeId::Battlecruiser, UnitTypeId::Starport),
    ]
    .iter()
    .cloned()
    .collect();

    let build_order = decide_build_order(bot);
    for unit in build_order {
        if let Some(source) = unit_source.get(&unit) {
            if tech_requirements_met(bot, unit) {
                
            }
        }
    }
}

fn tech_requirements_met(bot: &Nikolaj, unit: UnitTypeId) -> bool {
    match unit {
        UnitTypeId::Marine => !bot
            .units
            .my
            .structures
            .of_type_including_alias(UnitTypeId::Barracks)
            .ready()
            .is_empty(),
        UnitTypeId::Marauder => !bot
            .units
            .my
            .structures
            .of_type_including_alias(UnitTypeId::Barracks)
            .ready()
            .is_empty(),
        UnitTypeId::Reaper => !bot
            .units
            .my
            .structures
            .of_type_including_alias(UnitTypeId::Barracks)
            .ready()
            .is_empty(),
        UnitTypeId::Ghost => {
            !bot.units
                .my
                .structures
                .of_type_including_alias(UnitTypeId::Barracks)
                .ready()
                .is_empty()
                && !bot
                    .units
                    .my
                    .structures
                    .of_type(UnitTypeId::GhostAcademy)
                    .ready()
                    .is_empty()
        }
        UnitTypeId::Hellion => !bot
            .units
            .my
            .structures
            .of_type_including_alias(UnitTypeId::Factory)
            .ready()
            .is_empty(),
        UnitTypeId::WidowMine => !bot
            .units
            .my
            .structures
            .of_type_including_alias(UnitTypeId::Factory)
            .ready()
            .is_empty(),
        UnitTypeId::Cyclone => !bot
            .units
            .my
            .structures
            .of_type_including_alias(UnitTypeId::Factory)
            .ready()
            .is_empty(),
        UnitTypeId::SiegeTank => !bot
            .units
            .my
            .structures
            .of_type_including_alias(UnitTypeId::Factory)
            .ready()
            .is_empty(),
        UnitTypeId::Thor => {
            !bot.units
                .my
                .structures
                .of_type_including_alias(UnitTypeId::Factory)
                .ready()
                .is_empty()
                && !bot
                    .units
                    .my
                    .structures
                    .of_type(UnitTypeId::Armory)
                    .ready()
                    .is_empty()
        }
        UnitTypeId::VikingFighter => !bot
            .units
            .my
            .structures
            .of_type_including_alias(UnitTypeId::Starport)
            .ready()
            .is_empty(),
        UnitTypeId::Banshee => !bot
            .units
            .my
            .structures
            .of_type_including_alias(UnitTypeId::Starport)
            .ready()
            .is_empty(),
        UnitTypeId::Liberator => !bot
            .units
            .my
            .structures
            .of_type_including_alias(UnitTypeId::Starport)
            .ready()
            .is_empty(),
        UnitTypeId::Raven => !bot
            .units
            .my
            .structures
            .of_type_including_alias(UnitTypeId::Starport)
            .ready()
            .is_empty(),
        UnitTypeId::Medivac => !bot
            .units
            .my
            .structures
            .of_type_including_alias(UnitTypeId::Starport)
            .ready()
            .is_empty(),
        UnitTypeId::Battlecruiser => {
            !bot.units
                .my
                .structures
                .of_type_including_alias(UnitTypeId::Starport)
                .ready()
                .is_empty()
                && !bot
                    .units
                    .my
                    .structures
                    .of_type(UnitTypeId::FusionCore)
                    .ready()
                    .is_empty()
        }
        _ => false,
    }
}
