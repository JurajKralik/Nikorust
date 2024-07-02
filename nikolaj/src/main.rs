#![allow(non_snake_case)]
use rust_sc2::{bot, prelude::*};
use std::collections::HashMap;

mod ex_main;
mod modules;
mod params;

use crate::modules::build_order::*;
use crate::modules::buildings_micro::*;
use crate::modules::conditions::*;
use crate::modules::construction::*;
use crate::modules::scv::distribute_workers;
use crate::modules::strategy::*;
use crate::modules::townhalls::*;

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
    //mining
    last_loop_distributed: u32,
    bases: Vec<u64>,
    //enemy cheese
    worker_rush: bool,
    contain_rush: bool,
    ramp_blocker: Option<u64>,
    ramp_blocker_timer: usize,
    flooding: bool,
    //enemy strategy
    enemy_cloaking: bool,
    enemy_fliers: bool,
    enemy_heavy_fliers: bool,
    //points
    idle_point: Point2,
    main_army_point: Option<Point2>,
    defensive_point: Option<Point2>,
    offensive_point: Option<Point2>,
    repair_point: Point2,
    harass_point: Point2,
    //combat micro memory
    assembling: f32,
    scanner_sweep_time: f32,
    //macro
    saving_on: Vec<UnitTypeId>,
    idle_production: Vec<u64>,
    order_units: Vec<UnitTypeId>,
    next_multiproduction_structure: Option<UnitTypeId>,
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
        const UTILITY_STRUCTURES: &'static [UnitTypeId] = &[
            UnitTypeId::SupplyDepot,
            UnitTypeId::Refinery,
            UnitTypeId::EngineeringBay,
            UnitTypeId::Armory,
            UnitTypeId::Bunker,
            UnitTypeId::MissileTurret,
            UnitTypeId::CommandCenter,
        ];

        self.iteration = _iteration;
        bases_init(self);

        if _iteration % 5 == 0 && self.units.my.townhalls.len() > 0 {
            //set points
            if _iteration % 50 == 0 {
                set_idle_point(self);
                set_repair_point(self);
                set_harass_point(self);
            }
            set_main_army_point(self);
            set_defensive_point(self);
            set_offensive_point(self);

            //strategy reading
            units_memory(self);
            cheese_detection(self);
            enemy_macro_strategy(self);

            //buildings micro
            cancel_buildings(self);
            depot_micro(self);
            bunker_micro(self);
            set_rally_points(self);

            townhall_control(self);

            //utility structures
            for structure in UTILITY_STRUCTURES {
                if get_conditions(self, &structure) && self.can_afford(structure.clone(), false) {
                    construct(self, structure.clone());
                }
            }

            //build order
            self.idle_production.clear();
            self.order_units = decide_units(self);
            self.next_multiproduction_structure = decide_next_structure(self);
            execute_build_order(self);
            expand_production(self);

            finish_building_without_workers(self);
            distribute_workers(self);
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

    fn already_pending(&self, unit_type: UnitTypeId) -> usize {
        self.counter().ordered().count(unit_type)
    }

    fn my_unit_count(&self, unit_type: UnitTypeId) -> usize {
        self.units.my.units.of_type(unit_type).len() + self.already_pending(unit_type)
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
        Computer::new(Race::Terran, Difficulty::VeryHard, None),
        "BerlingradAIE",
        LaunchOptions {
            realtime: false,
            ..Default::default()
        },
    )
}
