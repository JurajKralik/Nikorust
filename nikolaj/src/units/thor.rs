use crate::Nikolaj;
use rust_sc2::prelude::*;
use crate::units::helpers::surroundings::*;
use crate::units::helpers::heatmap::*;
use crate::units::helpers::combat_movement::*;


pub fn thor_control(bot: &mut Nikolaj, unit: &Unit) {
    if let Some(abilities) = unit.abilities() {
        if abilities.contains(&AbilityId::MorphThorHighImpactMode) {
            unit.use_ability(AbilityId::MorphThorHighImpactMode, false);
            return;
        }
    }
    let surroundings = get_surroundings_info(bot, unit);
    let heatmap_options = HeatmapOptions {
        step: 2.0,
        avoid_damage: false,
        allies_influence: true,
        ..Default::default()
    };
    let heatmap = get_heatmap_for_unit(bot, unit.tag(), heatmap_options);
    let weapon_ready = unit.weapon_cooldown() < 0.2;

    let air_target = surroundings.best_target_in_range
        .as_ref()
        .filter(|target| target.is_flying())
        .or_else(|| surroundings.better_target_off_range.as_ref().filter(|target| target.is_flying()));

    if weapon_ready {
        if let Some(target) = air_target {
            attack_no_spam(unit, Target::Tag(target.tag()));
        } else if let Some(target) = surroundings.best_target_in_range {
            attack_no_spam(unit, Target::Tag(target.tag()));
        } else if let Some(target) = surroundings.better_target_off_range {
            attack_no_spam(unit, Target::Tag(target.tag()));
        } else {
            attack_no_spam(unit, Target::Pos(bot.strategy_data.army_center));
        }
    } else {
        if let Some(best_position) = heatmap.get_best_position() {
            move_no_spam(unit, Target::Pos(best_position));
        } else {
            attack_no_spam(unit, Target::Pos(bot.strategy_data.army_center));
        }
    }
}