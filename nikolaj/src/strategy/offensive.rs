use crate::Nikolaj;


pub fn decide_offensive(bot: &mut Nikolaj) {
    // TODO: Improve logic
    let enemy_supply = bot.strategy_data.get_enemy_army_supply();
    let my_supply = bot.supply_army;

    // Initial
    if enemy_supply == 0 {
        if my_supply > 12 {
            bot.strategy_data.attack = true;
        } else {
            bot.strategy_data.attack = false;
        }
        return;
    }
    // Midgame
    if enemy_supply < 100 {
        if my_supply >= enemy_supply as u32 + 10 {
            bot.strategy_data.attack = true;
        } else {
            bot.strategy_data.attack = false;
        }
        return;
    }
    // Lategame
    if my_supply >= enemy_supply as u32 - 10 {
        bot.strategy_data.attack = true;
    } else {
        bot.strategy_data.attack = false;
    }
}