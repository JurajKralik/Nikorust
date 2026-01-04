use crate::debug::types::*;

#[derive(Debug, Clone)]
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
    pub displaying_selected: bool,
    pub displaying_heatmaps: bool,
    pub run_resource_assignments_checks: bool,
    pub workers_current_mining_steps: Vec<WorkersCurrentMiningStep>,
}

impl Default for NikolajDebugger {
    fn default() -> Self {
        #[cfg(feature = "wine_sc2")]
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
                displaying_bases: false,
                displaying_repair: false,
                displaying_mining: true,
                displaying_strategy_points: false,
                displaying_selected: true,
                displaying_heatmaps: true,
                run_resource_assignments_checks: false,
                workers_current_mining_steps: vec![],
            }
        }
        #[cfg(not(feature = "wine_sc2"))]
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
                displaying_selected: true,
                displaying_heatmaps: false,
                run_resource_assignments_checks: false,
                workers_current_mining_steps: vec![],
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
}
