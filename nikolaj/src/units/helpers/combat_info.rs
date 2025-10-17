use crate::Nikolaj;
use rand::Rng;
use std::collections::HashMap;
use crate::units::helpers::heatmap::{Heatmap};


pub fn combat_info_step(bot: &mut Nikolaj) {
    siege_timer_step(bot);
    bot.combat_info.detection_step(bot.time);
    bot.combat_info.heatmaps.clear();
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
}

#[derive(Debug, Clone)]
pub struct UnsiegeTimer {
    pub tag: u64,
    pub unsiege_in: f32
}

