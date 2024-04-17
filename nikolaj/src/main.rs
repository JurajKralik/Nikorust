#![allow(non_snake_case)]
use std::collections::HashMap;

use rust_sc2::prelude::*;

mod ex_main;
mod strategy;

#[bot]
#[derive(Default)]
struct Nikolaj {
    iteration: usize,
    enemy_units_memory: Units,
    enemy_unit_types_memory: HashMap<UnitTypeId, i32>,
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
            println!("enemy_units_memory: {:?}", self.enemy_units_memory.tags());
        }
        Ok(())
    }
    fn on_end(&self, _result: GameResult) -> SC2Result<()> {
        println!("---------------------");
        println!("Game ended with result: {:?}", _result);
        Ok(())
    }
}

impl Nikolaj {
    
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
