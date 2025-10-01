use crate::Nikolaj;
use rust_sc2::prelude::*;


pub fn addons_control(bot: &mut Nikolaj) {
    barracks_tech_lab_control(bot);
    factory_tech_lab_control(bot);
    starport_tech_lab_control(bot);
}

fn barracks_tech_lab_control(bot: &mut Nikolaj) {
    for structure in bot.units.my.structures.of_type(UnitTypeId::BarracksTechLab).ready() {
        if let Some(abilities) = structure.abilities() {
            let combat_shield_researched = !abilities.contains(&AbilityId::ResearchCombatShield);
            let can_afford_combat_shield_research = bot.can_afford_upgrade(UpgradeId::ShieldWall);
            if !combat_shield_researched && can_afford_combat_shield_research {
                structure.use_ability(AbilityId::ResearchCombatShield, false);
                if bot.debugger.printing_research {
                    println!("[DEBUGGER] Researching Combat Shield");
                }
                continue;
            }

            let stimpack_researched = !abilities.contains(&AbilityId::BarracksTechLabResearchStimpack);
            let can_afford_stimpack_research = bot.can_afford_upgrade(UpgradeId::Stimpack);

            if !stimpack_researched && can_afford_stimpack_research {
                structure.use_ability(AbilityId::BarracksTechLabResearchStimpack, false);
                if bot.debugger.printing_research {
                    println!("[DEBUGGER] Researching Stimpack");
                }
                continue;
            }
        }
    }
}

fn factory_tech_lab_control(bot: &mut Nikolaj) {
    for structure in bot.units.my.structures.of_type(UnitTypeId::FactoryTechLab).ready() {
        if let Some(abilities) = structure.abilities() {
            let drilling_claws_researched = !abilities.contains(&AbilityId::ResearchDrillingClaws);
            let can_afford_drilling_claws_research = bot.can_afford_upgrade(UpgradeId::DrillClaws);

            if !drilling_claws_researched && can_afford_drilling_claws_research {
                structure.use_ability(AbilityId::ResearchDrillingClaws, false);
                if bot.debugger.printing_research {
                    println!("[DEBUGGER] Researching Drilling Claws");
                }
                continue;
            }
        }
    }
}

fn starport_tech_lab_control(bot: &mut Nikolaj) {
    for structure in bot.units.my.structures.of_type(UnitTypeId::StarportTechLab).ready().idle() {
        if let Some(abilities) = structure.abilities() {
            let banshee_cloak_researched = !abilities.contains(&AbilityId::ResearchBansheeCloakingField);
            let can_afford_banshee_cloak_research = bot.can_afford_upgrade(UpgradeId::BansheeCloak);

            if !banshee_cloak_researched && can_afford_banshee_cloak_research {
                structure.use_ability(AbilityId::ResearchBansheeCloakingField, false);
                if bot.debugger.printing_research {
                    println!("[DEBUGGER] Researching Banshee Cloak");
                }
                continue;
            }
        }
    }
}
