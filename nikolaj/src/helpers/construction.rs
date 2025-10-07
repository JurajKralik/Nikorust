use std::collections::HashMap;

use crate::Nikolaj;
use rust_sc2::prelude::*;


pub fn get_placement_on_grid(
    bot: &Nikolaj
) -> Option<Point2> {
    let start = bot.start_location;
    let start_height = bot.get_z_height(start);
    let spacing_x = 7;
    let spacing_y = 3;
    let search_range = 25;

    for x_offset in (-search_range..search_range).step_by(spacing_x) {
        for y_offset in (-search_range..search_range).step_by(spacing_y) {
            // Make a corridor in the middle
            if (x_offset as i32).abs() < 2 || (y_offset as i32).abs() < 2 {
                continue;
            }
            let position = start + Point2::new(x_offset as f32, y_offset as f32);
            for under_construction in bot.construction_info.under_construction.iter() {
                if position.distance(under_construction.position) < 2.0 {
                    continue;
                }
            }
            let check_addon = true;
            let grid_building_size = UnitTypeId::Barracks;
            if is_valid_building_position(bot, position, grid_building_size, check_addon, start_height) {
                return Some(position);
            }
        }
    }
    None
}

fn is_valid_building_position(
    bot: &Nikolaj, position: Point2, grid_building_size: UnitTypeId, check_addon: bool, start_height: f32) -> bool {
    let local_height = bot.get_z_height(position);
    if start_height != local_height {
        return false;
    }
    // Placement check
    if !bot.can_place(grid_building_size, position) {
        return false;
    }

    // Addon placement check
    if check_addon {
        let addon_position = position + Point2::new(2.5, -0.5);
        if !bot.can_place(UnitTypeId::SupplyDepot, addon_position) {
            return false;
        }
    }

    // Height check
    let height = bot.get_z_height(position);
    if height != local_height {
        return false;
    }
    true
}

pub fn get_builder(bot: &mut Nikolaj, target: Target) -> Option<&Unit> {
    match target {
        Target::None => None,

        Target::Tag(tag) => {
            let position = bot.units.vespene_geysers.get(tag)?.position();

            let worker_tag = bot.worker_allocator.get_closest_worker(&bot.units.clone(), position)?;
            bot.units.my.workers.iter().find_tag(worker_tag)
        }

        Target::Pos(pos) => {
            let position = pos;

            let worker_tag = bot.worker_allocator.get_closest_worker(&bot.units.clone(), position)?;
            bot.units.my.workers.iter().find_tag(worker_tag)
        }
    }
}

pub fn build(bot: &mut Nikolaj, position: Point2, structure: UnitTypeId) {
    if let Some(builder) = get_builder(bot, Target::Pos(position)){
        builder.build(structure, position, false);
        let under_construction = UnderConstruction {
            worker_tag: builder.tag(),
            position: position,
            structure: structure,
            time_started: bot.time,
        };
        bot.construction_info.under_construction.push(under_construction);
        if bot.debugger.printing_construction {
            println!("[DEBUGGER] {} Started building {:?} at {:?}", bot.debugger.time, structure, bot.time);
        }
    }
}

pub fn add_to_in_training(bot: &mut Nikolaj, unit: UnitTypeId, structure: Unit) {
    let in_training = InTraining {
        unit,
        time_started: bot.time
    };
    bot.construction_info.in_training.push(in_training);
    if bot.debugger.printing_construction {
        println!("[DEBUGGER] {} Started training {:?} at {:?} from {:?}", bot.debugger.time, unit, bot.time, structure.tag());
    }
}

pub fn is_in_training(bot: &Nikolaj, unit: UnitTypeId) -> bool {
    for in_training in bot.construction_info.in_training.iter() {
        if in_training.unit == unit {
            return true;
        }
    }
    false
}

