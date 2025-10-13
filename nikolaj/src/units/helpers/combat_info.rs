use crate::Nikolaj;
use rand::Rng;
use rust_sc2::prelude::*;
use std::collections::HashMap;


pub fn combat_info_step(bot: &mut Nikolaj) {
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
    bot.combat_info.heatmaps.clear();
}

#[derive(Default, Debug, Clone)]
pub struct CombatInfo {
    pub last_time: f32,
    pub unsiege_timer: Vec<UnsiegeTimer>,
    pub scanner_sweep_time: f32,
    pub heatmaps: HashMap<u64, Heatmap>,
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
    pub fn get_heatmap_for_unit(&mut self, unit_tag: u64) -> Heatmap {
        if let Some(heatmap) = self.heatmaps.get_mut(&unit_tag) {
            heatmap.clone()
        } else {
            let heatmap = create_heatmap_for_unit(unit_tag);
            self.heatmaps.insert(unit_tag, heatmap.clone());
            return heatmap;
        }
	}
}

#[derive(Debug, Clone)]
pub struct UnsiegeTimer {
    pub tag: u64,
    pub unsiege_in: f32
}


#[derive(Clone, Debug)]
pub struct Heatmap {
	pub unit_tag: u64,
	pub points: Vec<HeatPoint>,
}

impl Heatmap {
	pub fn add_heat_to_point(&mut self, point: Point2, intensity: f32) {
		if let Some(hp) = self.points.iter_mut().find(|hp| hp.position == point) {
			hp.intensity += intensity;
		} else {
			self.points.push(HeatPoint { position: point, intensity });
		}
	}
}

#[derive(Clone, Debug)]
pub struct HeatPoint {
	pub position: Point2,
	pub intensity: f32,
}

fn create_heatmap_for_unit(unit_tag: u64) -> Heatmap {
    Heatmap {
        unit_tag,
        points: Vec::new(),
    }
}