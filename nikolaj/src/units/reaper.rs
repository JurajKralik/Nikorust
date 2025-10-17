use crate::Nikolaj;
use crate::units::helpers::combat_movement::*;
use crate::units::helpers::surroundings::*;
use crate::units::helpers::threat_detection::*;
use crate::units::helpers::heatmap::*;
use rust_sc2::prelude::*;



pub fn reaper_control(bot: &mut Nikolaj, unit: &Unit) {
    let surroundings = get_surroundings_info(bot, unit);
    let heatmap_options = HeatmapOptions {
        avoid_damage: true,
        ..Default::default()
    };
    let heatmap = get_heatmap_for_unit(bot, unit.tag(), heatmap_options);
    let low_health = unit.health_percentage() < 0.6;
    let weapon_ready = unit.weapon_cooldown() < 0.2;
    let in_danger = surroundings.clone().threat_level > ThreatLevel::None;

    if kd8_charge(unit, &surroundings) {
        return;
    }

    if low_health {
        if surroundings.clone().closest_threat.is_some() || in_danger {
            flee_bio(bot, unit, surroundings.clone());
        } else {
            move_no_spam(unit, Target::Pos(bot.strategy_data.idle_point));
        }
    } else {
        if weapon_ready {
            if in_danger {
                flee_bio(bot, unit, surroundings.clone());
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
            if let Some(best_position) = heatmap.get_best_position() {
                move_no_spam(unit, Target::Pos(best_position));
            } else {
                if in_danger {
                    flee_bio(bot, unit, surroundings.clone());
                } else if let Some(target) = surroundings.better_target_off_range {
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