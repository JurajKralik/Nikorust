#![allow(non_snake_case)]
use std::collections::HashMap;

use rust_sc2::prelude::*;

mod ex_main;
mod strategy;

#[bot]
#[derive(Default)]
struct Nikolaj {
    iteration: usize,
    //memory
    enemy_units_memory: Units,
    enemy_unit_types_memory: HashMap<UnitTypeId, i32>,
    enemy_structures_memory: Units,
    enemy_structure_types_memory: HashMap<UnitTypeId, i32>,
    my_units_memory: Units,
    my_unit_types_memory: HashMap<UnitTypeId, i32>,
    my_structures_memory: Units,
    my_structure_types_memory: HashMap<UnitTypeId, i32>,
    //enemy strategy
    worker_rush: bool,
    contain_rush: bool,
    ramp_blocker: Option<u64>,
    ramp_blocker_timer: usize,
    flooding: bool,
    //points
    idle_point: Point2
}

impl Player for Nikolaj {
    fn get_player_settings(&self) -> PlayerSettings {
        PlayerSettings::new(Race::Terran)
    }
    fn on_start(&mut self) -> SC2Result<()> {
        println!("---------------------");
        println!("On start:");
        // Split workers
        for worker in &self.units.my.workers {
            worker.gather(
                self.units.mineral_fields.closest(worker).unwrap().tag(),
                false,
            );
        }

        println!("---------------------");
        println!("On loop:");
        Ok(())
    }
    fn on_step(&mut self, _iteration: usize) -> SC2Result<()> {
        self.iteration = _iteration;
        if _iteration % 5 == 0 && self.units.my.townhalls.len() > 0 {
            strategy::units_memory(self);
        }
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
        println!(" Enemy units memory:");
        for (unit_type, count) in &self.enemy_unit_types_memory {
            println!("# {:?}: {}", unit_type, count);
        }
        println!(" Enemy structures memory:");
        for (unit_type, count) in &self.enemy_structure_types_memory {
            println!("# {:?}: {}", unit_type, count);
        }
        println!(" My units memory:");
        for (unit_type, count) in &self.my_unit_types_memory {
            println!("# {:?}: {}", unit_type, count);
        }
        println!(" My structures memory:");
        for (unit_type, count) in &self.my_structure_types_memory {
            println!("# {:?}: {}", unit_type, count);
        }
        println!("---------------------");
    }
}
/*
fn main() -> SC2Result<()> {
    ex_main::main(Nikolaj::default())
}*/

fn main() -> SC2Result<()> {
    let mut bot = Nikolaj::default();
    run_vs_computer(
        &mut bot,
        Computer::new(Race::Random, Difficulty::VeryHard, None),
        "BerlingradAIE",
        LaunchOptions {
            realtime: false,
            ..Default::default()
        },
    )
}
