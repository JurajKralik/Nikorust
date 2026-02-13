use crate::Nikolaj;
use crate::consts::UNITS_PRIORITY_LOW;
use crate::units::helpers::combat_movement::*;
use crate::units::helpers::surroundings::*;
use rust_sc2::prelude::*;


pub fn bio_control(bot: &mut Nikolaj, unit: &Unit) {
    let surroundings = get_surroundings_info(bot, unit, SurroundingsOptions::default());
    let target = surroundings.best_target_in_range.clone();
    let distanced = surroundings.better_target_off_range.clone();
    let closest = surroundings.closest_threat.clone();
    let fear = surroundings.closest_counter.clone();

    let weapon_ready = unit.weapon_cooldown() < 0.2;
    let offensive = bot.strategy_data.attack;
    let defensive = bot.strategy_data.defend;
    let army_center = bot.strategy_data.army_center;
    let attack_point = bot.strategy_data.attack_point;
    let defense_point = bot.strategy_data.defense_point;

    let mut bunker: Option<Unit> = None;
    let bunkers = bot
        .units
        .my
        .structures
        .of_type(UnitTypeId::Bunker)
        .ready()
        .closer(20.0, unit.position());
    if !bunkers.is_empty() {
        bunker = bunkers.closest(unit.position()).cloned();
    }

    let mut medivac: Option<Unit> = None;
    for potential_medivac in bot
        .units
        .my
        .units
        .of_type(UnitTypeId::Medivac)
        .closer(10.0, unit.position())
    {
        if potential_medivac.cargo_left() > 1 && potential_medivac.health_percentage() > 0.5 {
            medivac = Some(potential_medivac.clone());
            break;
        }
    }

    let mut tank_cover: Option<Unit> = None;
    for possible_tank in bot
        .units
        .my
        .units
        .of_type(UnitTypeId::SiegeTankSieged)
        .closer(15.0, unit.position())
        .iter()
        .sort_by_distance(unit.position())
    {
        if let Some(closest_enemy) = &closest {
            if closest_enemy.distance(possible_tank.position()) >= unit.distance(possible_tank.position()) {
                tank_cover = Some(possible_tank.clone());
                break;
            }
        } else {
            tank_cover = Some(possible_tank.clone());
            break;
        }
    }

    if let Some(target_unit) = &target {
        if unit.health() > 30 && !UNITS_PRIORITY_LOW.contains(&target_unit.type_id()) {
            use_stimpack(unit, &surroundings);
        }

        if let Some(bunker_unit) = &bunker {
            if defensive && bunker_unit.cargo_left() >= unit.cargo_size() {
                let safe_to_load = if let Some(closest_enemy) = &closest {
                    closest_enemy.distance(bunker_unit.position()) > unit.distance(bunker_unit.position())
                } else {
                    true
                };
                if safe_to_load {
                    unit.smart(Target::Tag(bunker_unit.tag()), false);
                    return;
                }
            }
        }

        if unit.health() < 20 && !weapon_ready {
            if let (Some(medivac_unit), Some(closest_enemy)) = (&medivac, &closest) {
                if in_range(closest_enemy, unit.clone()) {
                    unit.smart(Target::Tag(medivac_unit.tag()), false);
                    return;
                }
            }
        }

        let lowered_depots = bot
            .units
            .my
            .structures
            .of_type(UnitTypeId::SupplyDepotLowered);
        if let Some(closest_enemy) = &closest {
            if let Some(closest_depot) = lowered_depots.closest(unit.position()) {
                if unit.distance(closest_depot.position()) < 1.0 && closest_enemy.distance(unit.position()) < 7.0 {
                    let retreat_position = unit.position().towards(closest_enemy.position(), -2.0);
                    move_no_spam(unit, Target::Pos(retreat_position));
                    return;
                }
            }
        }

        if let Some(fear_unit) = &fear {
            if in_range_with_avoidance(fear_unit.clone(), unit.clone(), 1.0) {
                if let Some(tank) = &tank_cover {
                    attack_no_spam(unit, Target::Pos(tank.position()));
                } else {
                    bio_flee(bot, unit, surroundings.clone());
                }
                return;
            }
        }

        if weapon_ready {
            attack_no_spam(unit, Target::Tag(target_unit.tag()));
            return;
        }

        if let Some(tank) = &tank_cover {
            if let Some(distanced_unit) = &distanced {
                if unit.health() > 40 && offensive {
                    move_no_spam(unit, Target::Pos(distanced_unit.position()));
                    return;
                }
            }

            if unit.health() < 20 {
                if let Some(closest_enemy) = &closest {
                    if in_range(closest_enemy, unit.clone()) {
                        bio_flee(bot, unit, surroundings.clone());
                        return;
                    }
                }
            }

            attack_no_spam(unit, Target::Pos(tank.position()));
            return;
        }

        if let Some(distanced_unit) = &distanced {
            if unit.health() > 40 && offensive {
                move_no_spam(unit, Target::Pos(distanced_unit.position()));
                return;
            }
        }

        bio_flee(bot, unit, surroundings.clone());
        return;
    }

    if let Some(distanced_unit) = &distanced {
        if fear.is_none() && (offensive || tank_cover.is_some()) {
            if unit.distance(army_center) < 25.0 || tank_cover.is_some() {
                attack_no_spam(unit, Target::Tag(distanced_unit.tag()));
            } else {
                move_no_spam(unit, Target::Pos(army_center));
            }
            return;
        }

        if let Some(tank) = &tank_cover {
            attack_no_spam(unit, Target::Pos(tank.position()));
            return;
        }

        if fear.is_some() {
            bio_flee(bot, unit, surroundings.clone());
            return;
        }

        if offensive {
            move_no_spam(unit, Target::Pos(attack_point));
        } else if defensive {
            move_no_spam(unit, Target::Pos(defense_point));
        } else {
            move_no_spam(unit, Target::Pos(army_center));
        }
        return;
    }

    if defensive {
        attack_no_spam(unit, Target::Pos(defense_point));
        return;
    }

    if offensive {
        if unit.distance(army_center) < 25.0 {
            let tanks_35 = bot
                .units
                .my
                .units
                .of_type(UnitTypeId::SiegeTank)
                .closer(35.0, unit.position());
            let tanks_20 = bot
                .units
                .my
                .units
                .of_type(UnitTypeId::SiegeTank)
                .closer(20.0, unit.position());

            if !tanks_35.is_empty() && tanks_20.is_empty() {
                if let Some(closest_tank) = tanks_35.closest(unit.position()) {
                    attack_no_spam(unit, Target::Pos(closest_tank.position()));
                    return;
                }
            }

            attack_no_spam(unit, Target::Pos(attack_point));
        } else {
            attack_no_spam(unit, Target::Pos(army_center));
        }
        return;
    }

    if unit.distance(defense_point) > 6.0 {
        attack_no_spam(unit, Target::Pos(defense_point));
    } else {
        move_no_spam(unit, Target::Pos(bot.strategy_data.idle_point));
    }
}