use crate::{debug::types::*, units::helpers::surroundings::SurroundingsInfo};

#[derive(Clone)]
pub struct NikolajDebugger {
    pub time: f32,
    pub printing_full_resource_assignments: bool,
    pub printing_bases_assignments: bool,
    pub printing_workers_assignments: bool,
    pub printing_resources_assignments: bool,
    pub printing_full_repair_assignments: bool,
    pub printing_repair_targets_assignments: bool,
    pub printing_construction: bool,
    pub printing_full_construction_info: bool,
    pub printing_combat_info: bool,
    pub printing_build_order: bool,
    pub printing_research: bool,
    pub printing_enemy_army_snapshot: bool,
    pub displaying_worker_roles: bool,
    pub displaying_worker_mining_steps: bool,
    pub displaying_bases: bool,
    pub displaying_repair: bool,
    pub displaying_mining: bool,
    pub displaying_strategy_points: bool,
    pub displaying_details_selected: bool,
    pub displaying_heatmaps_selected: bool,
    pub displaying_surroundings_selected: bool,
    pub displaying_strategy_monitor: bool,
    pub displaying_main_path: bool,
    pub run_resource_assignments_checks: bool,
    pub workers_current_mining_steps: Vec<WorkersCurrentMiningStep>,
    pub unit_surroundings: Vec<SurroundingsInfo>
}

impl Default for NikolajDebugger {
    fn default() -> Self {
        #[cfg(any(feature = "wine_sc2", feature = "headless"))]
        {
            Self {
                time: 0.0,
                printing_full_resource_assignments: false,
                printing_bases_assignments: false,
                printing_workers_assignments: false,
                printing_resources_assignments: false,
                printing_full_repair_assignments: false,
                printing_repair_targets_assignments: false,
                printing_construction: false,
                printing_full_construction_info: false,
                printing_combat_info: false,
                printing_build_order: false,
                printing_research: false,
                printing_enemy_army_snapshot: false,
                displaying_worker_roles: false,
                displaying_worker_mining_steps: false,
                displaying_bases: true,
                displaying_repair: false,
                displaying_mining: true,
                displaying_strategy_points: true,
                displaying_details_selected: true,
                displaying_heatmaps_selected: true,
                displaying_surroundings_selected: true,
                displaying_strategy_monitor: true,
                displaying_main_path: true,
                run_resource_assignments_checks: false,
                workers_current_mining_steps: vec![],
                unit_surroundings: vec![]
            }
        }
        #[cfg(not(any(feature = "wine_sc2", feature = "headless")))]
        {
            Self {
                time: 0.0,
                printing_full_resource_assignments: false,
                printing_bases_assignments: false,
                printing_workers_assignments: false,
                printing_resources_assignments: false,
                printing_full_repair_assignments: false,
                printing_repair_targets_assignments: false,
                printing_construction: true,
                printing_full_construction_info: false,
                printing_combat_info: false,
                printing_build_order: false,
                printing_research: false,
                printing_enemy_army_snapshot: false,
                displaying_worker_roles: false,
                displaying_worker_mining_steps: false,
                displaying_bases: false,
                displaying_repair: false,
                displaying_mining: false,
                displaying_strategy_points: false,
                displaying_details_selected: true,
                displaying_heatmaps_selected: false,
                displaying_surroundings_selected: false,
                displaying_strategy_monitor: false,
                displaying_main_path: false,
                run_resource_assignments_checks: false,
                workers_current_mining_steps: vec![],
                unit_surroundings: vec![]
            }
        }
    }
}

impl NikolajDebugger {
    pub fn add_mining_step(&mut self, tag: u64, step: WorkersMiningSteps) {
        if let Some(existing) = self.workers_current_mining_steps.iter_mut().find(|w| w.tag == tag) {
            existing.step = step;
        } else {
            self.workers_current_mining_steps.push(WorkersCurrentMiningStep {
                tag,
                step,
            });
        }
    }

    pub fn add_surroundings(&mut self, surroundings: SurroundingsInfo) {
        self.unit_surroundings.push(surroundings)
    }
}