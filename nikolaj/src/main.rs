use std::env;

use rust_sc2::{bot, geometry::*, prelude::*};

mod consts;
mod debug;
mod ex_main;
mod helpers;
mod structures;
mod units;

use crate::debug::*;
use crate::helpers::construction::*;
use crate::helpers::build_order::*;
use crate::helpers::strategy::*;
use crate::helpers::macro_manager::*;
use crate::structures::command_center::*;
use crate::structures::supply_depots::*;
use crate::structures::barracks::*;
use crate::structures::factory::*;
use crate::structures::refinery::*;
use crate::structures::starport::*;
use crate::structures::addons::*;
use crate::structures::engineering_bay::*;
use crate::structures::armory::*;
use crate::structures::missile_turrets::*;
use crate::units::scv::*;
use crate::units::helpers::combat_units::*;
use crate::units::helpers::combat_info::*;


#[bot]
#[derive(Default)]
struct Nikolaj {
    iteration: usize,
    debugger: NikolajDebugger,
    worker_allocator: WorkerAllocator,
    macro_manager: MacroManager,
    strategy_data: StrategyData,
    construction_info: ConstructionInfo,
    combat_info: CombatInfo,
}

impl Player for Nikolaj {
    fn get_player_settings(&self) -> PlayerSettings {
        PlayerSettings::new(Race::Terran)
    }
    fn on_start(&mut self) -> SC2Result<()> {
        self.start_game_report();
        Ok(())
    }
    fn on_step(&mut self, _iteration: usize) -> SC2Result<()> {
        self.time_step();

        scv_step(self);
        construction_info_step(self);
        decide_build_strategy(self);
        finish_constructions_without_worker(self);
        cancel_constructions_in_danger(self);
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
        construct_missile_turrets(self);
        construct_engineering_bay(self);
        engineering_bay_control(self);
        construct_armory(self);
        armory_control(self);
        addons_control(self);
        strategy_step(self);
        combat_info_step(self);
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
    fn time_step(&mut self) {
        self.iteration += 1;
        let time = (self.time * 10.0).round() / 10.0;
        self.debugger.time = time;
        self.worker_allocator.debugger.time = time;
    }
    fn start_game_report(&mut self) {
        let version_info = format!("Nikolaj version: {}", env!("CARGO_PKG_VERSION"));
        println!("---------------------");
        println!("On start:");
        println!("Map name: {}", self.game_info.map_name.clone());
        println!("{}", version_info);
        println!("{}", env!("CARGO_PKG_DESCRIPTION"));
        println!("---------------------");
        self.chat(version_info.as_str());
        self.chat("Good luck, have fun!");
    }
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

fn main() -> SC2Result<()> {
    #[cfg(feature = "wine_sc2")]
    {
        let mut bot = Nikolaj::default();
        return run_vs_computer(
            &mut bot,
            Computer::new(Race::Terran, Difficulty::VeryHard, Some(AIBuild::RandomBuild)),
            "BerlingradAIE",
            LaunchOptions {
                realtime: false,
                ..Default::default()
            },
        );
    }
    #[cfg(not(feature = "wine_sc2"))]
    {
        return ex_main::main(Nikolaj::default());
    }
}
