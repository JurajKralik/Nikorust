use crate::Nikolaj;
use rust_sc2::prelude::*;
use crate::units::helpers::combat_movement::*;


pub fn thor_control(bot: &mut Nikolaj, unit: &Unit) {
    if let Some(abilities) = unit.abilities() {
        if abilities.contains(&AbilityId::MorphThorHighImpactMode) {
            unit.use_ability(AbilityId::MorphThorHighImpactMode, false);
            return;
        }
    }
    attack_no_spam(unit, Target::Pos(bot.strategy_data.army_center));
}