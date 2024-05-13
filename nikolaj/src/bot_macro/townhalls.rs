use crate::Nikolaj;
use rust_sc2::prelude::*;

pub(crate) fn bases_init(bot: &mut Nikolaj) {
    bot.bases.clear();

    for base in &bot.units.my.townhalls.ready() {
        if !base.is_flying() && !bot.units.mineral_fields.closer(10.0, base).is_empty() {
            bot.bases.push(base.tag());
        }
    }
}
pub(crate) fn townhall_control(bot: &mut Nikolaj) {
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
                if let Some(expansion) = bot.get_enemy_expansion() {
                    base.land(expansion.loc, false);
                }
            }
            continue;
        } else {
            //Lift
            if bot.defensive_point.is_some()
                && base.is_idle()
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
                    //Scan for cloaked units
                    let mut enemy_units = bot.units.enemy.units.clone();
                    enemy_units.sort(|u| u.distance(base));
                    for enemy in enemy_units {
                        
                    }
                }
                //Drop MULE
                let mut mule_drop = false;
                if let Some(energy) = base.energy() {
                    let mut energy_needed = 50;
                    if bot.enemy_cloaking {
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
                            for other_base in bot.units.my.structures.find_tags(&bot.bases.clone())
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
        }
    }
}
