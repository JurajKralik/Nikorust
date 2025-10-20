use crate::Nikolaj;
use crate::units::helpers::combat_movement::*;
use crate::units::helpers::surroundings::*;
use crate::units::helpers::threat_detection::*;
use rust_sc2::prelude::*;


pub fn marauder_control(bot: &mut Nikolaj, unit: &Unit) {
    let surroundings = get_surroundings_info(bot, unit);
    let low_health = unit.health_percentage() < 0.4;
    let weapon_ready = unit.weapon_cooldown() < 0.2;
    let in_danger = surroundings.clone().threat_level > ThreatLevel::None;

    if weapon_ready {
        use_stimpack(unit, &surroundings.clone());
        // Direct attack
        if let Some(target) = surroundings.clone().best_target_in_range {
            attack_no_spam(unit, Target::Tag(target.tag()));
        // Off range attack
        } else if let Some(target) = surroundings.clone().better_target_off_range {
            if !low_health || surroundings.clone().closest_threat.is_none() {
                attack_no_spam(unit, Target::Tag(target.tag()));
            } else {
                flee_bio(bot, unit, surroundings.clone());
            }
        // Join defense or attack
        } else if bot.strategy_data.defend {
            let defend_point = bot.strategy_data.defense_point;
            attack_no_spam(unit, Target::Pos(defend_point));
        } else if bot.strategy_data.attack {
            let close_tanks = bot.units.my.units.of_type_including_alias(UnitTypeId::SiegeTank).closer(10.0, unit.position());
            let distanced_tanks = bot.units.my.units.of_type_including_alias(UnitTypeId::SiegeTank).closer(20.0, unit.position());
            if close_tanks.is_empty() && !distanced_tanks.is_empty() || unit.distance(bot.strategy_data.army_center) > 8.0 {
                attack_no_spam(unit, Target::Pos(bot.strategy_data.army_center));
            } else {
                let attack_point = bot.strategy_data.attack_point;
                attack_no_spam(unit, Target::Pos(attack_point));
            }
        // Idle
        } else {
            let idle_point = bot.strategy_data.idle_point;
            move_no_spam(unit, Target::Pos(idle_point));
        }
    } else {
        // No threats, no worries
        if surroundings.clone().closest_threat.is_none() {
            if let Some(target) = surroundings.clone().best_target_in_range {
                move_no_spam(unit, Target::Pos(target.position()));
            } else if let Some(target) = surroundings.clone().better_target_off_range {
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
            flee_bio(bot, unit, surroundings.clone());
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