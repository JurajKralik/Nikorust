use crate::Nikolaj;
use rust_sc2::prelude::*;
use crate::units::helpers::surroundings::*;
use crate::units::helpers::heatmap::*;
use crate::units::helpers::combat_movement::*;


pub fn viking_control(bot: &mut Nikolaj, unit: &Unit) {
    let avoid_damage = unit.weapon_cooldown() > 0.2;
    let surroundings = get_surroundings_info(bot, unit);
    let heatmap_options = HeatmapOptions {
        step: 2.0,
        avoid_damage: avoid_damage,
        ..Default::default()
    };
    let heatmap = get_heatmap_for_unit(bot, unit.tag(), heatmap_options);
    let weapon_ready = unit.weapon_cooldown() < 0.2;
    
    let air_target_in_range = surroundings.best_target_in_range
        .as_ref()
        .filter(|target| target.is_flying());
    
    let air_target_off_range = surroundings.better_target_off_range
        .as_ref()
        .filter(|target| target.is_flying());

    if weapon_ready {
        if let Some(target) = air_target_in_range {
            attack_no_spam(unit, Target::Tag(target.tag()));
        } else if let Some(target) = air_target_off_range {
            attack_no_spam(unit, Target::Tag(target.tag()));
        } else {
            if bot.strategy_data.attack {
                support_ground_attack(bot, unit);
            } else {
                defensive_air_patrol(bot, unit);
            }
        }
    } else {
        if let Some(best_position) = heatmap.get_best_position() {
            move_no_spam(unit, Target::Pos(best_position));
        } else if let Some(target) = air_target_off_range {
            move_no_spam(unit, Target::Pos(target.position()));
        } else if let Some(target) = air_target_in_range {
            move_no_spam(unit, Target::Pos(target.position()));
        } else {
            // No air targets, move to strategic position
            if bot.strategy_data.attack {
                support_ground_attack(bot, unit);
            } else {
                defensive_air_patrol(bot, unit);
            }
        }
    }
}

fn support_ground_attack(bot: &mut Nikolaj, unit: &Unit) {
    let army_center = bot.strategy_data.army_center;
    let air_support_pos = Point2::new(army_center.x, army_center.y + 4.0);
    
    move_no_spam(unit, Target::Pos(air_support_pos));
}

fn defensive_air_patrol(bot: &mut Nikolaj, unit: &Unit) {
    let defense_point = bot.strategy_data.defense_point;
    let air_patrol_pos = Point2::new(defense_point.x, defense_point.y + 5.0);
    let patrol_radius = 6.0;
    let angle = (bot.time * 0.4 + unit.tag() as f32 * 0.1) % (2.0 * std::f32::consts::PI);
    let patrol_pos = Point2::new(
        air_patrol_pos.x + patrol_radius * angle.cos(),
        air_patrol_pos.y + patrol_radius * angle.sin()
    );
    
    move_no_spam(unit, Target::Pos(patrol_pos));
}