use crate::Nikolaj;
use crate::helpers::construction::*;
use rust_sc2::prelude::*;
use crate::consts::*;


pub fn construct_armory(bot: &mut Nikolaj) {
	if !should_try_build_armory(bot) {
		return;
	}

	if let Some(pos) = find_armory_placement(bot) {
		build(bot, pos, UnitTypeId::Armory);
	}
}

fn should_try_build_armory(bot: &Nikolaj) -> bool {
    // Basics
    // Needs at least one Factory
    if bot.units.my.structures.of_type_including_alias(UnitTypeId::Factory).ready().is_empty() {
        return false;
    }
	// Under construction
	for under_construction in bot.construction_info.under_construction.iter() {
		if under_construction.structure == UnitTypeId::Armory {
			return false;
		}
	}
    // Don't build if one is already in progress
    if bot.already_pending(UnitTypeId::Armory) > 0 {
        return false;
    }
    // Max 1 Armory
    if bot.structure_count(UnitTypeId::Armory) > 0 {
        return false;
    }
	// Savings for expansion
	if bot.macro_manager.expand_priority && bot.get_unit_cost(UnitTypeId::Armory).minerals > bot.minerals.saturating_sub(400) {
		return false;
	}
    if !bot.can_afford(UnitTypeId::Armory, false) {
        return false;
    }

    // Additional conditions
    if bot.supply_army < 10 {
        return false;
    }
    if bot.units.my.structures.of_type(UnitTypeId::EngineeringBay).ready().is_empty() {
        return false;
    }
    true
}

fn find_armory_placement(bot: &Nikolaj) -> Option<Point2> {
	// Priority 1: Barracks middle ramp position (used for tech building clustering?)
	if let Some(pos) = bot.ramps.my.barracks_in_middle() {
		if bot.can_place(UnitTypeId::Armory, pos) {
			return Some(pos);
		}
	}

	// Priority 2: Placement on grid
	if let Some(pos) = get_placement_on_grid(bot) {
		if bot.can_place(UnitTypeId::Armory, pos) {
			return Some(pos);
		}
	}

	// Priority 3: Near base, toward enemy
	for base in &bot.units.my.townhalls {
		let pos = base.position().towards(bot.enemy_start, 4.0);
		if bot.can_place(UnitTypeId::Armory, pos) {
			return Some(pos);
		}
	}

	None
}

pub fn control_armory(bot: &mut Nikolaj) {
    let flying_army = bot.units.my.units.flying();
    let mut flying_army_supply_cost = 0.0;
    for unit in flying_army {
        let cost = bot.get_unit_cost(unit.type_id());
        flying_army_supply_cost += cost.supply;
    }
    let ground_army_supply_cost = bot.supply_army as f32 - flying_army_supply_cost - 10.0;
    
    if ground_army_supply_cost <= 0.0 && flying_army_supply_cost <= 0.0 {
        return;
    }

    if ground_army_supply_cost > flying_army_supply_cost {
        for armory in bot.units.my.structures.of_type(UnitTypeId::Armory).ready().idle() {
            if let Some(next_upgrade) = get_next_upgrade_ground(armory.clone()) {
                // Savings for expansion
                if bot.macro_manager.expand_priority && 250 > bot.minerals.saturating_sub(400) {
                    return;
                }
                if let Some(affordable) = bot.can_afford_ability_research(next_upgrade) {
                    if !affordable {
                        return;
                    }
                } else {
                    println!("Error checking affordability for ability {:?}", next_upgrade);
                }
                armory.use_ability(next_upgrade, false);
            }
        }
    } else {
        for armory in bot.units.my.structures.of_type(UnitTypeId::Armory).ready().idle() {
            if let Some(next_upgrade) = get_next_upgrade_flying(armory.clone()) {
                // Savings for expansion
                if bot.macro_manager.expand_priority && 250 > bot.minerals.saturating_sub(400) {
                    return;
                }
                if let Some(affordable) = bot.can_afford_ability_research(next_upgrade) {
                    if !affordable {
                        return;
                    }
                } else {
                    println!("Error checking affordability for ability {:?}", next_upgrade);
                }
                armory.use_ability(next_upgrade, false);
            }
        }
    }
}

fn get_next_upgrade_ground(armory: Unit) -> Option<AbilityId> {
    for ability in ARMORY_GROUND_UPGRADE_ORDER.iter() {
        if armory.has_ability(*ability) {
            return Some(*ability);
        }
    }
    None
}

fn get_next_upgrade_flying(armory: Unit) -> Option<AbilityId> {
    for ability in ARMORY_AIR_UPGRADE_ORDER.iter() {
        if armory.has_ability(*ability) {
            return Some(*ability);
        }
    }
    None
}