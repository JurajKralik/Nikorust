use crate::Nikolaj;
use crate::consts::UNITS_PRIORITY_LOW;
use crate::units::helpers::combat_movement::*;
use crate::units::helpers::surroundings::*;
use crate::units::helpers::nearby_utilities_reading::*;
use rust_sc2::prelude::*;


pub fn bio_control(bot: &mut Nikolaj, unit: &Unit) {
    let surroundings = get_surroundings_info(bot, unit, SurroundingsOptions::default());
    let target = surroundings.best_target_in_range.clone();
    let distanced = surroundings.better_target_off_range.clone();
    let closest = surroundings.closest_threat.clone();
    let fear = surroundings.closest_counter.clone();

    let offensive = bot.strategy_data.attack;
    let defensive = bot.strategy_data.defend;
    let army_center = bot.strategy_data.army_center;
    let attack_point = bot.strategy_data.attack_point;
    let defense_point = bot.strategy_data.defense_point;

    let bunker_request = bot.combat_info.get_bunker_by_unit(unit.tag());
    let medivac = get_closest_medivac(bot, unit);
    let tank_cover = get_closest_tank_cover(bot, unit);
    let standing_on_depot = get_standing_on_depot(bot, unit);

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

    if let Some(bunker) = bunker_request.and_then(|bunker_tag| bot.units.my.structures.iter().find_tag(bunker_tag)) {
        unit.smart(Target::Tag(bunker.tag()), false);
        return;
    }

    if let Some(tank) = tank_cover {
        if unit.distance(&tank) > 6.0 {
            move_no_spam(unit, Target::Pos(tank.position()));
            return;
        }
    }

    if let Some(depot) = standing_on_depot {
        let anti_depot_position = Target::Pos(unit.position().towards(depot.position(), -0.5));
        unit.move_to(anti_depot_position, false);
        return;
    }

    if let Some(medivac) = medivac {
        if unit.health_percentage() < 0.5 && unit.distance(&medivac) > 4.0 {
            move_no_spam(unit, Target::Pos(medivac.position()));
            return;
        }
    }

}