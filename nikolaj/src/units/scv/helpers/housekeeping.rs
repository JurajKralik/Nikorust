use crate::Nikolaj;

pub fn reset_frame(bot: &mut Nikolaj) {
    bot.scvs.current_refineries.clear();
    bot.scvs.current_mineral_fields.clear();
    bot.scvs.current_gas_workers.clear();
    bot.scvs.current_mineral_workers.clear();
    bot.scvs.current_idle_workers.clear();
}
