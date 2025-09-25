use crate::Nikolaj;
use crate::units::helpers::combat_movement::*;
use crate::units::helpers::surroundings::*;
use rust_sc2::prelude::*;


pub fn banshee_control(bot: &mut Nikolaj, unit: &Unit) {
    let surroundings = get_surroundings_info(bot, unit);
    let low_health = unit.health_percentage().unwrap_or(1.0) < 0.6;
    let weapon_ready = unit.weapon_cooldown().unwrap_or(0.0) < 0.2;

    if banshee_cloak(unit, &surroundings) {
        return;
    }

    if low_health {
        if surroundings.clone().closest_threat.is_some() || surroundings.clone().in_danger {
            flee_flying_unit(bot, unit, surroundings.clone());
        } else {
            let closest_repair_point = get_closest_repair_point(bot, unit);
            move_no_spam(unit, Target::Pos(closest_repair_point));
        }
    } else {
        if weapon_ready {
            if surroundings.clone().in_danger {
                flee_flying_unit(bot, unit, surroundings.clone());
            } else {
                if let Some(target) = surroundings.best_target_in_range {
                    attack_no_spam(unit, Target::Tag(target.tag()));
                } else if let Some(target) = surroundings.better_target_off_range {
                    attack_no_spam(unit, Target::Tag(target.tag()));
                } else {
                    let closest_harass_point = get_closest_harass_point(bot, unit);
                    move_no_spam(unit, Target::Pos(closest_harass_point));
                }
            }
        } else {
            if surroundings.clone().in_danger {
                flee_flying_unit(bot, unit, surroundings.clone());
            } else {
                if let Some(target) = surroundings.better_target_off_range {
                    move_no_spam(unit, Target::Pos(target.position()));
                } else if let Some(target) = surroundings.best_target_in_range {
                    move_no_spam(unit, Target::Pos(target.position()));
                } else {
                    let closest_harass_point = get_closest_harass_point(bot, unit);
                    move_no_spam(unit, Target::Pos(closest_harass_point));
                }
            }
        }
    }
}