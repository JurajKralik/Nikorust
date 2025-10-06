use crate::Nikolaj;
use rust_sc2::prelude::*;
use crate::units::helpers::combat_movement::*;


pub fn medivac_control(bot: &mut Nikolaj, unit: &Unit) {
    if let Some(cargo) = unit.cargo_space_taken() {
        if cargo > 0 {
            unit.command(AbilityId::UnloadAllAtMedivac, Target::Tag(unit.tag()), false);
            return;
        }
    }
    attack_no_spam(unit, Target::Pos(bot.strategy_data.army_center));
}