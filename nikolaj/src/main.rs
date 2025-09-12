#![allow(non_snake_case)]
use rust_sc2::{bot, debug::Debugger, geometry::*, prelude::*};

mod ex_main;
mod helpers;
mod structures;
mod units;
use crate::structures::command_center::*;
use crate::structures::supply_depots::*;
use crate::structures::barracks::*;
use crate::structures::factory::*;
use crate::structures::refinery::*;
use crate::structures::starport::*;
use crate::helpers::build_order::*;
use crate::units::scv::*;


#[bot]
#[derive(Default)]
struct Nikolaj {
    debugger: Debugger,
    iteration: usize,
    worker_allocator: WorkerAllocator,
    scanner_sweep_time: f32,
    enemy_cloaking: bool,
    enemy_flooding: bool,
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
        Ok(())
    }
    fn on_step(&mut self, _iteration: usize) -> SC2Result<()> {
        self.iteration = _iteration;
        scv_step(self);
        decide_strategy(self);
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
        Ok(())
    }
    fn on_end(&self, _result: GameResult) -> SC2Result<()> {
        self.end_game_report(_result);
        Ok(())
    }
}

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
            "Red" => Some((255, 0, 0)),
            "Green" => Some((0, 255, 0)),
            "Blue" => Some((0, 0, 255)),
            "Cyan" => Some((0, 255, 255)),
            "Yellow" => Some((255, 255, 0)),
            "Magenta" => Some((255, 0, 255)),
            "White" => Some((255, 255, 255)),
            "Black" => Some((0, 0, 0)),
            _ => Some((255, 255, 255)),
        }
    }
    fn debug_cube(&mut self, center: Point3, size: f32, color: &str) {
        let debug_color = self.get_debug_color(color);
        self.debugger.draw_cube(center, size, debug_color);
    }
    fn debug_sphere(&mut self, center: Point3, radius: f32, color: &str) {
        let debug_color = self.get_debug_color(color);
        self.debugger.draw_sphere(center, radius, debug_color);
    }
    fn debug_line(&mut self, start: Point3, end: Point3, color: &str) {
        let debug_color = self.get_debug_color(color);
        self.debugger.draw_line(start, end, debug_color);
    }
    fn debug_text(&mut self, text: &str, pos: Point3, color: &str, size: Option<u32>) {
        let debug_color = self.get_debug_color(color);
        self.debugger.draw_text_world(text, pos, debug_color, size);
    }
    fn debug_text_screen(&mut self, text: &str, pos: Point2, color: &str, size: Option<u32>) {
        let debug_color = self.get_debug_color(color);
        let position = Some((pos.x, pos.y));
        self.debugger.draw_text_screen(text, position, debug_color, size);
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
