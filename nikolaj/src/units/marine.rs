use crate::Nikolaj;
use crate::units::helpers::combat_movement::*;
use crate::units::helpers::surroundings::*;
use crate::units::helpers::threat_detection::*;
use rust_sc2::prelude::*;


pub fn marine_control(bot: &mut Nikolaj, unit: &Unit) {
    let surroundings = get_surroundings_info(bot, unit, SurroundingsOptions::default());
    let low_health = unit.health_percentage() < 0.4;
    let weapon_ready = unit.weapon_cooldown() < 0.2;
    let in_danger = surroundings.threat_level > ThreatLevel::None;
    
    let best_target_in_range = surroundings.best_target_in_range.as_ref();
    let better_target_off_range = surroundings.better_target_off_range.as_ref();
    let closest_threat = surroundings.closest_threat.as_ref();
    
    let has_target_in_range = best_target_in_range.is_some();
    let has_target_off_range = better_target_off_range.is_some();
    let safe_to_engage = !low_health && closest_threat.is_none();
    let defense_needed = bot.strategy_data.defend;
    let attacking_strategy = bot.strategy_data.attack;

    if weapon_ready {
        use_stimpack(unit, &surroundings);

        if has_target_in_range {
            let target = Target::Tag(best_target_in_range.unwrap().tag());
            attack_no_spam(unit, target);
            return;
        } 
        
        if has_target_off_range && safe_to_engage {
            let target = Target::Tag(better_target_off_range.unwrap().tag());
            attack_no_spam(unit, target);
            return;
        } 

        if in_danger {
            bio_flee(bot, unit, surroundings);
            return;
        }

        if defense_needed {
            let defend_point = Target::Pos(bot.strategy_data.defense_point);
            attack_no_spam(unit, defend_point);
            return;
        } 
        
        if attacking_strategy {
            let close_tanks = bot.units.my.units.of_type_including_alias(UnitTypeId::SiegeTank).closer(10.0, unit.position());
            let distanced_tanks = bot.units.my.units.of_type_including_alias(UnitTypeId::SiegeTank).closer(20.0, unit.position());

            if close_tanks.is_empty() && !distanced_tanks.is_empty() || unit.distance(bot.strategy_data.army_center) > 8.0 {
                attack_no_spam(unit, Target::Pos(bot.strategy_data.army_center));
            } else {
                let attack_point = bot.strategy_data.attack_point;
                attack_no_spam(unit, Target::Pos(attack_point));
            }
            return;
        }
        let idle_point = bot.strategy_data.idle_point;
        move_no_spam(unit, Target::Pos(idle_point));
    } else {
        // No threats, no worries
        if closest_threat.is_none() {
            if let Some(target) = best_target_in_range {
                move_no_spam(unit, Target::Pos(target.position()));
            } else if let Some(target) = better_target_off_range {
                move_no_spam(unit, Target::Pos(target.position()));
            } else if bot.strategy_data.defend {
                let defend_point = bot.strategy_data.defense_point;
                move_no_spam(unit, Target::Pos(defend_point));
            } else if bot.strategy_data.attack {
                if unit.distance(bot.strategy_data.army_center) > 8.0 {
                    move_no_spam(unit, Target::Pos(bot.strategy_data.army_center));
                } else {
                    let attack_point = bot.strategy_data.attack_point;
                    move_no_spam(unit, Target::Pos(attack_point));
                }
            } else {
                let idle_point = bot.strategy_data.idle_point;
                move_no_spam(unit, Target::Pos(idle_point));
            }
        // Threats, flee
        } else if in_danger || low_health {
            bio_flee(bot, unit, surroundings);
        } else if bot.strategy_data.defend {
            let defend_point = bot.strategy_data.defense_point;
            move_no_spam(unit, Target::Pos(defend_point));
        } else if bot.strategy_data.attack {
            if unit.distance(bot.strategy_data.army_center) > 8.0 {
                move_no_spam(unit, Target::Pos(bot.strategy_data.army_center));
            } else {
                let close_tanks = bot.units.my.units.of_type_including_alias(UnitTypeId::SiegeTank).closer(10.0, unit.position());
                let distanced_tanks = bot.units.my.units.of_type_including_alias(UnitTypeId::SiegeTank).closer(20.0, unit.position());
                if close_tanks.is_empty() && !distanced_tanks.is_empty() {
                    attack_no_spam(unit, Target::Pos(bot.strategy_data.army_center));
                } else {
                    let attack_point = bot.strategy_data.attack_point;
                    move_no_spam(unit, Target::Pos(attack_point));
                }
            }
        } else {
            let idle_point = bot.strategy_data.idle_point;
            move_no_spam(unit, Target::Pos(idle_point));
        }
    }
}