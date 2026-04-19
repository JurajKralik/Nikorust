use crate::units::helpers::threat_detection::*;
use crate::units::helpers::surroundings::*;
use crate::units::helpers::combat_info::*;
use crate::Nikolaj;
use rust_sc2::{prelude::*, units::AllUnits};


pub fn bio_flee(bot: &mut Nikolaj, unit: &Unit, surroundings: SurroundingsInfo) {
    let units = bot.units.clone();
    if flee_to_bunker(units.clone(), unit) {
        return;
    }
    if flee_to_mine(units.clone(), unit) {
        return;
    }
    if flee_to_medivac(units.clone(), unit) {
        return;
    }
    if flee_to_tank(units.clone(), unit) {
        return;
    }
    if flee_from_threat(bot, unit, surroundings.closest_threat) {
        return;
    }
    let idle_point = bot.strategy_data.idle_point;
    move_no_spam(unit, Target::Pos(idle_point))
}
fn flee_to_bunker(units: AllUnits, unit: &Unit) -> bool {
    let bunkers = units.my.structures.of_type(UnitTypeId::Bunker).ready();
    let nearby_bunkers = bunkers.closer(12.0, unit.position());
    if let Some(bunker) = nearby_bunkers.first() {
        if bunker.cargo_left() >= unit.cargo_size() {
            unit.smart(Target::Tag(bunker.tag()), false);
            return true;
        }
    }
    false
}
fn flee_to_mine(units: AllUnits, unit: &Unit) -> bool {
    let mines = units.my.units.of_type_including_alias(UnitTypeId::WidowMine).ready();
    let nearby_mines = mines.closer(12.0, unit.position());
    let sorted_mines = nearby_mines.iter().sort_by_distance(unit.position());
    for mine in sorted_mines {
        if mine.buff_duration_remain() == 0 {
            if unit.distance(mine.position()) > 3.0 {
                move_no_spam(unit, Target::Pos(mine.position()));
                return true;
            } else {
                attack_no_spam(unit, Target::Pos(mine.position()));
                return true;
            }
        }
    }
    false
}
fn flee_to_medivac(units: AllUnits, unit: &Unit) -> bool {
    let medivacs = units.my.units.of_type(UnitTypeId::Medivac);
    let nearby_medivacs = medivacs.closer(12.0, unit.position());
    if let Some(medivac) = nearby_medivacs.first() {
        if medivac.cargo_space_taken() == 0
            && medivac.health_percentage() > 0.5
        {
            unit.smart(Target::Tag(medivac.tag()), false);
            return true;
        }
    }
    false
}
fn flee_to_tank(units: AllUnits, unit: &Unit) -> bool {
    let tanks = units.my.units.of_type(UnitTypeId::SiegeTankSieged);
    let nearby_tanks = tanks.closer(12.0, unit.position());
    if let Some(tank) = nearby_tanks.first() {
        if unit.distance(tank.position()) > 3.0 {
            move_no_spam(unit, Target::Pos(tank.position()));
            return true;
        } else {
            attack_no_spam(unit, Target::Pos(unit.position()));
            return true;
        }
    }
    false
}
fn flee_from_threat(bot: &mut Nikolaj, unit: &Unit, threat: Option<Unit>) -> bool {
    if let Some(threat_unit) = threat {
        // Use combat grid to find furthest point
        let combat_grid = combat_grid8_pathable(bot, unit.position(), 3.0);
        let furthest_point = combat_grid.into_iter().max_by(|a, b| {
            let dist_a = a.distance(threat_unit.position());
            let dist_b = b.distance(threat_unit.position());
            dist_a
                .partial_cmp(&dist_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        if let Some(point) = furthest_point {
            unit.move_to(Target::Pos(point), false);
            return true;
        }
        // Just run
        let escape_point = unit.position().towards(threat_unit.position(), 5.0);
        unit.move_to(Target::Pos(escape_point), false);
        return true;
    }
    false
}
pub fn flee_flying_unit(bot: &mut Nikolaj, unit: &Unit, surroundings: SurroundingsInfo) {
    let in_danger = surroundings.clone().threat_level > ThreatLevel::None;
    if surroundings.clone().closest_threat.is_some()
        || (in_danger && surroundings.clone().closest_structure.is_some())
    {
        if let Some(threat) = surroundings.clone().closest_threat {
            let retreat_position = unit.position().towards(threat.position(), -5.0);
            move_no_spam(unit, Target::Pos(retreat_position));
            return;
        } else if let Some(structure) = surroundings.clone().closest_structure {
            let retreat_position = unit.position().towards(structure.position(), -5.0);
            move_no_spam(unit, Target::Pos(retreat_position));
            return;
        }
    }
    let idle_point = bot.strategy_data.idle_point;
    move_no_spam(unit, Target::Pos(idle_point));
}

#[allow(unused)]
pub fn should_wait_for_tanks(bot: &Nikolaj, unit: &Unit) -> bool {
    let close_tanks = bot.units.my.units.of_type_including_alias(UnitTypeId::SiegeTank).closer(6.0, unit.position());
    let distanced_tanks = bot.units.my.units.of_type_including_alias(UnitTypeId::SiegeTank).closer(20.0, unit.position());
    close_tanks.is_empty() && !distanced_tanks.is_empty()
}

fn combat_grid4(position: Point2, distance: f32) -> Vec<Point2> {
    let x = position.x;
    let y = position.y;
    let d = distance;

    vec![
        Point2 { x: x - d, y },
        Point2 { x: x + d, y },
        Point2 { x, y: y - d },
        Point2 { x, y: y + d },
    ]
}

fn combat_grid8(position: Point2, distance: f32) -> Vec<Point2> {
    let mut grid = combat_grid4(position, distance);
    let x = position.x;
    let y = position.y;
    let d = distance;

    grid.extend(vec![
        Point2 { x: x - d, y: y - d },
        Point2 { x: x - d, y: y + d },
        Point2 { x: x + d, y: y - d },
        Point2 { x: x + d, y: y + d },
    ]);
    grid
}

fn combat_grid8_pathable(bot: &mut Nikolaj, position: Point2, distance: f32) -> Vec<Point2> {
    let grid = combat_grid8(position, distance);
    grid.into_iter().filter(|p| bot.is_pathable(*p)).collect()
}

pub fn attack_no_spam(unit: &Unit, target: Target) {
    const DISTANCE_THRESHOLD: f32 = 4.0;
    if let Target::Pos(target_position) = target {
        if unit.position().distance(target_position) < DISTANCE_THRESHOLD {
            return;
        }
        if let Some(ordered_position) = unit.target_pos() {
            if ordered_position.distance(target_position) < DISTANCE_THRESHOLD {
                return;
            }
        }
    }

    if let Some(ordered_target) = unit.target_tag() {
        if let Target::Tag(target_tag) = target {
            if ordered_target == target_tag {
                return;
            }
        }
    }

    unit.attack(target, false);
}

#[allow(unused)]
pub fn move_into_range(unit: &Unit, target: &Unit) {
    let target_weapon_range = target.real_range_vs(unit);
    let target_position = target.position();
    let desired_position = target_position.towards(unit.position(), target_weapon_range + 2.0);
    move_no_spam(unit, Target::Pos(desired_position));
}

pub fn move_no_spam(unit: &Unit, target: Target) {
    if let Target::Pos(target_position) = target {
        if unit.position().distance(target_position) < 3.0 {
            return;
        }
        if let Some(ordered_position) = unit.target_pos() {
            if ordered_position.distance(target_position) < 2.0 {
                return;
            }
        }
    }
    
    unit.move_to(target, false);
}

pub fn unsiege(bot: &mut Nikolaj, unit: &Unit) {
    let unsiege_timer = bot.combat_info.get_unsiege_timer(unit.tag());
    if let Some(timer) = unsiege_timer {
        if timer.unsiege_in <= 0.0 {
            bot.combat_info.remove_unsiege_timer(unit.tag());
            if unit.type_id() == UnitTypeId::SiegeTankSieged {
                unit.use_ability(AbilityId::UnsiegeUnsiege, false);
            } else if unit.type_id() == UnitTypeId::WidowMineBurrowed {
                unit.use_ability(AbilityId::BurrowUpWidowMine, false);
            }
        }
    } else {
        if unit.type_id() == UnitTypeId::SiegeTankSieged
            || unit.type_id() == UnitTypeId::WidowMineBurrowed
        {
            bot.combat_info.add_unsiege_timer(unit.tag());
        }
    }
}

pub fn siege_up(bot: &mut Nikolaj, unit: &Unit) {
    bot.combat_info.remove_unsiege_timer(unit.tag());
    if unit.type_id() == UnitTypeId::SiegeTank {
        unit.use_ability(AbilityId::SiegeModeSiegeMode, false);
    } else if unit.type_id() == UnitTypeId::WidowMine {
        unit.command(AbilityId::BurrowDownWidowMine, Target::None, false);
    }
}

pub fn force_unsiege(bot: &mut Nikolaj, unit: &Unit) {
    bot.combat_info.remove_unsiege_timer(unit.tag());
    if unit.type_id() == UnitTypeId::SiegeTankSieged {
        unit.use_ability(AbilityId::UnsiegeUnsiege, false);
    } else if unit.type_id() == UnitTypeId::WidowMineBurrowed {
        unit.use_ability(AbilityId::BurrowUpWidowMine, false);
    }
}

pub fn get_closest_harass_point(bot: &Nikolaj, unit: &Unit) -> Point2 {
    let harass_points = &bot.strategy_data.harass_points;
    let attack_point = bot.strategy_data.attack_point;

    if let Some(closest_point) = harass_points.iter().min_by(|a, b| {
        let dist_a = unit.distance(**a);
        let dist_b = unit.distance(**b);
        dist_a
            .partial_cmp(&dist_b)
            .unwrap_or(std::cmp::Ordering::Equal)
    }) {
        return *closest_point;
    } else {
        return attack_point;
    }
}

/// Returns a point where either x or y is adjusted to the closest map border
/// This creates an offset point for pathing along the map edge to reach harass points
#[allow(dead_code)]
pub fn get_harass_point_border_offset(bot: &Nikolaj, point: Point2) -> Point2 {
    let playable = &bot.game_info.playable_area;
    
    // Calculate distances to each border
    let dist_to_left = point.x - playable.x0 as f32;
    let dist_to_right = playable.x1 as f32 - point.x;
    let dist_to_bottom = point.y - playable.y0 as f32;
    let dist_to_top = playable.y1 as f32 - point.y;
    
    // Find the minimum distance
    let min_dist = dist_to_left.min(dist_to_right).min(dist_to_bottom).min(dist_to_top);
    
    // Return point with the closest coordinate adjusted to that border
    if min_dist == dist_to_left {
        Point2::new(playable.x0 as f32, point.y)
    } else if min_dist == dist_to_right {
        Point2::new(playable.x1 as f32, point.y)
    } else if min_dist == dist_to_bottom {
        Point2::new(point.x, playable.y0 as f32)
    } else {
        Point2::new(point.x, playable.y1 as f32)
    }
}

pub fn get_closest_repair_point(bot: &Nikolaj, unit: &Unit) -> Point2 {
    let repair_points = &bot.strategy_data.repair_points;
    let idle_point = bot.strategy_data.idle_point;
    if let Some(closest_point) = repair_points.iter().min_by(|a, b| {
        let dist_a = unit.distance(**a);
        let dist_b = unit.distance(**b);
        dist_a
            .partial_cmp(&dist_b)
            .unwrap_or(std::cmp::Ordering::Equal)
    }) {
        return *closest_point;
    } else {
        return idle_point;
    }
}
pub fn kd8_charge(bot: &Nikolaj, unit: &Unit, surroundings: &SurroundingsInfo) -> bool {
    if let Some(threat) = surroundings.clone().closest_threat {
        if let Some(abilities) = unit.abilities() {
            if !abilities.contains(&AbilityId::KD8ChargeKD8Charge) {
                return false;
            }
            // KD8 Charge pathfinding
            if let Some(path) = bot.get_path(threat.position(), unit.position(), PathfindingUnitType::Ground, false, false) {
                if let Some(target_point) = path.0.iter().nth(3) {
                    let target_position = *target_point;
                    unit.command(
                        AbilityId::KD8ChargeKD8Charge,
                        Target::Pos(target_position),
                        false,
                    );
                    return true;
                }
            }
            // Direct KD8 Charge
            let target_position = threat.position().towards(unit.position(), 4.0);
            unit.command(
                AbilityId::KD8ChargeKD8Charge,
                Target::Pos(target_position),
                false,
            );
            return true;
        }
    }
    false
}

pub fn banshee_cloak(unit: &Unit, surroundings: &SurroundingsInfo) -> bool {
    let in_danger = surroundings.clone().threat_level > ThreatLevel::None;
    if let Some(abilities) = unit.abilities() {
        if abilities.contains(&AbilityId::BehaviorCloakOnBanshee) && !unit.is_cloaked() {
            if surroundings.clone().closest_threat.is_some() || in_danger {
                unit.use_ability(AbilityId::BehaviorCloakOnBanshee, false);
                return true;
            }
        }
    }
    false
}


pub fn join_strategy(bot: &mut Nikolaj, unit: &Unit) {
    let offensive = bot.strategy_data.attack;
    let defensive = bot.strategy_data.defend;
    let attack_point = bot.strategy_data.attack_point;
    let defense_point = bot.strategy_data.defense_point;
    let idle_point = bot.strategy_data.idle_point;

    if defensive {
        join_army_to_point(bot, unit, defense_point);
    } else if offensive {
        join_army_to_point(bot, unit, attack_point);
    } else {
        move_no_spam(unit, Target::Pos(idle_point));
    }
}


fn join_army_to_point(bot: &mut Nikolaj, unit: &Unit, point: Point2) {
    let army_leader_tag = bot.strategy_data.army_leader_tag;
    let army_center = bot.strategy_data.army_center;
    if unit.tag() == army_leader_tag {
        join_formation(bot, unit);
        attack_no_spam(unit, Target::Pos(point));
    } else {
        let formation_assignment = join_formation(bot, unit);
        if let Some(assignment) = formation_assignment {
            if assignment.formation_leader != unit.tag() {
                unit.move_to(Target::Pos(assignment.position), false);
                return;
            }
        }
        if unit.position().distance(army_center) > 15.0 {
            attack_no_spam(unit, Target::Pos(army_center));
        }
        attack_no_spam(unit, Target::Pos(point));
    }
}

/// Assigns `unit` to the nearest open formation slot, creating a new formation if needed.
///
/// Returns the claimed position so the caller can decide what order to issue.
pub fn join_formation(bot: &mut Nikolaj, unit: &Unit) -> Option<CombatFormationAssignment> {
    // TODO: Walkable only
    // TODO: no standing on my depots
    let attack_point = bot.strategy_data.attack_point;
    let facing_angle = {
        let dx = attack_point.x - unit.position().x;
        let dy = attack_point.y - unit.position().y;
        dy.atan2(dx)
    };

    // Try to find a close slot in an existing formation
    for formation in bot.combat_info.formations.iter_mut() {
        if let Some(pos) = formation.closest_position(unit.position()) {
            if unit.position().distance(pos) < 10.0 {
                formation.claim_position(pos);
                return Some(CombatFormationAssignment {
                    position: pos,
                    formation_leader: formation.leader
                });
            }
        }
    }

    // No nearby formation — create a new one
    let new_formation = CombatFormation::new(unit.tag(), unit.position(), facing_angle, 2.0, 5, 8);
    let pos = new_formation.closest_position(unit.position());
    bot.combat_info.formations.push(new_formation.clone());

    if let Some(pos) = pos {
        // Claim from the just-pushed formation
        if let Some(formation) = bot.combat_info.formations.last_mut() {
            formation.claim_position(pos);
        }
        Some(CombatFormationAssignment {
            position: pos,
            formation_leader: new_formation.leader
        })
    } else {
        None
    }
}
