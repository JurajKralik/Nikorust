use crate::helpers::construction::*;
use crate::Nikolaj;
use rust_sc2::prelude::*;


pub fn construct_factory(bot: &mut Nikolaj) {
	if !should_try_build_factory(bot) {
		return;
	}

	if let Some(pos) = find_factory_placement(bot) {
		build(bot, pos, UnitTypeId::Factory);
	}
}

pub fn factory_control(bot: &mut Nikolaj) {
	handle_grounded_factory(bot);
	handle_flying_factory(bot);
}

fn should_try_build_factory(bot: &Nikolaj) -> bool {
	// Don't build if one is already in progress
	if bot.already_pending(UnitTypeId::Factory) > 0 {
		return false;
	}

	// Can't afford it
	if !bot.can_afford(UnitTypeId::Factory, false) {
		return false;
	}

	// Needs at least one Barracks
	let has_barracks = bot.structure_count(UnitTypeId::Barracks)
		+ bot.structure_count(UnitTypeId::BarracksFlying);
	if has_barracks == 0 {
		return false;
	}

	// Max 4 factories
	let factory_total = bot.structure_count(UnitTypeId::Factory)
		+ bot.structure_count(UnitTypeId::FactoryFlying)
		+ bot.already_pending(UnitTypeId::Factory);
	if factory_total >= 4 {
		return false;
	}

	// Avoid building if a Factory is flying
	if !bot.units.my.structures.of_type(UnitTypeId::FactoryFlying).is_empty() {
		return false;
	}

	// Avoid building if there are idle factories
	if !bot.units.my.structures.of_type(UnitTypeId::Factory).idle().is_empty() {
		return false;
	}

	// Prioritize Starport before 2nd+ Factory
	if bot.structure_count(UnitTypeId::Factory) > 0 {
		let has_starport = bot.structure_count(UnitTypeId::Starport)
			+ bot.structure_count(UnitTypeId::StarportFlying)
			> 0;
		if !has_starport && bot.minerals < 300 {
			return false;
		}
	}

	true
}

fn find_factory_placement(bot: &Nikolaj) -> Option<Point2> {
	// Priority 1: Barracks middle ramp position (used for tech building clustering?)
	if let Some(pos) = bot.ramps.my.barracks_in_middle() {
		if bot.can_place(UnitTypeId::Barracks, pos) {
			return Some(pos);
		}
	}

	// Priority 2: Placement on grid
	if let Some(pos) = get_placement_on_grid(bot) {
		if bot.can_place(UnitTypeId::Factory, pos) {
			return Some(pos);
		}
	}

	// Priority 3: Near base, toward enemy
	for base in &bot.units.my.townhalls {
		let pos = base.position().towards(bot.enemy_start, 4.0);
		if bot.can_place(UnitTypeId::Factory, pos) {
			return Some(pos);
		}
	}

	None
}

fn handle_grounded_factory(bot: &mut Nikolaj) {
	let Some(unit_type) = bot.factory_priority else {
		return;
	};

	for factory in bot.units.my.structures.of_type(UnitTypeId::Factory).idle().clone() {
        if factory.rally_targets().is_empty(){
            if let Some(base) = bot.units.my.townhalls.first() {
                factory.smart(Target::Pos(base.position()), false);
            }
        }
		if requires_techlab_factory(unit_type) {
			if !factory.has_techlab() {
				try_build_factory_techlab_or_lift(bot, &factory);
			} else {
				factory.train(unit_type, false);
			}
		} else {
			factory.train(unit_type, false);
		}
	}
}

fn handle_flying_factory(bot: &mut Nikolaj) {
	for factory in bot.units.my.structures.of_type(UnitTypeId::FactoryFlying).idle() {
		if let Some(pos) = get_placement_on_grid(bot) {
			factory.command(AbilityId::LandFactory, Target::Pos(pos), false);
			continue;
		}

		for base in &bot.units.my.townhalls {
			let pos = base.position().towards(bot.enemy_start, 4.0);
			if bot.can_place(UnitTypeId::Factory, pos) {
				factory.command(AbilityId::LandFactory, Target::Pos(pos), false);
				break;
			}
		}
	}
}

fn requires_techlab_factory(unit_type: UnitTypeId) -> bool {
	matches!(
		unit_type,
		UnitTypeId::SiegeTank
			| UnitTypeId::Cyclone
			| UnitTypeId::Thor
	)
}

fn try_build_factory_techlab_or_lift(bot: &Nikolaj, factory: &Unit) {
	let addon_pos = factory.position().offset(2.5, -0.5);
	if bot.can_place(UnitTypeId::SupplyDepot, addon_pos) {
		factory.command(AbilityId::BuildTechLabFactory, Target::None, false);
	} else {
		factory.command(AbilityId::LiftFactory, Target::None, false);
	}
}
