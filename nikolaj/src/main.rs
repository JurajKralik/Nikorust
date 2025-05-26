#![allow(non_snake_case)]
use rust_sc2::{bot, prelude::*};

mod ex_main;
mod helpers;
mod structures;
use crate::structures::command_center::*;
use crate::structures::supply_depots::*;
use crate::structures::barracks::*;
use crate::structures::factory::*;
use crate::helpers::build_order::*;


#[bot]
#[derive(Default)]
struct Nikolaj {
    iteration: usize,
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
        decide_strategy(self);
        construct_command_centers(self);
        townhall_control(self);
        construct_supply_depots(self);
        supply_depots_control(self);
        construct_barracks(self);
        construct_factory(self);
        barracks_control(self);
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

    #[allow(dead_code)]
    fn unit_count(&self, unit_type: UnitTypeId) -> usize {
        self.units.my.units.of_type(unit_type).len()
    }

    #[allow(dead_code)]
    fn structure_count(&self, unit_type: UnitTypeId) -> usize {
        self.units.my.structures.ready().of_type(unit_type).len()
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
