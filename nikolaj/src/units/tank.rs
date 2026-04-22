use crate::Nikolaj;
use crate::helpers::map_manager::TacticalPosition;
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

    let offensive = bot.strategy_data.attack;
    let fortification_position = bot.map_manager.get_tank_position_by_tag(unit.tag());
    let mut stay_fortified = false;
    if let Some(pos) = fortification_position {
        if unit.distance(pos) < 2.0 && !offensive {
            stay_fortified = true;
        }
    }

    if sieged {
        if in_danger && no_targets {
            force_unsiege(bot, unit);
            return;
        }

        if !should_siege && !stay_fortified {
            unsiege(bot, unit);
        }
        return;
    }

    if should_siege {
        siege_up(bot, unit);
        return;
    }

    if join_strategy(bot, unit){
        return;
    }
    
    fortify(bot, unit)
}


fn fortify(bot: &mut Nikolaj, unit: &Unit) {
    let idle_point = bot.strategy_data.idle_point;
    let closest_base_position = match bot.units.my.townhalls.closest(idle_point) {
        None => {
            move_no_spam(unit, Target::Pos(idle_point));
            return;
        },
        Some(unit) => unit.position(),
    };
    let mut tank_positions = bot.map_manager.get_tank_positions_for_base(closest_base_position);
    tank_positions.sort_by(|a, b| {
        a.position.distance(unit.position())
            .partial_cmp(&b.position.distance(unit.position()))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    if let Some(best_position) = get_best_tank_position(unit, tank_positions) {
        if unit.distance(best_position) > 1.0 {
            move_no_spam(unit, Target::Pos(best_position));
        } else {
            siege_up(bot, unit);
        }
        bot.map_manager.mark_position_occupied(best_position, unit.tag());
    } else {
        move_no_spam(unit, Target::Pos(idle_point));
    }
}


fn get_best_tank_position(unit: &Unit, positions: Vec<TacticalPosition>) -> Option<Point2> {
    let mut least_crowded_position = None;
    let mut lowest_crowd_level = f32::MAX;
    for pos in positions {
        if let Some(tag) = pos.occupied_by {
            if tag == unit.tag() {
                return Some(pos.position);
            }
        } else {
            let crowd_level = pos.crowd_level;
            if crowd_level < lowest_crowd_level {
                lowest_crowd_level = crowd_level;
                least_crowded_position = Some(pos.position);
            }
        }
    }
    least_crowded_position
}