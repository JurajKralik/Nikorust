#![allow(dead_code)]

use rust_sc2::prelude::*;
use std::collections::HashMap;
use crate::Nikolaj;


pub fn map_manager_step(bot: &mut Nikolaj) {
	bot.map_manager.heatmaps.clear();
}

#[derive(Default)]
pub struct MapManager {
	heatmaps: HashMap<u64, Heatmap>,
}

impl MapManager {
	pub fn get_heatmap_for_unit(&mut self, unit_tag: u64) -> &mut Heatmap {
		self.heatmaps.entry(unit_tag).or_insert_with(|| Heatmap {
			unit_tag,
			points: Vec::new(),
		})
	}
}

#[derive(Clone)]
pub struct Heatmap {
	unit_tag: u64,
	points: Vec<HeatPoint>,
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

#[derive(Clone)]
pub struct HeatPoint {
	position: Point2,
	intensity: f32,
}
