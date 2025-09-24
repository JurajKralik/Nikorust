use crate::Nikolaj;
use rust_sc2::prelude::*;


pub fn medivac_control(bot: &mut Nikolaj, unit: &Unit) {
    if let Some(cargo) = unit.cargo_space_taken() {
        if cargo > 0 {
            unit.command(AbilityId::UnloadAllAtMedivac, Target::Pos(unit.position()), false);
            return;
        }
    }
    unit.attack(Target::Pos(bot.strategy_data.army_center), false);
}