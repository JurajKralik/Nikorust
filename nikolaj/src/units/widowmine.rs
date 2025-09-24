use crate::Nikolaj;
use crate::units::helpers::surroundings::*;
use rust_sc2::prelude::*;


pub fn widowmine_control(bot: &mut Nikolaj, unit: &Unit) {
    let surroundings = get_surroundings_info(bot, unit);
    if surroundings.best_target_in_range.is_some() || surroundings.better_target_off_range.is_some() || surroundings.closest_threat.is_some() {
        if unit.type_id() == UnitTypeId::WidowMine {
            unit.use_ability(AbilityId::BurrowDownWidowMine, false);
            return;
        }
    } else if unit.type_id() == UnitTypeId::WidowMineBurrowed {
        unit.use_ability(AbilityId::BurrowUpWidowMine, false);
        return;
    }
    unit.attack(Target::Pos(bot.strategy_data.army_center), false);
}