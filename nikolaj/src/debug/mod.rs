mod debugger;
mod display;
mod printing;
mod spawning;
mod types;

pub use debugger::*;
pub use types::*;

use crate::Nikolaj;

pub fn debug_step(bot: &mut Nikolaj) {
    display::debug_show_bases(bot);
    display::debug_show_mining(bot);
    display::debug_show_repair(bot);
    display::debug_show_worker_roles(bot);
    display::debug_show_strategy_points(bot);
    display::debug_show_worker_mining_steps(bot);
    display::debug_display_selected(bot);
    display::debug_show_heatmaps(bot);
    display::debug_show_strategy_monitor(bot);
    
    printing::debug_print_repair(bot);
    printing::debug_print_resource_assignments(bot);
    printing::debug_print_combat_info(bot);
    printing::debug_print_build_order(bot);
    printing::debug_print_full_construction_info(bot);
    printing::debug_print_enemy_army_snapshot(bot);
    printing::debug_resource_assignments_checks(bot);
    
    spawning::debug_spawn_unit(bot);
}

pub fn print_new_bases_assignments(old_bases: &Vec<u64>, new_bases: &Vec<u64>, time: f32) {
    printing::print_new_bases_assignments(old_bases, new_bases, time);
}
