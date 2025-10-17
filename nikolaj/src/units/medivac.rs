use crate::Nikolaj;
use rust_sc2::prelude::*;
use crate::units::helpers::surroundings::*;
use crate::units::helpers::heatmap::*;
use crate::units::helpers::combat_movement::*;
use crate::units::helpers::threat_detection::*;


pub fn medivac_control(bot: &mut Nikolaj, unit: &Unit) {
    if unit.cargo_space_taken() > 0 {
        unit.command(AbilityId::UnloadAllAtMedivac, Target::Tag(unit.tag()), false);
        return;
    }

    let surroundings = get_surroundings_info(bot, unit);
    let in_danger = surroundings.threat_level > ThreatLevel::None;

    if in_danger {
        let heatmap_options = HeatmapOptions {
            step: 2.0,
            avoid_damage: true,
            allies_influence: true,
            ..Default::default()
        };
        
        let heatmap = get_heatmap_for_unit(bot, unit.tag(), heatmap_options);
        
        if let Some(best_position) = heatmap.get_best_position() {
            move_no_spam(unit, Target::Pos(best_position));
        }
    } else {
        attack_no_spam(unit, Target::Pos(bot.strategy_data.army_center));
    }
}