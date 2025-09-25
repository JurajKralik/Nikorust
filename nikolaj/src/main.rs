use rust_sc2::{bot, geometry::*, prelude::*};

mod debug;
mod ex_main;
mod helpers;
mod structures;
mod units;

use crate::debug::*;
use crate::helpers::construction::*;
use crate::helpers::strategy::*;
use crate::helpers::build_order::*;
use crate::structures::command_center::*;
use crate::structures::supply_depots::*;
use crate::structures::barracks::*;
use crate::structures::factory::*;
use crate::structures::refinery::*;
use crate::structures::starport::*;
use crate::units::scv::*;
use crate::units::helpers::combat_units::*;
use crate::units::helpers::combat_info::*;


#[bot]
#[derive(Default)]
struct Nikolaj {
    iteration: usize,
    worker_allocator: WorkerAllocator,
    debugger: NikolajDebugger,
    strategy_data: StrategyData,
    construction_info: ConstructionInfo,
    combat_info: CombatInfo,
    scanner_sweep_time: f32,
    barracks_priority: Option<UnitTypeId>,
    factory_priority: Option<UnitTypeId>,
    starport_priority: Option<UnitTypeId>,
    starter_reaper: bool,
    starter_banshee: bool,
}

impl Player for Nikolaj {
    fn get_player_settings(&self) -> PlayerSettings {
        PlayerSettings::new(Race::Terran)
    }
    fn on_start(&mut self) -> SC2Result<()> {
        println!("---------------------");
        println!("On start:");
        println!("Map name: {}", self.game_info.map_name);
        println!("---------------------");
        println!("On loop:");
        self.barracks_priority = None;
        self.factory_priority = None;
        self.starport_priority = None;
        self.starter_reaper = true;
        self.starter_banshee = true;
        self.worker_allocator.debugger = self.debugger.clone();
        Ok(())
    }
    fn on_step(&mut self, _iteration: usize) -> SC2Result<()> {
        self.iteration = _iteration;
        scv_step(self);
        combat_info_step(self);
        refresh_construction_info(self);
        decide_build_strategy(self);
        construct_command_centers(self);
        townhall_control(self);
        construct_refinery(self);
        construct_supply_depots(self);
        supply_depots_control(self);
        construct_barracks(self);
        construct_factory(self);
        construct_starport(self);
        barracks_control(self);
        factory_control(self);
        starport_control(self);
        strategy_step(self);
        army_step(self);
        debug_step(self);
        Ok(())
    }
    fn on_end(&self, _result: GameResult) -> SC2Result<()> {
        self.end_game_report(_result);
        Ok(())
    }
}

#[allow(dead_code)]
impl Nikolaj {
    fn end_game_report(&self, result: GameResult) {
        println!("---------------------");
        println!("On end:");
        println!("Map name: {}", self.game_info.map_name);
        println!("Result: {:?}", result);
        println!("---------------------");
    }

    fn already_pending(&self, unit_type: UnitTypeId) -> usize {
        self.counter().ordered().count(unit_type)
    }

    fn unit_count(&self, unit_type: UnitTypeId) -> usize {
        self.units.my.units.of_type(unit_type).len()
    }

    fn structure_count(&self, unit_type: UnitTypeId) -> usize {
        self.units.my.structures.ready().of_type(unit_type).len()
    }

    fn get_debug_color(&self, color: &str) -> Option<(u32, u32, u32)> {
        match color {
            "red" => Some((255, 0, 0)),
            "green" => Some((0, 255, 0)),
            "blue" => Some((0, 0, 255)),
            "cyan" => Some((0, 255, 255)),
            "yellow" => Some((255, 255, 0)),
            "magenta" => Some((255, 0, 255)),
            "white" => Some((255, 255, 255)),
            "orange" => Some((255, 165, 0)),
            "black" => Some((0, 0, 0)),
            _ => self.get_debug_color("white"),
        }
    }
    fn debug_translate_point(&self, position_p2: Point2) -> Point3 {
        let height = self.get_z_height(position_p2) + 1.0;
        Point3::new(position_p2.x, position_p2.y, height)
    }
    fn debug_cube(&mut self, center_p2: Point2, size: f32, color: &str) {
        let debug_color = self.get_debug_color(color);
        let center = self.debug_translate_point(center_p2);
        self.debug.draw_cube(center, size, debug_color);
    }
    fn debug_sphere(&mut self, center_p2: Point2, radius: f32, color: &str) {
        let debug_color = self.get_debug_color(color);
        let center = self.debug_translate_point(center_p2);
        self.debug.draw_sphere(center, radius, debug_color);
    }
    fn debug_line(&mut self, start_p2: Point2, end_p2: Point2, color: &str) {
        let debug_color = self.get_debug_color(color);
        let start = self.debug_translate_point(start_p2);
        let end = self.debug_translate_point(end_p2);
        self.debug.draw_line(start, end, debug_color);
    }
    fn debug_text(&mut self, text: &str, pos_p2: Point2, color: &str, size: Option<u32>) {
        let debug_color = self.get_debug_color(color);
        let pos = self.debug_translate_point(pos_p2);
        self.debug.draw_text_world(text, pos, debug_color, size);
    }
    fn debug_text_screen(&mut self, text: &str, pos: Point2, color: &str, size: Option<u32>) {
        let debug_color = self.get_debug_color(color);
        let position = Some((pos.x, pos.y));
        self.debug.draw_text_screen(text, position, debug_color, size);
    }
}


/* LADDER */
/* 
fn main() -> SC2Result<()> {
    ex_main::main(Nikolaj::default())
}
*/

/* VS AI*/
fn main() -> SC2Result<()> {
    let mut bot = Nikolaj::default();
    run_vs_computer(
        &mut bot,
        Computer::new(Race::Terran, Difficulty::VeryHard, None),
        "BerlingradAIE",
        LaunchOptions {
            realtime: false,
            ..Default::default()
        },
    )
}
