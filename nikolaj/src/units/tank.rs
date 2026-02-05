use crate::Nikolaj;
use crate::units::helpers::surroundings::*;
use crate::units::helpers::combat_movement::*;
use crate::units::helpers::threat_detection::*;
use rust_sc2::prelude::*;


pub fn tank_control(bot: &mut Nikolaj, unit: &Unit) {
    let surroundings = get_surroundings_info(bot, unit, SurroundingsOptions::default());
    let sieged = unit.type_id() == UnitTypeId::SiegeTankSieged;
    let in_danger = surroundings.threat_level.clone() > ThreatLevel::None;
    let no_targets =  surroundings.best_target_in_range.is_none() && surroundings.better_target_off_range.is_none() && surroundings.closest_structure.is_none();
    let should_siege = surroundings.best_target_in_range.is_some() || surroundings.better_target_off_range.is_some() || surroundings.closest_structure.is_some();

    if sieged {
        if in_danger && no_targets {
            force_unsiege(bot, unit);
            return;
        }

        if !should_siege{
            unsiege(bot, unit);
        }
        return;
    }

    if should_siege {
        siege_up(bot, unit);
        return;
    }

    attack_no_spam(unit, Target::Pos(bot.strategy_data.army_center));
}