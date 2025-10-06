use crate::Nikolaj;
use crate::helpers::construction::*;
use rust_sc2::prelude::*;


pub fn construct_missile_turrets(bot: &mut Nikolaj) {
    if !should_try_build_missile_turrets(bot) {
        return;
    }
    for base in bot.units.my.townhalls.ready() {
        let turrets_nearby = bot.units.my.structures.of_type(UnitTypeId::MissileTurret).closer(13.0, base.position());
        if turrets_nearby.is_empty() {
            build_missile_turret(bot, base.position());
        }
    }
}

fn should_try_build_missile_turrets(bot: &Nikolaj) -> bool {
    if bot.units.my.structures.of_type(UnitTypeId::EngineeringBay).ready().is_empty() {
        return false;
    }
    if !bot.can_afford(UnitTypeId::MissileTurret, false) {
        return false;
    }
	for under_construction in bot.construction_info.under_construction.iter() {
		if under_construction.structure == UnitTypeId::MissileTurret {
			return false;
		}
	}
    if !bot.strategy_data.enemy_cloaking && !bot.strategy_data.enemy_flying_units && bot.minerals < 450 {
        return false;
    }
    true
}

fn build_missile_turret(bot: &mut Nikolaj, near: Point2) {
    let nearby_minerals = bot.units.mineral_fields.closer(13.0, near);
    if !nearby_minerals.is_empty() {
        if let Some(center) = nearby_minerals.center() {
            let position = near.towards(center, 4.0);
            if bot.can_place(UnitTypeId::MissileTurret, position) {
                build(bot, position, UnitTypeId::MissileTurret);
                return;
            } else if let Some(position) = bot.find_placement(UnitTypeId::MissileTurret, near, PlacementOptions::default()) {
                build(bot, position, UnitTypeId::MissileTurret);
                return;
            }
        }
    }
    let near = near.towards(bot.enemy_start, 4.0);
    if let Some(position) = bot.find_placement(UnitTypeId::MissileTurret, near, PlacementOptions::default()) {
        build(bot, position, UnitTypeId::MissileTurret);
    }
}