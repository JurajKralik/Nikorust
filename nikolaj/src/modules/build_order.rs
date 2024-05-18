use crate::construct;
use crate::params::*;
use crate::Nikolaj;
use rust_sc2::prelude::*;

pub struct BuildOrder {
    pub bot: Nikolaj,
    pub units: Vec<UnitTypeId>,
    pub next_multiproduction_structure: Option<UnitTypeId>,
}

fn decide_units(bot: &mut Nikolaj) -> Vec<UnitTypeId> {
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
    if bot.iteration > 1 {
        for unit in [UnitTypeId::Reaper, UnitTypeId::Banshee].iter() {
            if bot.already_pending(unit.clone()) == 0
                && bot.my_units_memory.of_type(unit.clone()).is_empty()
            {
                build_order.insert(0, unit.clone());
            }
        }
    }
    build_order
}

fn decide_next_structure(bot: &mut Nikolaj, units: Vec<UnitTypeId>) -> Option<UnitTypeId> {
    //empty build order
    if units.len() == 0 {
        return None;
    }

    if let Some(first_production) = UNIT_SOURCE.get(&units[0].clone()) {
        //single production
        if units.len() == 1 {
            return Some(first_production.clone());
        }
        if let Some(second_production) = UNIT_SOURCE.get(&units[0].clone()) {
            let first_production_amount = bot.my_unit_count(first_production.clone());
            let second_production_amount = bot.my_unit_count(second_production.clone());
            if first_production_amount <= second_production_amount {
                return Some(first_production.clone());
            } else {
                return Some(second_production.clone());
            }
        }
    }
    None
}

impl BuildOrder {
    pub fn new(_bot: &mut Nikolaj) -> Self {
        let mut order = BuildOrder {
            bot: _bot,
            units: vec![],
            next_multiproduction_structure: None,
        };
        order.units = decide_units(&mut order.bot);
        order.next_multiproduction_structure =
            decide_next_structure(&mut order.bot, order.units.clone());

        order
    }

    pub fn execute_build_order(&self, bot: &mut Nikolaj) {
        //init idle production to prevent double training
        for structure in bot.units.my.structures.of_types(&PRODUCTION).ready().idle() {
            bot.idle_production.push(structure.tag());
        }

        for unit in self.units.clone() {
            if let Some(source) = UNIT_SOURCE.get(&unit) {
                //missing source building
                if bot
                    .units
                    .my
                    .structures
                    .of_type_including_alias(source.clone())
                    .ready()
                    .is_empty()
                {
                    if let Some(missing) = BuildOrder::resolve_tech(bot, source.clone()) {
                        if bot.already_pending(missing.clone()) == 0 {
                            BuildOrder::add_to_saving(bot, missing.clone());
                        }
                    }
                    continue;
                }
                if let Some(tech) = TECH_REQUIREMENT.get(&unit) {
                    if bot
                        .units
                        .my
                        .structures
                        .of_type_including_alias(tech.clone())
                        .ready()
                        .is_empty()
                    {
                        if let Some(missing) = BuildOrder::resolve_tech(bot, tech.clone()) {
                            if bot.already_pending(missing.clone()) == 0 {
                                BuildOrder::add_to_saving(bot, missing.clone());
                            }
                        }
                        continue;
                    }
                }
                BuildOrder::train(bot, source.clone(), unit);
            }
        }
    }

    pub fn expand_production(&self, bot: &mut Nikolaj) {
        if let Some(next_structure) = self.next_multiproduction_structure.clone() {
            if !bot.idle_production.is_empty()
                || bot.minerals < 450
                || bot.already_pending(next_structure) != 0
            {
                return;
            }
            if bot.can_afford(next_structure, false) {
                construct(bot, next_structure);
            }
        }
    }

    fn resolve_tech(bot: &mut Nikolaj, unit: UnitTypeId) -> Option<UnitTypeId> {
        //no missing tech
        if !bot
            .units
            .my
            .structures
            .of_type_including_alias(unit.clone())
            .is_empty()
        {
            return None;
        }
        //missing tech is already pending
        if bot.already_pending(unit.clone()) != 0 {
            return Some(unit.clone());
        }
        if let Some(tech) = TECH_REQUIREMENT.get(&unit) {
            //no lower tech missing. Construct current tech
            if !bot
                .units
                .my
                .structures
                .of_type_including_alias(tech.clone())
                .is_empty()
            {
                construct(bot, unit.clone());
                return Some(unit.clone());
            }
            //go deeper
            return BuildOrder::resolve_tech(bot, tech.clone());
        } else {
            //no tech requirement
            construct(bot, unit.clone());
            return Some(unit.clone());
        }
    }

    fn add_to_saving(bot: &mut Nikolaj, unit: UnitTypeId) {
        if !bot.saving_on.contains(&unit.clone()) {
            bot.subtract_resources(unit.clone(), false);
            bot.saving_on.push(unit.clone());
        }
    }

    fn train(bot: &mut Nikolaj, source: UnitTypeId, unit: UnitTypeId) {
        if UNITS_NEED_TECHLAB.contains(&unit.clone()) {
            //train with techlab structure
            for structure in bot
                .units
                .my
                .structures
                .of_type(source)
                .find_tags(&bot.idle_production)
            {
                if structure.has_techlab() {
                    if bot.idle_production.contains(&structure.tag()) {
                        bot.idle_production.retain(|&x| x != structure.tag());
                    }
                    structure.train(unit, false);
                    BuildOrder::add_to_saving(bot, unit);
                    return;
                }
            }
            //techlab missing
            if let Some(techlab) = TECHLABS.get(&source) {
                if bot.already_pending(techlab.clone()) == 0 {
                    for structure in bot
                        .units
                        .my
                        .structures
                        .of_type(techlab.clone())
                        .find_tags(&bot.idle_production)
                    {
                        if bot.can_afford(techlab.clone(), true) {
                            if bot.idle_production.contains(&structure.tag()) {
                                bot.idle_production.retain(|&x| x != structure.tag());
                            }
                            structure.command(AbilityId::BuildTechLab, Target::None, false);
                            BuildOrder::add_to_saving(bot, techlab.clone());
                        }
                        return;
                    }
                } else {
                    //wait for techlab to finish
                    return;
                }
            }
        } else {
            //train without techlab
            for structure in bot
                .units
                .my
                .structures
                .of_type(source)
                .find_tags(&bot.idle_production)
            {
                if bot.can_afford(unit, true) {
                    if bot.idle_production.contains(&structure.tag()) {
                        bot.idle_production.retain(|&x| x != structure.tag());
                    }
                    structure.train(unit, false);
                    BuildOrder::add_to_saving(bot, unit);
                }
                return;
            }
        }

        for structure in bot
            .units
            .my
            .structures
            .of_type(source)
            .find_tags(&bot.idle_production)
        {
            if bot.can_afford(unit, true) {
                if bot.idle_production.contains(&structure.tag()) {
                    bot.idle_production.retain(|&x| x != structure.tag());
                }
                structure.train(unit, false);
            }
            BuildOrder::add_to_saving(bot, unit);
        }
    }
}
