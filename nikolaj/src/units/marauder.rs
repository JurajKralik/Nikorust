use crate::Nikolaj;
use crate::units::helpers::combat_movement::*;
use crate::units::helpers::surroundings::*;
use rust_sc2::prelude::*;


pub fn marauder_control(bot: &mut Nikolaj, unit: &Unit) {
    let surroundings = get_surroundings_info(bot, unit);
    let low_health = unit.health_percentage() < 0.4;
    let weapon_ready = unit.weapon_cooldown().unwrap_or(0.0) < 0.2;

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
            if unit.distance(bot.strategy_data.army_center) > 8.0 {
                attack_no_spam(unit, Target::Pos(bot.strategy_data.army_center));
            } else {
                let attack_point = bot.strategy_data.attack_point;
                attack_no_spam(unit, Target::Pos(attack_point));
            }
        // Idle
        } else {
            let idle_point = bot.strategy_data.idle_point;
            unit.move_to(Target::Pos(idle_point), false);
        }
    } else {
        // No threats, no worries
        if surroundings.clone().closest_threat.is_none() {
            if let Some(target) = surroundings.clone().best_target_in_range {
                unit.move_to(Target::Pos(target.position()), false);
            } else if let Some(target) = surroundings.clone().better_target_off_range {
                unit.move_to(Target::Pos(target.position()), false);
            } else if bot.strategy_data.defend {
                let defend_point = bot.strategy_data.defense_point;
                unit.move_to(Target::Pos(defend_point), false);
            } else if bot.strategy_data.attack {
                if unit.distance(bot.strategy_data.army_center) > 8.0 {
                    unit.move_to(Target::Pos(bot.strategy_data.army_center), false);
                } else {
                    let attack_point = bot.strategy_data.attack_point;
                    unit.move_to(Target::Pos(attack_point), false);
                }
            } else {
                let idle_point = bot.strategy_data.idle_point;
                unit.move_to(Target::Pos(idle_point), false);
            }
        // Threats, flee
        } else if surroundings.clone().in_danger || low_health {
            flee_bio(bot, unit, surroundings.clone());
        } else if bot.strategy_data.defend {
            let defend_point = bot.strategy_data.defense_point;
            unit.move_to(Target::Pos(defend_point), false);
        } else if bot.strategy_data.attack {
            if unit.distance(bot.strategy_data.army_center) > 8.0 {
                unit.move_to(Target::Pos(bot.strategy_data.army_center), false);
            } else {
                let attack_point = bot.strategy_data.attack_point;
                unit.move_to(Target::Pos(attack_point), false);
            }
        } else {
            let idle_point = bot.strategy_data.idle_point;
            unit.move_to(Target::Pos(idle_point), false);
        }
    }
}