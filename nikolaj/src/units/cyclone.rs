use crate::Nikolaj;
use rust_sc2::prelude::*;
use crate::units::helpers::surroundings::*;
use crate::units::helpers::heatmap::*;
use crate::units::helpers::combat_movement::*;


pub fn cyclone_control(bot: &mut Nikolaj, unit: &Unit) {
    let surroundings = get_surroundings_info(bot, unit);
    let heatmap_options = HeatmapOptions {
        step: 2.0,
        avoid_damage: true,
        allies_influence: false, // Cyclones don't care about allies
        ..Default::default()
    };
    let heatmap = get_heatmap_for_unit(bot, unit.tag(), heatmap_options);
    let weapon_ready = unit.weapon_cooldown() < 0.2;

    if weapon_ready {
        // Attack targets when weapon is ready
        if let Some(target) = surroundings.best_target_in_range {
            attack_no_spam(unit, Target::Tag(target.tag()));
        } else if let Some(target) = surroundings.better_target_off_range {
            attack_no_spam(unit, Target::Tag(target.tag()));
        } else {
            // No targets, attack move to army center
            attack_no_spam(unit, Target::Pos(bot.strategy_data.army_center));
        }
    } else {
        // Weapon on cooldown, use heatmap for positioning
        if let Some(best_position) = heatmap.get_best_position() {
            move_no_spam(unit, Target::Pos(best_position));
        } else {
            // Fallback to army center
            attack_no_spam(unit, Target::Pos(bot.strategy_data.army_center));
        }
    }
}