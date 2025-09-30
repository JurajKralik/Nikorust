use crate::helpers::construction::*;
use crate::Nikolaj;
use rust_sc2::prelude::*;

pub fn construct_starport(bot: &mut Nikolaj) {
	if !should_try_build_starport(bot) {
		return;
	}

	if let Some(pos) = find_starport_placement(bot) {
		build(bot, pos, UnitTypeId::Starport);
	}
}

pub fn starport_control(bot: &mut Nikolaj) {
	handle_grounded_starport(bot);
	handle_flying_starport(bot);
}

fn should_try_build_starport(bot: &Nikolaj) -> bool {
	// Basics
	// Don't build if one is already in progress
	if bot.already_pending(UnitTypeId::Starport) > 0 {
		return false;
	}
	// Under construction
	for under_construction in bot.construction_info.under_construction.iter() {
		if under_construction.structure == UnitTypeId::Starport {
			return false;
		}
	}
	// Can't afford it
	if !bot.can_afford(UnitTypeId::Starport, false) {
		return false;
	}
	// Needs at least one Factory
	let has_factory = bot.structure_count(UnitTypeId::Factory)
		+ bot.structure_count(UnitTypeId::FactoryFlying);
	if has_factory == 0 {
		return false;
	}
	// Additional conditions
	// Max 4 Starports
	let starport_total = bot.structure_count(UnitTypeId::Starport)
		+ bot.structure_count(UnitTypeId::StarportFlying)
		+ bot.already_pending(UnitTypeId::Starport);
	if starport_total >= 4 {
		return false;
	}
	// Avoid building if one is flying
	if !bot.units.my.structures.of_type(UnitTypeId::StarportFlying).is_empty() {
		return false;
	}
	// Avoid building if there are idle Starports
	if !bot.units.my.structures.of_type(UnitTypeId::Starport).idle().is_empty() {
		return false;
	}
	// At least 2 Factories before 2nd+ Starport
	if starport_total > 0 {
		let factory = bot.structure_count(UnitTypeId::Factory) + bot.structure_count(UnitTypeId::FactoryFlying);
		if factory < 2 {
			return false;
		}
	}
	true
}

fn find_starport_placement(bot: &Nikolaj) -> Option<Point2> {
	// Priority 1: Barracks middle ramp
	if let Some(pos) = bot.ramps.my.barracks_in_middle() {
		if bot.can_place(UnitTypeId::Barracks, pos) {
			return Some(pos);
		}
	}

	// Priority 2: On build grid
	if let Some(pos) = get_placement_on_grid(bot) {
		if bot.can_place(UnitTypeId::Starport, pos) {
			return Some(pos);
		}
	}

	// Priority 3: Near base toward enemy
	for base in &bot.units.my.townhalls {
		let pos = base.position().towards(bot.enemy_start, 4.0);
		if bot.can_place(UnitTypeId::Starport, pos) {
			return Some(pos);
		}
	}

	None
}

fn handle_grounded_starport(bot: &mut Nikolaj) {
	let Some(unit_type) = bot.macro_manager.starport_priority else {
		return;
	};

	if bot.macro_manager.expand_priority && bot.get_unit_cost(unit_type).minerals > bot.minerals - 400 {
		return;
	}

	for starport in bot.units.my.structures.of_type(UnitTypeId::Starport).idle().clone() {
		if starport.rally_targets().is_empty() {
			if let Some(base) = bot.units.my.townhalls.closest(starport.position()) {
				starport.smart(Target::Pos(base.position()), false);
			}
		}

		if requires_techlab_starport(unit_type) {
			if !starport.has_techlab() {
				try_build_starport_techlab_or_lift(bot, &starport);
			} else {
				starport.train(unit_type, false);
			}
		} else {
			starport.train(unit_type, false);
		}
	}
}

fn handle_flying_starport(bot: &mut Nikolaj) {
	for starport in bot.units.my.structures.of_type(UnitTypeId::StarportFlying).idle() {
		if let Some(pos) = get_placement_on_grid(bot) {
			starport.command(AbilityId::LandStarport, Target::Pos(pos), false);
			continue;
		}

		for base in &bot.units.my.townhalls {
			let pos = base.position().towards(bot.enemy_start, 4.0);
			if bot.can_place(UnitTypeId::Starport, pos) {
				starport.command(AbilityId::LandStarport, Target::Pos(pos), false);
				break;
			}
		}
	}
}

fn requires_techlab_starport(unit_type: UnitTypeId) -> bool {
	matches!(
		unit_type,
		UnitTypeId::Banshee | UnitTypeId::Raven | UnitTypeId::Battlecruiser
	)
}

fn try_build_starport_techlab_or_lift(bot: &Nikolaj, starport: &Unit) {
	let addon_pos = starport.position().offset(2.5, -0.5);
	if bot.can_place(UnitTypeId::SupplyDepot, addon_pos) {
		starport.command(AbilityId::BuildTechLabStarport, Target::None, false);
	} else {
		starport.command(AbilityId::LiftStarport, Target::None, false);
	}
}
