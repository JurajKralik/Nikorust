use crate::Nikolaj;
use rand::Rng;
use std::collections::HashMap;
use std::f32::consts::PI;
use crate::units::helpers::heatmap::{Heatmap};
use rust_sc2::prelude::*;


pub fn combat_info_step(bot: &mut Nikolaj) {
    siege_timer_step(bot);
    bot.combat_info.detection_step(bot.time);
    bot.combat_info.heatmaps.clear();
    bot.combat_info.formations.clear();
    let structures = bot.units.enemy.structures.clone();
    bot.combat_info.cleanup_dead_bunkers(&structures);
}

fn siege_timer_step(bot: &mut Nikolaj) {
    for timer in bot.combat_info.unsiege_timer.clone().iter() {
        if timer.unsiege_in < 0.0 {
            bot.combat_info.remove_unsiege_timer(timer.tag);
        }
    }
    let current_time = bot.time;
    let delta_time = current_time - bot.combat_info.last_time;
    bot.combat_info.last_time = current_time;
    for timer in bot.combat_info.unsiege_timer.iter_mut() {
        timer.unsiege_in -= delta_time;
    }
}

#[derive(Default, Debug, Clone)]
pub struct CombatInfo {
    pub last_time: f32,
    pub unsiege_timer: Vec<UnsiegeTimer>,
    pub scanner_sweep_time: f32,
    pub heatmaps: HashMap<u64, Heatmap>,
    pub detected: bool,
    pub detected_at: f32,
    pub formations: Vec<CombatFormation>,
    pub bunker_requests: HashMap<u64, Vec<u64>>
}

impl CombatInfo {
    pub fn get_unsiege_timer(&self, tag: u64) -> Option<&UnsiegeTimer> {
        self.unsiege_timer.iter().find(|t| t.tag == tag)
    }
    pub fn add_unsiege_timer(&mut self, tag: u64) {
        let new_time = rand::thread_rng().gen_range(2.0..6.0);
        self.remove_unsiege_timer(tag);
        self.unsiege_timer.push(UnsiegeTimer {
            tag,
            unsiege_in: new_time
        });
    }
    pub fn remove_unsiege_timer(&mut self, tag: u64) {
        self.unsiege_timer.retain(|t| t.tag != tag);
    }
    pub fn detection_step(&mut self, current_time: f32) {
        if !self.detected {
            return;
        }
        if current_time - self.detected_at > 9.0 {
            self.detected = false;
        }
    }
    pub fn get_bunker_by_unit(&self, tag: u64) -> Option<u64> {
        for (bunker_tag, bio_tags) in self.bunker_requests.iter() {
            if bio_tags.contains(&tag) {
                return Some(*bunker_tag);
            }
        }
        None
    }
    pub fn get_bunker_requests_for_bunker(&self, bunker_tag: u64) -> Option<Vec<u64>> {
        self.bunker_requests.get(&bunker_tag).cloned()
    }
    fn cleanup_dead_bunkers(&mut self, structures: &Units) {
        let mut dead_bunkers = Vec::new();
        for (bunker_tag, _bio_tags) in self.bunker_requests.iter() {
            if structures.iter().find_tag(*bunker_tag).is_none() {
                dead_bunkers.push(*bunker_tag);
            }
        }
        for dead in dead_bunkers.iter() {
            self.bunker_requests.remove(dead);
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnsiegeTimer {
    pub tag: u64,
    pub unsiege_in: f32
}

#[derive(Debug, Clone)]
pub struct CombatFormation {
    pub positions: Vec<Point2>,
    pub leader: u64
}

impl CombatFormation {
    /// Creates a formation grid facing `facing_angle`, centered at `origin`.
    ///
    /// Columns run along the facing direction (front-to-back),
    /// rows are perpendicular (left-to-right).
    pub fn new(leader: u64, origin: Point2, facing_angle: f32, spacing: f32, rows: i32, columns: i32) -> Self {
        let back_angle = facing_angle + PI;
        let dx = back_angle.cos();
        let dy = back_angle.sin();

        let perp_left_angle = facing_angle + 0.5 * PI;
        let px_l = perp_left_angle.cos();
        let py_l = perp_left_angle.sin();

        let mut positions = Vec::new();

        for c in 0..columns {
            let cx = origin.x + dx * spacing * c as f32;
            let cy = origin.y + dy * spacing * c as f32;

            let row_offset = (rows - 1) as f32 / 2.0;
            for r in 0..rows {
                let r_centered = r as f32 - row_offset;
                let slot = Point2::new(
                    cx + px_l * spacing * r_centered,
                    cy + py_l * spacing * r_centered,
                );
                positions.push(slot);
            }
        }
        CombatFormation { positions, leader }
    }

    /// Returns the closest available position to `pos`, or None if empty.
    pub fn closest_position(&self, pos: Point2) -> Option<Point2> {
        self.positions
            .iter()
            .min_by(|a, b| {
                pos.distance(**a)
                    .partial_cmp(&pos.distance(**b))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .copied()
    }

    /// Removes a specific position from the formation (slot claimed).
    pub fn claim_position(&mut self, pos: Point2) {
        if let Some(idx) = self.positions.iter().position(|p| *p == pos) {
            self.positions.swap_remove(idx);
        }
    }

    /// Removes all positions that fail the pathability check.
    ///
    /// Pass any closure that maps a grid coordinate to `true` when pathable,
    /// e.g. `|pos| bot.is_pathable(pos)`.
    pub fn retain_pathable<F>(&mut self, is_pathable: F)
    where
        F: Fn((usize, usize)) -> bool,
    {
        self.positions
            .retain(|p| is_pathable((p.x as usize, p.y as usize)));
    }
}

pub struct CombatFormationAssignment {
    pub position: Point2,
    pub formation_leader: u64
}