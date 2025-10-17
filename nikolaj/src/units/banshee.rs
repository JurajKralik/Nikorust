use crate::Nikolaj;
use crate::units::helpers::combat_movement::*;
use crate::units::helpers::surroundings::*;
use crate::units::helpers::threat_detection::*;
use crate::units::helpers::heatmap::*;
use rust_sc2::prelude::*;


pub fn banshee_control(bot: &mut Nikolaj, unit: &Unit) {
    let surroundings = get_surroundings_info(bot, unit);
    let heatmap_options = HeatmapOptions {
        evaluate_detection: unit.has_buff(BuffId::BansheeCloak),
        ..Default::default()
    };
    let heatmap = get_heatmap_for_unit(bot, unit.tag(), heatmap_options);
    let weapon_ready = unit.weapon_cooldown() < 0.2;
    let in_repair_list = bot.worker_allocator.repair.contains_key(&unit.tag());
    let in_danger = surroundings.clone().threat_level > ThreatLevel::None;

    if banshee_cloak(unit, &surroundings) {
        return;
    }

    if in_repair_list {
        if surroundings.clone().closest_threat.is_some() || in_danger {
            flee_flying_unit(bot, unit, surroundings.clone());
        } else {
            let closest_repair_point = get_closest_repair_point(bot, unit);
            move_no_spam(unit, Target::Pos(closest_repair_point));
        }
    } else {
        if weapon_ready {
            if in_danger {
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
            if let Some(best_position) = heatmap.get_best_position() {
                move_no_spam(unit, Target::Pos(best_position));
            } else if in_danger {
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