use crate::Nikolaj;
use rust_sc2::prelude::*;

pub fn strategy_step(bot: &mut Nikolaj) {
    refresh_idle_point(bot);
    refresh_defense_point(bot);
    refresh_attack_point(bot);
    refresh_army_center(bot);
    refresh_harass_points(bot);
    refresh_repair_points(bot);
    enemy_army_snapshot(bot);
    decide_offensive(bot);
    read_enemy_strategy(bot);
}
fn refresh_idle_point(bot: &mut Nikolaj) {
    let bases_amount = bot.units.my.townhalls.ready().len();
    if bases_amount == 1 {
        if let Some(base) = bot.units.my.townhalls.ready().first() {
            if base.position() == bot.start_location {
                let ramp = bot
                    .ramps
                    .my
                    .barracks_in_middle()
                    .unwrap_or(Point2 { x: 0.0, y: 0.0 });
                bot.strategy_data.idle_point = ramp.towards(bot.start_location, 8.0);
                return;
            }
        }
    }
    if bases_amount > 0 {
        let ready_townhalls = bot.units.my.townhalls.ready();
        let mut bases: Vec<_> = ready_townhalls.iter().collect();
        bases.sort_unstable_by(|a, b| b.tag().cmp(&a.tag()));
        let mut frontal_base: Option<&Unit> = None;
        for base in bases {
            if let Some(frontal) = frontal_base {
                let distance_to_enemy = base.position().distance(bot.enemy_start) + 5.0;
                let frontal_distance_to_enemy = frontal.position().distance(bot.enemy_start);
                if distance_to_enemy < frontal_distance_to_enemy {
                    frontal_base = Some(base);
                }
            } else {
                frontal_base = Some(base);
            }
        }
        if let Some(frontal) = frontal_base {
            bot.strategy_data.idle_point = frontal.position().towards(bot.enemy_start, 5.0);
            return;
        }
    }
    bot.strategy_data.idle_point = bot.start_location;
}
fn refresh_defense_point(bot: &mut Nikolaj) {
    let enemies = bot.units.enemy.units.clone();
    bot.strategy_data.defend = false;
    if enemies.is_empty() {
        return;
    }
    let mut closest_enemy: Option<Unit> = None;
    let mut closest_distance = f32::MAX;
    for enemy in enemies {
        if !enemy.can_attack() {
            continue;
        }
        if enemy.position().distance(bot.enemy_start) < bot.enemy_start.distance(bot.start_location)/2.0 {
            continue;
        }
        if let Some(closest) = closest_enemy.as_ref() {
            let closest_structure = bot.units.my.structures.closest(closest.position());
            if let Some(structure) = closest_structure {
                let distance = enemy.position().distance(structure.position());
                if distance < closest_distance {
                    closest_distance = distance;
                    closest_enemy = Some(enemy.clone());
                }
            }
        } else {
            closest_enemy = Some(enemy.clone());
            if let Some(closest) = closest_enemy.as_ref() {
                let closest_structure = bot.units.my.structures.closest(closest.position());
                if let Some(structure) = closest_structure {
                    closest_distance = enemy.position().distance(structure.position());
                }
            }
        }
    }
    if let Some(closest) = closest_enemy {
        let defense_point = {
            if let Some(closest_structure) = bot.units.my.structures.closest(closest.position()) {
                Some(
                    closest_structure
                        .position()
                        .towards(closest.position(), 4.0),
                )
            } else {
                None
            }
        };
        if let Some(point) = defense_point {
            bot.strategy_data.defend = true;
            bot.strategy_data.defense_point = point;
        } else {
            bot.strategy_data.defend = false;
        }
    }
}
fn refresh_attack_point(bot: &mut Nikolaj) {
    let enemy_structures = bot.units.enemy.structures.clone();
    enemy_structures.iter().sort_by_distance(bot.start_location);
    let my_units = bot.units.my.units.clone();
    let ramp = bot
        .ramps
        .enemy
        .barracks_in_middle()
        .unwrap_or(Point2 { x: 0.0, y: 0.0 })
        .towards(bot.enemy_start, -2.0);

    if !enemy_structures.is_empty() {
        let mut closest_structure: Option<Unit> = None;
        let mut closest_distance = f32::MAX;
        for structure in enemy_structures {
            if let Some(closest) = closest_structure.as_ref() {
                if closest.type_id().is_townhall() {
                    break;
                }
                let distance = structure.position().distance(bot.start_location);
                if distance < closest_distance || structure.type_id().is_townhall() {
                    closest_distance = distance;
                    closest_structure = Some(structure.clone());
                }
            } else {
                closest_structure = Some(structure.clone());
                closest_distance = structure.position().distance(bot.start_location);
            }
        }
        if let Some(closest) = closest_structure {
            bot.strategy_data.attack_point = closest.position();
            return;
        }
    } else if !my_units.is_empty() {
        let closest_unit = my_units.closest(bot.enemy_start);
        if let Some(unit) = closest_unit {
            let ramp_base_distance = ramp.distance(bot.enemy_start);
            let unit_base_distance = unit.position().distance(bot.enemy_start);
            if unit_base_distance > ramp_base_distance + 3.0 {
                bot.strategy_data.attack_point = ramp;
                return;
            } else {
                bot.strategy_data.attack_point = bot.enemy_start;
                return;
            }
        }
    } else {
        bot.strategy_data.attack_point = ramp;
    }
}
fn refresh_army_center(bot: &mut Nikolaj) {
    let attack_point = bot.strategy_data.attack_point;
    let my_units = bot.units.my.units.clone();
    // Prefer bio units for center
    if bot.unit_count(UnitTypeId::Marine) + bot.unit_count(UnitTypeId::Marauder) > 6 {
        let mut bio_units = Units::new();
        for unit in my_units.iter().sort_by_distance(attack_point) {
            if unit.type_id() != UnitTypeId::Marine && unit.type_id() != UnitTypeId::Marauder {
                continue;
            }
            if bio_units.len() >= 6 {
                break;
            }
            bio_units.push(unit.clone());
        }
        if let Some(center) = bio_units.center() {
            bot.strategy_data.army_center = center;
            return;
        }
    }
    // Any combat units
    let mut frontal_units = Units::new();
    for unit in my_units.iter().sort_by_distance(attack_point) {
        if frontal_units.len() >= 6 {
            break;
        }
        if unit.can_attack() && !unit.type_id().is_worker() && unit.type_id() != UnitTypeId::Banshee && unit.type_id() != UnitTypeId::Reaper {
            frontal_units.push(unit.clone());
        }
    }
    if let Some(center) = frontal_units.center() {
        bot.strategy_data.army_center = center;
    } else {
        bot.strategy_data.army_center = attack_point;
    }
}
fn refresh_harass_points(bot: &mut Nikolaj) {
    let enemy_bases = bot.units.enemy.townhalls.clone();
    if !enemy_bases.is_empty() {
        let mut harass_points: Vec<Point2> = Vec::new();
        for base in enemy_bases {
            let minerals = bot.units.mineral_fields.closer(13.0, base.position());
            let mineral_center = minerals.center();
            if let Some(center) = mineral_center {
                let harass_position = center.towards(base.position(), 2.0);
                harass_points.push(harass_position);
            } else {
                harass_points.push(base.position());
            }
        }
        bot.strategy_data.harass_points = harass_points;
    } else {
        let mut harass_points: Vec<Point2> = Vec::new();
        let minerals = bot.units.mineral_fields.closer(13.0, bot.enemy_start);
        let mineral_center = minerals.center();
        if let Some(center) = mineral_center {
            let harass_position = center.towards(bot.enemy_start, 2.0);
            harass_points.push(harass_position);
            bot.strategy_data.harass_points = harass_points;
            return;
        } else {
            harass_points.push(bot.enemy_start);
            bot.strategy_data.harass_points = harass_points;
            return;
        }
    }
}
fn refresh_repair_points(bot: &mut Nikolaj) {
    let bases = bot.units.my.townhalls.ready();
    let mut repair_points: Vec<Point2> = Vec::new();
    for base in bases {
        let workers = bot.units.my.workers.closer(20.0, base.position());
        if workers.len() > 5 {
            let repair_position = base.position().towards(bot.enemy_start, 3.0);
            repair_points.push(repair_position);
        }
    }
    if repair_points.is_empty() {
        repair_points.push(bot.start_location);
    }
    bot.strategy_data.repair_points = repair_points;
}
fn enemy_army_snapshot(bot: &mut Nikolaj) {
    let visible_enemies = get_visible_enemies(bot);
    let appended_snapshot = get_appended_enemy_army_snapshot(bot, visible_enemies);
    bot.strategy_data.enemy_army = appended_snapshot;
    delete_outdated_enemy_snapshots(bot);
    let dead_units = bot.state.observation.raw.dead_units.clone();
    delete_dead_enemy_snapshots(bot, dead_units);
}
fn get_visible_enemies(bot: &Nikolaj) -> Vec<UnitSnapshot> {
    let enemies = bot.units.enemy.units.clone();
    let mut currently_visible_army: Vec<UnitSnapshot> = Vec::new();
    for enemy in enemies {
        if !enemy.can_attack() || enemy.type_id().is_worker() {
            continue;
        }
        let health = (enemy.health().unwrap_or(0) + enemy.shield().unwrap_or(0)) as f32;
        let supply = enemy.supply_cost() as usize;
        let snapshot = UnitSnapshot {
            id: enemy.tag(),
            type_id: enemy.type_id(),
            position: enemy.position(),
            health: health,
            supply: supply,
            last_seen: bot.time,
            is_snapshot: false,
        };
        currently_visible_army.push(snapshot);
    }
    currently_visible_army
}
fn get_appended_enemy_army_snapshot(
    bot: &mut Nikolaj,
    visible_enemies: Vec<UnitSnapshot>,
) -> Vec<UnitSnapshot> {
    let mut current_snapshot: Vec<UnitSnapshot> = bot.strategy_data.enemy_army.clone();
    // Mark all as snapshot
    for current_enemy in current_snapshot.iter_mut() {
        current_enemy.is_snapshot = true;
    }
    // Update or add
    for visible in visible_enemies {
        if let Some(existing) = current_snapshot.iter_mut().find(|e| e.id == visible.id) {
            existing.position = visible.position;
            existing.health = visible.health;
            existing.last_seen = bot.time;
            existing.supply = visible.supply;
        } else {
            current_snapshot.push(visible.clone());
        }
    }
    current_snapshot
}
fn delete_outdated_enemy_snapshots(bot: &mut Nikolaj) {
    let current_time = bot.time;
    bot.strategy_data
        .enemy_army
        .retain(|snapshot| current_time - snapshot.last_seen <= 120.0);
}
fn delete_dead_enemy_snapshots(bot: &mut Nikolaj, dead_units: Vec<u64>) {
    bot.strategy_data
        .enemy_army
        .retain(|snapshot| !dead_units.contains(&snapshot.id));
}
fn decide_offensive(bot: &mut Nikolaj) {
    let enemy_supply = bot.strategy_data.get_army_supply();
    let my_supply = bot.supply_army;

    // Initial
    if enemy_supply == 0 {
        if my_supply > 12 {
            bot.strategy_data.attack = true;
        } else {
            bot.strategy_data.attack = false;
        }
        return;
    }
    // Midgame
    if enemy_supply < 100 {
        if my_supply >= enemy_supply as u32 + 10 {
            bot.strategy_data.attack = true;
        } else {
            bot.strategy_data.attack = false;
        }
        return;
    }
    // Lategame
    if my_supply >= enemy_supply as u32 - 10 {
        bot.strategy_data.attack = true;
    } else {
        bot.strategy_data.attack = false;
    }
}
fn read_enemy_strategy(bot: &mut Nikolaj) {
    // Worker rush and ramp blocking
    if bot.time < 60.0 * 3.0 {
        detect_enemy_worker_rush(bot);
        if bot.time - bot.strategy_data.enemy_worker_rush_time > 5.0 {
            bot.strategy_data.enemy_worker_rush = false;
        }
        detect_enemy_ramp_blocking(bot);
    }

    detect_enemy_flooding(bot);
    detect_cloaking_enemy(bot);
}
fn detect_enemy_worker_rush(bot: &mut Nikolaj) {
    let enemy_workers = bot.units.enemy.workers.clone();
    let mut offensive_workers = 0;
    for worker in enemy_workers {
        let distance_to_my_base = worker.position().distance(bot.start_location);
        let distance_to_enemy_base = worker.position().distance(bot.enemy_start);
        if distance_to_my_base < distance_to_enemy_base {
            offensive_workers += 1;
        }
    }
    if offensive_workers >= 4 {
        bot.strategy_data.enemy_worker_rush_time = bot.time;
        if !bot.strategy_data.enemy_worker_rush {
            println!(
                "Strategy: Detected enemy worker rush at time {:.1}",
                bot.time
            );
        }
        bot.strategy_data.enemy_worker_rush = true;
    }
}
fn detect_enemy_ramp_blocking(bot: &mut Nikolaj) {
    let enemy_workers = bot.units.enemy.workers.clone();
    let block_positions = bot.ramps.my.corner_depots();
    let ramp_center = bot.ramps.my.barracks_in_middle();
    if bot.strategy_data.enemy_ramp_blocking {
        if enemy_workers
            .closer(10.0, ramp_center.unwrap_or(Point2 { x: 0.0, y: 0.0 }))
            .is_empty()
        {
            bot.strategy_data.enemy_ramp_blocking = false;
            bot.strategy_data.enemy_ramp_blocking_steps = 0;
            println!(
                "Strategy: Enemy ramp blocking ended at time {:.1}",
                bot.time
            );
        }
    }
    let mut blocking = false;
    if let Some(positions) = block_positions {
        for position in positions {
            if !enemy_workers.closer(1.0, position).is_empty() {
                blocking = true;
                continue;
            }
        }
    }
    if blocking {
        bot.strategy_data.enemy_ramp_blocking_steps += 1;
        bot.strategy_data.enemy_ramp_blocking_time = bot.time;
    }
    if bot.time - bot.strategy_data.enemy_ramp_blocking_time > 3.0 {
        bot.strategy_data.enemy_ramp_blocking_steps = 0;
    }
    if bot.strategy_data.enemy_ramp_blocking_steps > 8 {
        bot.strategy_data.enemy_ramp_blocking = true;
        println!(
            "Strategy: Detected enemy ramp blocking at time {:.1}",
            bot.time
        );
    }
}
fn detect_enemy_flooding(bot: &mut Nikolaj) {
    if bot.time > 60.0 * 5.0 || bot.strategy_data.enemy_flooding {
        bot.strategy_data.enemy_flooding = false;
        return;
    }

    let enemy_units = bot.units.enemy.units.clone();
    let mut offensive_units = 0;
    for enemy in enemy_units {
        if !enemy.can_attack() || enemy.type_id().is_worker() {
            continue;
        }
        let distance_to_my_base = enemy.position().distance(bot.start_location);
        let distance_to_enemy_base = enemy.position().distance(bot.enemy_start);
        if distance_to_my_base < distance_to_enemy_base + 10.0 {
            offensive_units += 1;
        }
    }
    if offensive_units >= 5 {
        println!("Strategy: Detected enemy flooding at time {:.1}", bot.time);
        bot.strategy_data.enemy_flooding = true;
    }
}
fn detect_cloaking_enemy(bot: &mut Nikolaj) {
    if bot.strategy_data.enemy_cloaking {
        return;
    }

    let enemy_units = bot.strategy_data.enemy_army.clone();
    let cloaking_units = [
        UnitTypeId::Banshee,
        UnitTypeId::Ghost,
        UnitTypeId::WidowMine,
        UnitTypeId::WidowMineBurrowed,
        UnitTypeId::Roach,
        UnitTypeId::RoachBurrowed,
        UnitTypeId::DarkTemplar,
        UnitTypeId::Mothership,
        UnitTypeId::LurkerMP,
        UnitTypeId::LurkerMPBurrowed,
        UnitTypeId::LurkerMPEgg,
    ];
    for enemy in enemy_units {
        let enemy_type = enemy.type_id;
        if cloaking_units.contains(&enemy_type) {
            if !bot.strategy_data.enemy_cloaking {
                println!(
                    "Strategy: Detected enemy cloaking unit {:?} at time {:.1}",
                    enemy_type,
                    bot.time
                );
            }
            bot.strategy_data.enemy_cloaking = true;
            return;
        }
    }
    let enemy_structures = bot.units.enemy.structures.clone();
    let cloaking_structures = [
        UnitTypeId::DarkShrine,
        UnitTypeId::LurkerDenMP,
        UnitTypeId::GhostAcademy,
        UnitTypeId::StarportTechLab,
    ];
    for structure in enemy_structures {
        let structure_type = structure.type_id();
        if cloaking_structures.contains(&structure_type) {
            if !bot.strategy_data.enemy_cloaking {
                println!(
                    "Strategy: Detected enemy cloaking structure {:?} at time {:.1}",
                    structure_type,
                    bot.time
                );
            }
            bot.strategy_data.enemy_cloaking = true;
            return;
        }
    }
}
pub struct StrategyData {
    pub enemy_army: Vec<UnitSnapshot>,
    pub idle_point: Point2,
    pub defense_point: Point2,
    pub attack_point: Point2,
    pub army_center: Point2,
    pub harass_points: Vec<Point2>,
    pub repair_points: Vec<Point2>,
    pub defend: bool,
    pub attack: bool,
    pub enemy_cloaking: bool,
    pub enemy_flooding: bool,
    pub enemy_worker_rush: bool,
    pub enemy_worker_rush_time: f32,
    pub enemy_ramp_blocking: bool,
    pub enemy_ramp_blocking_steps: usize,
    pub enemy_ramp_blocking_time: f32,
}

impl Default for StrategyData {
    fn default() -> Self {
        StrategyData {
            enemy_army: Vec::new(),
            idle_point: Point2::new(0.0, 0.0),
            defense_point: Point2::new(0.0, 0.0),
            attack_point: Point2::new(0.0, 0.0),
            army_center: Point2::new(0.0, 0.0),
            harass_points: Vec::new(),
            repair_points: Vec::new(),
            defend: false,
            attack: false,
            enemy_cloaking: false,
            enemy_flooding: false,
            enemy_worker_rush: false,
            enemy_worker_rush_time: -6.0,
            enemy_ramp_blocking: false,
            enemy_ramp_blocking_steps: 0,
            enemy_ramp_blocking_time: -6.0,
        }
    }
}

impl StrategyData {
    pub fn get_army_supply(&self) -> usize {
        self.enemy_army.iter().map(|unit| unit.supply).sum()
    }
}

#[derive(Clone)]
pub struct UnitSnapshot {
    pub id: u64,
    pub type_id: UnitTypeId,
    pub position: Point2,
    pub health: f32,
    pub supply: usize,
    pub last_seen: f32,
    pub is_snapshot: bool,
}
