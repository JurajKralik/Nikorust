use crate::Nikolaj;
use rust_sc2::prelude::*;
use crate::units::helpers::surroundings::*;
use crate::units::helpers::heatmap::*;
use crate::units::helpers::combat_movement::*;
use crate::units::helpers::threat_detection::*;

pub fn raven_control(bot: &mut Nikolaj, unit: &Unit) {
    let surroundings = get_surroundings_info(bot, unit);
    let in_danger = surroundings.threat_level > ThreatLevel::None;
    
    if unit.energy() >= 50.0 as u32 {
        if let Some(target) = surroundings.closest_threat.as_ref() {
            let turret_pos = unit.position().towards(target.position(), 5.0);
            unit.command(AbilityId::BuildAutoTurretAutoTurret, Target::Pos(turret_pos), false);
            return;
        }
    }

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
        let army_center = bot.strategy_data.army_center;
        let support_pos = Point2::new(army_center.x, army_center.y + 4.0);
        move_no_spam(unit, Target::Pos(support_pos));
    }
}