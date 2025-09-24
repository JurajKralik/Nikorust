use crate::Nikolaj;
use rust_sc2::prelude::*;
use crate::units::helpers::surroundings::*;


pub fn raven_control(bot: &mut Nikolaj, unit: &Unit) {
    let surroundings = get_surroundings_info(bot, unit);
    if surroundings.closest_threat.is_some() {
        unit.use_ability(AbilityId::BuildAutoTurretAutoTurret, false);
        return;
    }
    unit.attack(Target::Pos(bot.strategy_data.army_center), false);
}