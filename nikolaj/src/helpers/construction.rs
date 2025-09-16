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
            let position = start + Point2::new(x_offset as f32, y_offset as f32);
            if is_valid_position(bot, position, start_height) {
                return Some(position);
            }
        }
    }
    None
}

fn is_valid_position(
    bot: &Nikolaj, position: Point2, start_height: f32) -> bool {
    // Placement check
    if !bot.can_place(UnitTypeId::Barracks, position) {
        return false;
    }

    // Addon placement check
    let addon_position = position + Point2::new(2.5, -0.5);
    if !bot.can_place(UnitTypeId::SupplyDepot, addon_position) {
        return false;
    }

    // Height check
    let height = bot.get_z_height(position);
    if height != start_height {
        return false;
    }
    true
}

pub fn get_builder(bot: &mut Nikolaj, target: Target) -> Option<&Unit> {
    match target {
        Target::None => None,

        Target::Tag(tag) => {
            // clone position first (immutable borrow ends here)
            let position = bot.units.vespene_geysers.get(tag)?.position();

            // now safe to borrow worker_allocator mutably
            let worker_tag = bot.worker_allocator.get_closest_worker(&bot.units.clone(), position)?;
            bot.units.my.workers.iter().find_tag(worker_tag)
        }

        Target::Pos(pos) => {
            // position is already given
            let position = pos;

            // do the mut borrow after we’re done with bot.units
            let worker_tag = bot.worker_allocator.get_closest_worker(&bot.units.clone(), position)?;
            bot.units.my.workers.iter().find_tag(worker_tag)
        }
    }
}

pub fn build(bot: &mut Nikolaj, position: Point2, structure: UnitTypeId) {
    if let Some(builder) = get_builder(bot, Target::Pos(position)) {
        builder.build(structure, position, false);
    }
}