pub fn construction_info_step(bot: &mut Nikolaj) {
    let time_now = bot.time;
    for under_construction in bot.construction_info.under_construction.clone().iter() {
        if time_now - under_construction.time_started > 5.0 {
            bot.construction_info.under_construction.retain(|x| x.worker_tag != under_construction.worker_tag);
        }
    }
    for in_training in bot.construction_info.in_training.clone().iter() {
        if time_now - in_training.time_started > 1.0 {
            bot.construction_info.in_training.retain(|x| x.unit != in_training.unit);
        }
    }
}

pub fn finish_constructions_without_worker(bot: &mut Nikolaj) {
    let structures_to_finish = get_structures_to_finish(bot);
    for structure in structures_to_finish.iter() {
        let worker = get_builder(bot, Target::Pos(structure.position().clone()));
        if let Some(worker) = worker {
            worker.smart(Target::Tag(structure.tag()), false);
            let worker_tag = worker.tag().clone();
            let structure_tag = structure.tag().clone();
            bot.construction_info.structures_being_finished.insert(structure_tag, worker_tag);
            if bot.debugger.printing_construction {
                println!("[DEBUGGER] {} Finishing construction of {:?} at {:?}", bot.debugger.time, structure.type_id(), bot.time);
            }
        }
    }
}

fn get_structures_to_finish(bot: &Nikolaj) -> Units {
    let mut structures_to_finish = Units::new();
    let unfinished_structures = bot.units.my.structures.not_ready().clone();
    let mut orders_targets: Vec<Target> = vec![];
    let workers = bot.units.my.workers.clone();

    for worker in workers.iter() {
        if let Some(order) = worker.order() {
            orders_targets.push(order.1);
        }
    }

    for structure in unfinished_structures.iter() {
        if structure.type_id().is_addon() || structure.type_id() == UnitTypeId::Refinery || structure.type_id() == UnitTypeId::RefineryRich {
            continue;
        }
        if bot
            .construction_info
            .structures_being_finished
            .get(&structure.tag())
            .and_then(|&worker_tag| bot.units.my.workers.iter().find_tag(worker_tag))
            .and_then(|worker| worker.order())
            .is_some_and(|(_, target, _)| target == Target::Tag(structure.tag()))
        {
            continue;
        }
        if structure.build_progress() >= 1.0 {
            continue;
        }
        let mut being_built = false;
        for target in orders_targets.iter() {
            match target {
                Target::Tag(tag) => {
                    if *tag == structure.tag() {
                        being_built = true;
                        break;
                    }
                }
                Target::Pos(pos) => {
                    if pos.distance(structure.position()) < 1.0 {
                        being_built = true;
                        break;
                    }
                }
                Target::None => {}
            }
        }
        if !being_built {
            structures_to_finish.push(structure.clone());
        }
    }

    structures_to_finish
}

pub fn cancel_constructions_in_danger(bot: &mut Nikolaj) {
    for structure in bot.units.my.structures.clone() {
        if structure.build_progress() >= 1.0 || structure.build_progress() <= 0.1 {
            continue;
        }
        if structure.health_percentage() > 0.1 {
            continue;
        }
        let enemies_close = bot.units.enemy.units.closer(12.0, structure.position());
        for enemy in enemies_close.iter() {
            if enemy.can_attack_ground() {
                structure.cancel_building(false);
                if bot.debugger.printing_construction {
                    println!("[DEBUGGER] {} Cancelled construction of {:?} at {:?}", bot.debugger.time, structure.type_id(), bot.time);
                }
                break;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConstructionInfo {
    pub under_construction: Vec<UnderConstruction>,
    pub structures_being_finished: HashMap<u64, u64>,
    pub in_training: Vec<InTraining>,
}

impl Default for ConstructionInfo {
    fn default() -> Self {
        ConstructionInfo {
            under_construction: vec![],
            structures_being_finished: HashMap::new(),
            in_training: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnderConstruction {
    pub worker_tag: u64,
    pub position: Point2,
    pub structure: UnitTypeId,
    pub time_started: f32,
}

#[derive(Debug, Clone)]
pub struct InTraining {
    pub unit: UnitTypeId,
    pub time_started: f32,
}