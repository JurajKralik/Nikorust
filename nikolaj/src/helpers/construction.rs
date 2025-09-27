use crate::Nikolaj;
use rust_sc2::prelude::*;


pub fn get_placement_on_grid(
    bot: &Nikolaj
) -> Option<Point2> {
    let start = bot.start_location;
    let start_height = bot.get_z_height(start);
    let start_x = start.x as i32;
    let start_y = start.y as i32;
    let spacing_x = 7;
    let spacing_y = 3;
    let search_range = 25;

    for x_offset in (-search_range..search_range).step_by(spacing_x) {
        for y_offset in (-search_range..search_range).step_by(spacing_y) {
            // Make a corridor in the middle
            if (x_offset - start_x).abs() < 3 || (y_offset - start_y).abs() < 3 {
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
    let builder = get_builder(bot, Target::Pos(position));
    if builder.is_none() {
        return;
    }
    builder.unwrap().build(structure, position, false);
    let mut under_construction = UnderConstruction::default();
    under_construction.worker_tag = builder.unwrap().tag();
    under_construction.position = position;
    under_construction.structure = structure;
    under_construction.time_started = bot.time as f32 / 22.4;
    bot.construction_info.under_construction.push(under_construction);
    if bot.debugger.printing_construction {
        println!("Construction: Started building {:?} at {:?}", structure, bot.time);
    }
}

pub fn refresh_construction_info(bot: &mut Nikolaj) {
    let time_now = bot.time;
    for under_construction in bot.construction_info.under_construction.clone().iter() {
        if time_now - under_construction.time_started > 20.0 {
            bot.construction_info.under_construction.retain(|x| x.worker_tag != under_construction.worker_tag);
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConstructionInfo {
    pub under_construction: Vec<UnderConstruction>,
}

impl Default for ConstructionInfo {
    fn default() -> Self {
        ConstructionInfo {
            under_construction: vec![UnderConstruction::default()],
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

impl Default for UnderConstruction {
    fn default() -> Self {
        UnderConstruction {
            worker_tag: 0,
            position: Point2::new(0.0, 0.0),
            structure: UnitTypeId::NotAUnit,
            time_started: 0.0,
        }
    }
}