use crate::helpers::construction::*;
use crate::Nikolaj;
use rust_sc2::prelude::*;


pub fn construct_barracks(bot: &mut Nikolaj) {
	if !should_try_build_barracks(bot) {
		return;
	}

	if let Some(pos) = find_barracks_placement(bot) {
		build(bot, pos, UnitTypeId::Barracks);
	}
}

pub fn barracks_control(bot: &mut Nikolaj) {
	handle_grounded_barracks(bot);
	handle_flying_barracks(bot);
}

fn should_try_build_barracks(bot: &Nikolaj) -> bool {
	// Basics
	// Don't build if one is already in progress
	if bot.already_pending(UnitTypeId::Barracks) > 0 {
		return false;
	}
	// Under construction
	for under_construction in bot.construction_info.under_construction.iter() {
		if under_construction.structure == UnitTypeId::Barracks {
			return false;
		}
	}
	// Can't afford it
	if !bot.can_afford(UnitTypeId::Barracks, false) {
		return false;
	}
	// Needs at least one supply depot
	let depot_count = bot.structure_count(UnitTypeId::SupplyDepot)
		+ bot.structure_count(UnitTypeId::SupplyDepotLowered);
	if depot_count == 0 {
		return false;
	}

	// Additional conditions
	// Max 4 barracks (built + flying + pending)
	let barracks_total = bot.structure_count(UnitTypeId::Barracks)
		+ bot.structure_count(UnitTypeId::BarracksFlying)
		+ bot.already_pending(UnitTypeId::Barracks);
	if barracks_total >= 4 {
		return false;
	}
	// Avoid building if a Barracks is flying
	if !bot
		.units
		.my
		.structures
		.of_type(UnitTypeId::BarracksFlying)
		.is_empty()
	{
		return false;
	}
	// Avoid building if there are idle Barracks
	if !bot
		.units
		.my
		.structures
		.of_type(UnitTypeId::Barracks)
		.idle()
		.is_empty()
	{
		return false;
	}
	// More barracks
	if bot.structure_count(UnitTypeId::Barracks) > 0 {
		let factories = bot.units.my.structures.of_type_including_alias(UnitTypeId::Factory);
		if factories.is_empty() || bot.minerals < 300 {
			return false;
		}
	}
	true
}

fn find_barracks_placement(bot: &Nikolaj) -> Option<Point2> {
	// Priority 1: Barracks at middle ramp
	if let Some(pos) = bot.ramps.my.barracks_in_middle() {
		if bot.can_place(UnitTypeId::Barracks, pos) {
			return Some(pos);
		}
	}

	// Priority 2: Placement on grid
	if let Some(pos) = get_placement_on_grid(bot) {
		if bot.can_place(UnitTypeId::Barracks, pos) {
			return Some(pos);
		}
	}

	// Priority 3: Near base towards enemy
	for base in &bot.units.my.townhalls {
		let pos = base.position().towards(bot.enemy_start, 4.0);
		if bot.can_place(UnitTypeId::Barracks, pos) {
			return Some(pos);
		}
	}

	None
}

fn handle_grounded_barracks(bot: &mut Nikolaj) {
	let Some(unit_type) = bot.macro_manager.barracks_priority else {
		return;
	};

	if bot.macro_manager.expand_priority && bot.get_unit_cost(unit_type).minerals > bot.minerals - 400 {
		return;
	}

	for barracks in bot.units.my.structures.of_type(UnitTypeId::Barracks).idle().clone() {
        if barracks.rally_targets().is_empty(){
            if let Some(base) = bot.units.my.townhalls.closest(barracks.position()) {
                barracks.smart(Target::Pos(base.position()), false);
            }
        }
		if requires_techlab(unit_type) {
			if !barracks.has_techlab() {
				try_build_techlab_or_lift(bot, &barracks);
			} else {
				barracks.train(unit_type, false);
			}
		} else {
			barracks.train(unit_type, false);
		}
	}
}

fn handle_flying_barracks(bot: &mut Nikolaj) {
	for barracks in bot.units.my.structures.of_type(UnitTypeId::BarracksFlying).idle() {
		if let Some(pos) = get_placement_on_grid(bot) {
			barracks.command(AbilityId::LandBarracks, Target::Pos(pos), false);
			continue;
		}

		for base in &bot.units.my.townhalls {
			let pos = base.position().towards(bot.enemy_start, 4.0);
			if bot.can_place(UnitTypeId::Barracks, pos) {
				barracks.command(AbilityId::LandBarracks, Target::Pos(pos), false);
				break;
			}
		}
	}
}

fn requires_techlab(unit_type: UnitTypeId) -> bool {
	matches!(unit_type, UnitTypeId::Marauder | UnitTypeId::Ghost)
}

fn try_build_techlab_or_lift(bot: &Nikolaj, barracks: &Unit) {
	let addon_pos = barracks.position().offset(2.5, -0.5);
	if bot.can_place(UnitTypeId::SupplyDepot, addon_pos) {
		barracks.command(AbilityId::BuildTechLabBarracks, Target::None, false);
	} else {
		barracks.command(AbilityId::LiftBarracks, Target::None, false);
	}
}
