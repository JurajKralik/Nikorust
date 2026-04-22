use crate::Nikolaj;
use crate::units::helpers::combat_movement::*;
use crate::units::helpers::surroundings::*;
use crate::units::helpers::nearby_utilities_reading::*;
use rust_sc2::prelude::*;


pub fn bio_control(bot: &mut Nikolaj, unit: &Unit) {
    let surroundings = get_surroundings_info(bot, unit, SurroundingsOptions::default());
    let idle_point = bot.strategy_data.idle_point;

    let target = &surroundings.best_target_in_range;
    let weapon_ready = unit.weapon_cooldown() < 0.2;

    if should_stimpack(unit, &surroundings) {
        use_stimpack(unit);
        return;
    }

    if let Some(target) = target {
        if weapon_ready {
            unit.attack(Target::Tag(target.tag()), false);
            return;
        }
    }

    if react_to_area(bot, unit, &surroundings) {
        return;
    }

    if join_strategy(bot, unit) {
        return;
    }
    
    move_no_spam(unit, Target::Pos(idle_point));
}


fn should_stimpack(unit: &Unit, surroundings: &SurroundingsInfo) -> bool {
    (surroundings.best_target_in_range.is_some()
        || surroundings.better_target_off_range.is_some())
        && !unit.has_buff(BuffId::Stimpack)
        && !unit.has_buff(BuffId::StimpackMarauder)
        && unit.health_percentage() > 0.5
        && (unit.abilities().map_or(false, |a| a.contains(&AbilityId::EffectStimMarine))
        || unit.abilities().map_or(false, |a| a.contains(&AbilityId::EffectStimMarauder)))
}


fn use_stimpack(unit: &Unit) {
    match unit.type_id() {
        UnitTypeId::Marine => unit.use_ability(AbilityId::EffectStimMarine, false),
        UnitTypeId::Marauder => unit.use_ability(AbilityId::EffectStimMarauder, false),
        _ => {}
    }
}


fn react_to_area(bot: &mut Nikolaj, unit: &Unit, surroundings: &SurroundingsInfo) -> bool {
    let bunker_request = bot.combat_info.get_bunker_by_unit(unit.tag());
    let medivac = get_closest_medivac(bot, unit);
    let tank_cover = get_closest_tank_cover(bot, unit);
    let standing_on_depot = get_standing_on_depot(bot, unit);

    if let Some(bunker) = bunker_request.and_then(|bunker_tag| bot.units.my.structures.iter().find_tag(bunker_tag)) {
        unit.smart(Target::Tag(bunker.tag()), false);
        return true;
    }

    if surroundings.closest_counter.is_none() && surroundings.closest_threat.is_none() {
        return false;
    }

    if let Some(tank) = tank_cover {
        if unit.distance(&tank) > 6.0 {
            move_no_spam(unit, Target::Pos(tank.position()));
            return true;
        }
    }

    if let Some(depot) = standing_on_depot {
        let anti_depot_position = Target::Pos(unit.position().towards(depot.position(), -0.5));
        unit.move_to(anti_depot_position, false);
        return true;
    }

    if let Some(medivac) = medivac {
        if unit.health_percentage() < 0.5 && unit.distance(&medivac) > 4.0 {
            move_no_spam(unit, Target::Pos(medivac.position()));
            return true;
        }
    }
    false
}