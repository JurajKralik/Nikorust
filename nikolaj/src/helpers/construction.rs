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
        Target::None => {
            return None;
        }
        Target::Tag(tag) => {
            let position = bot.units.vespene_geysers.get(tag).unwrap().position();
            return bot
                .units
                .my
                .workers
                .iter()
                .filter(|u| !(u.is_constructing() || u.is_returning() || u.is_carrying_resource()))
                .closest(position);
        }
        Target::Pos(pos) => {
            let position = pos;
            return bot
                .units
                .my
                .workers
                .iter()
                .filter(|u| !(u.is_constructing() || u.is_returning() || u.is_carrying_resource()))
                .closest(position);
        }
    }
}