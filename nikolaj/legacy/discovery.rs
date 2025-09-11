use crate::Nikolaj;
use rust_sc2::prelude::*;
use std::collections::HashSet;

const GATHER_RADIUS: f32 = 10.0;

pub fn init_resources(bot: &mut Nikolaj) {
    let (valid_minerals, valid_refineries) = collect_valid_resource_tags(bot);

    // Fill per-frame caches
    for &tag in &valid_minerals {
        if let Some(mf) = bot.units.mineral_fields.iter().find_tag(tag) {
            bot.scvs.current_mineral_fields.push(mf.clone());
        }
    }
    for &tag in &valid_refineries {
        if let Some(rf) = bot.units.my.structures.iter().find_tag(tag) {
            bot.scvs.current_refineries.push(rf.clone());
        }
    }

    // Keep mining_distribution in sync with what’s actually valid
    let valid_all: HashSet<u64> = valid_minerals.union(&valid_refineries).cloned().collect();

    // Remove stale resources
    bot.scvs
        .mining_distribution
        .retain(|tag, _| valid_all.contains(tag));

    // Add missing ones
    for tag in valid_all {
        bot.scvs.mining_distribution.entry(tag).or_default();
    }
}

// Helper: build sets of minerals/refineries that are valid this frame
fn collect_valid_resource_tags(bot: &Nikolaj) -> (HashSet<u64>, HashSet<u64>) {
    let mut minerals = HashSet::new();
    let mut refineries = HashSet::new();

    for base in bot.units.my.townhalls.ready().clone() {
        let pos = base.position();

        // Minerals near the base
        for mf in bot.units.mineral_fields.closer(GATHER_RADIUS, pos) {
            minerals.insert(mf.tag());
        }

        // Ready refineries with gas near the base
        for rf in bot
            .units
            .my
            .structures
            .of_type_including_alias(UnitTypeId::Refinery)
            .closer(GATHER_RADIUS, pos)
            .ready()
        {
            if rf.vespene_contents().unwrap_or(0) > 0 {
                refineries.insert(rf.tag());
            }
        }
    }

    (minerals, refineries)
}
