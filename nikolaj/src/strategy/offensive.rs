use rust_sc2::prelude::Distance;

use crate::Nikolaj;


pub fn decide_offensive(bot: &mut Nikolaj) {
    let enemy_supply = bot.strategy_data.get_enemy_army_supply();
    let my_supply = bot.supply_army;
    let defense_needed = bot.strategy_data.defend;
    let have_more_supply_than_enemy = my_supply >= enemy_supply as u32;
    let have_decent_supply = my_supply > 25;
    let maxed_out = my_supply >= 190;
    let army_center = bot.strategy_data.army_center;
    let army_is_closer_to_enemy_base = army_center.distance(bot.enemy_start) < army_center.distance(bot.start_location);
    let have_minimal_attack_supply = my_supply > 13;
    let can_attack = have_decent_supply && have_more_supply_than_enemy;

    bot.strategy_data.attack = false;

    if defense_needed {
        return;
    }

    if can_attack || maxed_out {
        bot.strategy_data.attack = true;
        return;
    }

    if have_minimal_attack_supply && army_is_closer_to_enemy_base {
        bot.strategy_data.attack = true;
        return;
    }
}