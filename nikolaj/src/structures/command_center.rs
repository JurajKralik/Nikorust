use crate::Nikolaj;
use crate::helpers::construction::*;
use rust_sc2::prelude::*;

const CLOAK_AND_BURROW: &'static [UnitTypeId] = &[
    UnitTypeId::DarkTemplar,
    UnitTypeId::Mothership,
    UnitTypeId::Banshee,
    UnitTypeId::Ghost,
    UnitTypeId::WidowMine,
    UnitTypeId::WidowMineBurrowed,
    UnitTypeId::LurkerMP,
    UnitTypeId::LurkerMPBurrowed,
    UnitTypeId::RoachBurrowed,
];

pub fn construct_command_centers(bot: &mut Nikolaj) {
    // One at a time
    if bot.already_pending(UnitTypeId::CommandCenter) > 0 {
        return;
    }

    // Under construction
    for under_construction in bot.construction_info.under_construction.iter() {
        if under_construction.structure == UnitTypeId::CommandCenter {
            return;
        }
    }

    // Resources
    let minerals = bot.minerals;
    if minerals < 400 {
        return;
    }

    // Saturation check
    let command_centers = bot.units.my.townhalls.clone();
    let mut ideal_workers: usize = 0;
    for command_center in command_centers {
        ideal_workers += command_center.ideal_harvesters().unwrap_or(0) as usize;
    }
    if ideal_workers > bot.supply_workers as usize {
        return;
    }

    // Position check
    let position = bot.get_expansion();
    let position = match position {
        None => return,
        Some(pos) => pos.loc,
    };

    // Safety check
    let enemies_nearby = bot.units.enemy.units.closer(20.0, position);
    for unit in enemies_nearby {
        if unit.can_attack_ground() {
            return;
        }
    }
    
    // Expand
    let structure = UnitTypeId::CommandCenter;
    build(bot, position, structure);
}

pub fn townhall_control(bot: &mut Nikolaj) {
    for base in &bot.units.my.townhalls.ready() {
        if base.is_flying() {
            let enemies = bot.units.enemy.units.closer(15.0, base);
            let mut ground_threat: Option<Unit> = None;

            //Flee
            for enemy in enemies {
                if enemy.can_attack_air() {
                    base.move_to(
                        Target::Pos(base.position().towards(enemy.position(), -1.0)),
                        false,
                    );
                    continue;
                }
                if enemy.can_attack_ground() {
                    ground_threat = Some(enemy);
                }
            }
            //Cannot land
            if ground_threat.is_some() {
                base.stop(false);
                continue;
            }
            //Land
            if base.orders().is_empty() {
                if let Some(expansion) = bot.get_expansion() {
                    base.land(expansion.loc, false);
                }
            }
            continue;
        } else {
            //Lift
            if base.is_idle()
                && !bot
                    .units
                    .enemy
                    .units
                    .closer(15.0, base.position())
                    .is_empty()
                && base.health_percentage().unwrap() < 0.35
                && base.type_id() != UnitTypeId::PlanetaryFortress
            {
                base.lift(false);
                continue;
            }
            if base.type_id() == UnitTypeId::OrbitalCommand {
                //Scan
                if bot.time > bot.scanner_sweep_time && base.energy().unwrap() > 50 {
                    let mut scanned = false;
                    //Scan for cloaked units
                    let mut enemy_units = bot.units.enemy.units.clone();
                    enemy_units.sort(|u| u.distance(base));
                    for enemy in enemy_units {
                        if CLOAK_AND_BURROW.contains(&enemy.clone().type_id()) {
                            base.command(
                                AbilityId::ScannerSweepScan,
                                Target::Pos(enemy.clone().position()),
                                false,
                            );
                            bot.scanner_sweep_time = bot.time + 10.0;
                            scanned = true;
                            break;
                        }
                    }
                    //Finishing hidden bases
                    if let Some(closest) = bot.units.my.units.closest(bot.enemy_start) {
                        if closest.distance(bot.enemy_start) < 5.0 {
                            let structure = bot.units.enemy.structures.closest(closest);
                            if structure.is_none() {
                                for mineral in bot.units.mineral_fields.clone() {
                                    if mineral.is_visible() {
                                        continue;
                                    } else {
                                        base.command(
                                            AbilityId::ScannerSweepScan,
                                            Target::Pos(mineral.position()),
                                            false,
                                        );
                                        scanned = true;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    if scanned {
                        continue;
                    }
                }
                //Drop MULE
                let mut mule_drop = false;
                if let Some(energy) = base.energy() {
                    let mut energy_needed = 50;
                    if bot.strategy_data.enemy_cloaking {
                        energy_needed = 100;
                    }
                    if energy >= energy_needed {
                        let close_minerals = bot.units.mineral_fields.closer(10.0, base);
                        if let Some(max_contents_minerals) =
                            close_minerals.max(|u| u.mineral_contents().unwrap())
                        {
                            base.command(
                                AbilityId::CalldownMULECalldownMULE,
                                Target::Tag(max_contents_minerals.tag()),
                                false,
                            );
                            continue;
                        } else {
                            for other_base in bot.units.my.townhalls.clone()
                            {
                                let close_minerals =
                                    bot.units.mineral_fields.closer(10.0, &other_base);

                                if let Some(max_contents_minerals) =
                                    close_minerals.max(|u| u.mineral_contents().unwrap())
                                {
                                    base.command(
                                        AbilityId::CalldownMULECalldownMULE,
                                        Target::Tag(max_contents_minerals.tag()),
                                        false,
                                    );
                                    mule_drop = true;
                                    break;
                                }
                            }
                        }
                        if mule_drop {
                            continue;
                        }
                    }
                }
            }
            if base.is_idle() {
                //Morph to Planetary
                if base.type_id() == UnitTypeId::CommandCenter
                    && bot.can_afford(UnitTypeId::PlanetaryFortress, false)
                    && !bot
                        .units
                        .my
                        .structures
                        .of_type(UnitTypeId::EngineeringBay)
                        .ready()
                        .is_empty()
                    && (bot
                        .units
                        .my
                        .structures
                        .of_type_including_alias(UnitTypeId::OrbitalCommand)
                        .len()
                        > 1
                        || (bot.strategy_data.enemy_flooding
                            && bot
                                .units
                                .my
                                .structures
                                .of_type_including_alias(UnitTypeId::OrbitalCommand)
                                .len()
                                > 0))
                {
                    base.command(
                        AbilityId::UpgradeToPlanetaryFortressPlanetaryFortress,
                        Target::None,
                        false,
                    );
                    continue;
                }
                //Morph to Orbital
                if base.type_id() == UnitTypeId::CommandCenter
                    && bot.can_afford(UnitTypeId::OrbitalCommand, false)
                    && !bot
                        .units
                        .my
                        .structures
                        .of_type_including_alias(UnitTypeId::Barracks)
                        .ready()
                        .is_empty()
                {
                    base.command(
                        AbilityId::UpgradeToOrbitalOrbitalCommand,
                        Target::None,
                        false,
                    );
                    continue;
                }
                //SCVs
                if bot.units.my.workers.len() < (bot.units.my.townhalls.len() * 22)
                && bot.units.my.workers.len() + bot.already_pending(UnitTypeId::SCV) < 70 {
                    base.train(UnitTypeId::SCV, false);
                    continue;
                }
            }
        }
    }
}
