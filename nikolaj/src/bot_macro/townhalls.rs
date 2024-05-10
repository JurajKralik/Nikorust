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
            //Scan
            if ((bot.time - bot.scanner_sweep_time as f32) > 10.0) || base.energy().unwrap() > 50 {
                
            }
            //Drop MULE
            if let (Some(energy), Some(mineral)) = (
                base.energy(),
                bot.units.mineral_fields.closest(base.position()),
            ) {
                let mut energy_needed = 50;
                if bot.enemy_cloaking {
                    energy_needed = 100;
                }

                if energy >= energy_needed {
                    if mineral.position().distance(base.position()) < 10.0 {
                        base.command(
                            AbilityId::CalldownMULECalldownMULE,
                            Target::Tag(mineral.tag()),
                            false,
                        );
                        continue;
                    }
                }
            }
        }
    }
}
