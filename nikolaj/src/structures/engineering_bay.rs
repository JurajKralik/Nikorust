use crate::Nikolaj;
use crate::helpers::construction::*;
use rust_sc2::prelude::*;
use crate::consts::*;


pub fn construct_engineering_bay(bot: &mut Nikolaj) {
	if !should_try_build_engineering_bay(bot) {
		return;
	}

	if let Some(pos) = find_engineering_bay_placement(bot) {
		build(bot, pos, UnitTypeId::EngineeringBay);
	}
}

fn should_try_build_engineering_bay(bot: &Nikolaj) -> bool {
    // Basics
    // Needs at least one base
    if bot.units.my.townhalls.ready().is_empty() {
        return false;
    }
	// Under construction
	for under_construction in bot.construction_info.under_construction.iter() {
		if under_construction.structure == UnitTypeId::EngineeringBay {
			return false;
		}
	}
    // Don't build if one is already in progress
    if bot.already_pending(UnitTypeId::EngineeringBay) > 0 {
        return false;
    }
    // Max 1 Engineering Bay
    if bot.structure_count(UnitTypeId::EngineeringBay) > 0 {
        return false;
    }
	// Savings for expansion
	if bot.macro_manager.expand_priority && bot.get_unit_cost(UnitTypeId::EngineeringBay).minerals > bot.minerals.saturating_sub(400) {
		return false;
	}
    if !bot.can_afford(UnitTypeId::EngineeringBay, false) {
        return false;
    }

    // Additional conditions
    // Needs at least 8 bio units
    // TODO: Add flying enemies condition
    if bot.unit_count(UnitTypeId::Marine) + bot.unit_count(UnitTypeId::Marauder) < 8 && !bot.strategy_data.enemy_cloaking {
        return false;
    }
    true
}

fn find_engineering_bay_placement(bot: &Nikolaj) -> Option<Point2> {
	// Priority 1: Barracks middle ramp position (used for tech building clustering?)
	if let Some(pos) = bot.ramps.my.barracks_in_middle() {
		if bot.can_place(UnitTypeId::EngineeringBay, pos) {
			return Some(pos);
		}
	}

	// Priority 2: Placement on grid
	if let Some(pos) = get_placement_on_grid(bot) {
		if bot.can_place(UnitTypeId::EngineeringBay, pos) {
			return Some(pos);
		}
	}

	// Priority 3: Near base, toward enemy
	for base in &bot.units.my.townhalls {
		let pos = base.position().towards(bot.enemy_start, 4.0);
		if bot.can_place(UnitTypeId::EngineeringBay, pos) {
			return Some(pos);
		}
	}

	None
}

pub fn engineering_bay_control(bot: &mut Nikolaj) {
    // No units
    if bot.unit_count(UnitTypeId::Marine) + bot.unit_count(UnitTypeId::Marauder) < 8 {
        return;
    }

    for engineering_bay in bot.units.my.structures.of_type(UnitTypeId::EngineeringBay).ready().idle() {
        if let Some(next_upgrade) = get_next_upgrade(engineering_bay.clone()) {
            // Savings for expansion
            if bot.macro_manager.expand_priority && 250 > bot.minerals.saturating_sub(400) {
                return;
            }
            // TODO: Price check
            engineering_bay.use_ability(next_upgrade, false);
        }
    }
}

fn get_next_upgrade(engineering_bay: Unit) -> Option<AbilityId> {
    for ability in ENGINEERING_BAY_UPGRADE_ORDER.iter() {
        if engineering_bay.has_ability(*ability) {
            return Some(*ability);
        }
    }
    None
}