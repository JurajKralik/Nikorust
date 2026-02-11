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
    let idle_point = Target::Pos(bot.strategy_data.idle_point);
    let attack_point = Target::Pos(bot.strategy_data.attack_point);
    let defense_point = Target::Pos(bot.strategy_data.defense_point);
    
    let has_target_in_range = best_target_in_range.is_some();
    let has_target_off_range = better_target_off_range.is_some();
    let safe_to_engage = !low_health && closest_threat.is_none();
    let defense_needed = bot.strategy_data.defend;
    let attacking_strategy = bot.strategy_data.attack;
    let should_join_army = should_wait_for_tanks(bot, unit) || unit.distance(bot.strategy_data.army_center) > 8.0;

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

    } else if safe_to_engage {
        if has_target_in_range {
            move_into_range(unit, best_target_in_range.unwrap());
            return;
        }
        if has_target_off_range {
            move_into_range(unit, better_target_off_range.unwrap());
            return;
        } 
    }

    if in_danger {
        bio_flee(bot, unit, surroundings);
        return;
    }

    if defense_needed {
        attack_no_spam(unit, defense_point);
        return;
    } 
    
    if attacking_strategy {
        if should_join_army {
            attack_no_spam(unit, Target::Pos(bot.strategy_data.army_center));
        } else {
            attack_no_spam(unit, attack_point);
        }
        return;
    }
    
    move_no_spam(unit, idle_point);
}