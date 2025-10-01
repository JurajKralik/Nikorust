use crate::Nikolaj;
use rust_sc2::prelude::*;
use crate::units::helpers::surroundings::*;
use crate::units::helpers::combat_movement::*;


pub fn raven_control(bot: &mut Nikolaj, unit: &Unit) {
    let surroundings = get_surroundings_info(bot, unit);
    if surroundings.closest_threat.is_some() {
        let placement_options = PlacementOptions {
			max_distance: 4,
			step: 1,
			random: false,
			addon: false,
        };
        let position = bot.find_placement(UnitTypeId::AutoTurret, unit.position(), placement_options).unwrap_or(unit.position());
        unit.command(AbilityId::BuildAutoTurretAutoTurret, Target::Pos(position), false);
        return;
    }
    attack_no_spam(unit, Target::Pos(bot.strategy_data.army_center));
}