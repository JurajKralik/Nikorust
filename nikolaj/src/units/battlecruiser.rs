use crate::Nikolaj;
use rust_sc2::prelude::*;
use crate::units::helpers::surroundings::*;
use crate::units::helpers::heatmap::*;
use crate::units::helpers::combat_movement::*;


pub fn battlecruiser_control(bot: &mut Nikolaj, unit: &Unit) {
    let surroundings = get_surroundings_info(bot, unit, SurroundingsOptions::default());
    let heatmap_options = HeatmapOptions {
        step: 2.0,
        avoid_damage: true,
        allies_influence: false,
        ..Default::default()
    };
    let heatmap = get_heatmap_for_unit(bot, unit.tag(), heatmap_options);
    let weapon_ready = unit.weapon_cooldown() < 0.2;
    let low_health = unit.health_percentage() < 0.3;

    if low_health { 
        if let Some(abilities)= unit.abilities() {
            if abilities.contains(&AbilityId::EffectTacticalJump) {
                let escape_pos = get_closest_repair_point(bot, unit);
                unit.command(AbilityId::EffectTacticalJump, Target::Pos(escape_pos), false);
                return;
            }
        }
    }

    if let Some(target) = find_yamato_target(&surroundings) {
        unit.command(AbilityId::YamatoYamatoGun, Target::Tag(target.tag()), false);
        return;
    }
    

    if weapon_ready {
        if let Some(target) = surroundings.best_target_in_range {
            attack_no_spam(unit, Target::Tag(target.tag()));
        } else if let Some(target) = surroundings.better_target_off_range {
            attack_no_spam(unit, Target::Tag(target.tag()));
        } else {
            attack_no_spam(unit, Target::Pos(bot.strategy_data.attack_point));
        }
    } else {
        if let Some(best_position) = heatmap.get_best_position() {
            move_no_spam(unit, Target::Pos(best_position));
        } else {
            attack_no_spam(unit, Target::Pos(bot.strategy_data.attack_point));
        }
    }
}

fn find_yamato_target(surroundings: &SurroundingsInfo) -> Option<Unit> {
    let high_value_types = [
        UnitTypeId::Battlecruiser,
        UnitTypeId::Carrier,
        UnitTypeId::Mothership,
        UnitTypeId::Thor,
        UnitTypeId::Ultralisk,
        UnitTypeId::Colossus,
    ];

    surroundings.better_target_off_range
        .as_ref()
        .filter(|target| high_value_types.contains(&target.type_id()))
        .cloned()
}