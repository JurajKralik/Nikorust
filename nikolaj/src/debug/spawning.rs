use crate::Nikolaj;
use rust_sc2::prelude::*;

pub fn debug_spawn_unit(bot: &mut Nikolaj) {
    let chat = bot.state.chat.clone();
    if chat.is_empty() {
        return;
    }

    let camera_pos = bot.state.observation.raw.camera;
    for chat_message in chat {
        let sender_id = chat_message.player_id;
        let message = chat_message.message.to_lowercase();
        
        let is_enemy = message.starts_with("enemy ");
        let target_player_id = if is_enemy {
            2
        } else {
            sender_id
        };
        
        if let Some(army_comp) = get_army_composition(&message) {
            println!("[DEBUGGER] {} Spawning army composition '{}' for player {}", 
                bot.debugger.time, message, target_player_id);
            bot.debug.create_units(&army_comp.iter()
                .map(|(unit_type, count)| (*unit_type, Some(target_player_id), camera_pos, *count))
                .collect::<Vec<_>>());
        } else if is_enemy {
            let parts: Vec<&str> = message.split_whitespace().collect();
            if parts.len() >= 2 {
                let unit_name = parts[1];
                let count = if parts.len() >= 3 {
                    parts[2].parse::<u32>().unwrap_or(5)
                } else {
                    5
                };
                
                if let Some(unit_type) = get_unit_from_string(unit_name) {
                    println!("[DEBUGGER] {} Spawning {} {:?} for player {}", 
                        bot.debugger.time, count, unit_type, target_player_id);
                    bot.debug.create_units(&[(unit_type, Some(target_player_id), camera_pos, count)]);
                } else {
                    println!("[DEBUGGER] {} Unknown unit type: {}", bot.debugger.time, unit_name);
                }
            }
        } else if let Some(unit_type) = get_unit_from_string(&message) {
            println!("[DEBUGGER] {} Spawning unit {:?} for player {}", 
                bot.debugger.time, unit_type, target_player_id);
            bot.debug.create_units(&[(unit_type, Some(target_player_id), camera_pos, 1)]);
        } else {
            println!("[CHAT] {} by {}: {}", bot.debugger.time, chat_message.player_id,chat_message.message);
        }
    }
}

fn get_army_composition(message: &str) -> Option<Vec<(UnitTypeId, u32)>> {
    match message {
        "enemy basic army" | "enemy basic" => Some(vec![
            (UnitTypeId::Marine, 4),
            (UnitTypeId::Marauder, 2),
            (UnitTypeId::SiegeTank, 1),
            (UnitTypeId::Medivac, 1),
        ]),
        "enemy bio army" | "enemy bio" => Some(vec![
            (UnitTypeId::Marine, 8),
            (UnitTypeId::Marauder, 4),
            (UnitTypeId::Medivac, 2),
        ]),
        "enemy mech army" | "enemy mech" => Some(vec![
            (UnitTypeId::SiegeTank, 3),
            (UnitTypeId::Thor, 2),
            (UnitTypeId::Cyclone, 2),
            (UnitTypeId::Hellion, 4),
        ]),
        "enemy marine push" => Some(vec![
            (UnitTypeId::Marine, 12),
            (UnitTypeId::Medivac, 1),
        ]),
        "enemy tank push" => Some(vec![
            (UnitTypeId::SiegeTank, 4),
            (UnitTypeId::Marine, 6),
        ]),
        "enemy all in" => Some(vec![
            (UnitTypeId::Marine, 10),
            (UnitTypeId::Marauder, 6),
            (UnitTypeId::SiegeTank, 2),
            (UnitTypeId::Medivac, 2),
        ]),
        "enemy massive" => Some(vec![
            (UnitTypeId::Marine, 20),
            (UnitTypeId::Marauder, 10),
            (UnitTypeId::SiegeTank, 4),
            (UnitTypeId::Medivac, 3),
        ]),
        
        "enemy ling flood" | "enemy zerglings" => Some(vec![
            (UnitTypeId::Zergling, 24),
        ]),
        "enemy roach push" | "enemy roaches" => Some(vec![
            (UnitTypeId::Roach, 12),
        ]),
        "enemy hydra push" | "enemy hydras" => Some(vec![
            (UnitTypeId::Hydralisk, 8),
        ]),
        "enemy bane bust" | "enemy banelings" => Some(vec![
            (UnitTypeId::Zergling, 12),
            (UnitTypeId::Baneling, 8),
        ]),
        "enemy roach hydra" => Some(vec![
            (UnitTypeId::Roach, 6),
            (UnitTypeId::Hydralisk, 6),
        ]),
        "enemy muta harass" | "enemy mutas" => Some(vec![
            (UnitTypeId::Mutalisk, 8),
        ]),
        "enemy zerg army" => Some(vec![
            (UnitTypeId::Roach, 8),
            (UnitTypeId::Hydralisk, 6),
            (UnitTypeId::Zergling, 12),
        ]),
        
        "enemy zealot rush" | "enemy zealots" => Some(vec![
            (UnitTypeId::Zealot, 8),
        ]),
        "enemy stalker push" | "enemy stalkers" => Some(vec![
            (UnitTypeId::Stalker, 8),
        ]),
        "enemy protoss army" | "enemy toss army" => Some(vec![
            (UnitTypeId::Stalker, 6),
            (UnitTypeId::Zealot, 4),
            (UnitTypeId::Immortal, 2),
        ]),
        "enemy carrier rush" | "enemy carriers" => Some(vec![
            (UnitTypeId::Carrier, 3),
        ]),
        "enemy void rays" | "enemy voids" => Some(vec![
            (UnitTypeId::VoidRay, 5),
        ]),
        "enemy colossus" => Some(vec![
            (UnitTypeId::Colossus, 2),
            (UnitTypeId::Stalker, 6),
            (UnitTypeId::Zealot, 4),
        ]),
        "enemy dt rush" | "enemy dark templars" => Some(vec![
            (UnitTypeId::DarkTemplar, 4),
        ]),
        
        "enemy worker rush" | "enemy probes" => Some(vec![
            (UnitTypeId::Probe, 12),
        ]),
        
        _ => None,
    }
}

fn get_unit_from_string(name: &str) -> Option<UnitTypeId> {
    match name {
        "marine" | "marines" => Some(UnitTypeId::Marine),
        "marauder" | "marauders" => Some(UnitTypeId::Marauder),
        "reaper" | "reapers" => Some(UnitTypeId::Reaper),
        "ghost" | "ghosts" => Some(UnitTypeId::Ghost),
        
        "hellion" | "hellions" => Some(UnitTypeId::Hellion),
        "hellbat" | "hellbats" => Some(UnitTypeId::HellionTank),
        "tank" | "tanks" | "siegetank" => Some(UnitTypeId::SiegeTank),
        "cyclone" | "cyclones" => Some(UnitTypeId::Cyclone),
        "thor" | "thors" => Some(UnitTypeId::Thor),
        "widowmine" | "mine" | "mines" => Some(UnitTypeId::WidowMine),
        
        "medivac" | "medivacs" => Some(UnitTypeId::Medivac),
        "viking" | "vikings" => Some(UnitTypeId::VikingFighter),
        "banshee" | "banshees" => Some(UnitTypeId::Banshee),
        "raven" | "ravens" => Some(UnitTypeId::Raven),
        "battlecruiser" | "bc" | "bcs" => Some(UnitTypeId::Battlecruiser),
        "liberator" | "liberators" | "lib" => Some(UnitTypeId::Liberator),
        
        "scv" | "scvs" => Some(UnitTypeId::SCV),
        "mule" | "mules" => Some(UnitTypeId::MULE),
        "cc" | "commandcenter" => Some(UnitTypeId::CommandCenter),
        "barracks" | "rax" => Some(UnitTypeId::Barracks),
        "factory" | "fact" => Some(UnitTypeId::Factory),
        "starport" | "port" => Some(UnitTypeId::Starport),
        "bunker" | "bunkers" => Some(UnitTypeId::Bunker),
        "turret" | "turrets" | "missileturret" => Some(UnitTypeId::MissileTurret),
        
        "zergling" | "ling" | "zerglings" | "lings" => Some(UnitTypeId::Zergling),
        "roach" | "roaches" => Some(UnitTypeId::Roach),
        "hydra" | "hydras" | "hydralisk" => Some(UnitTypeId::Hydralisk),
        "mutalisk" | "muta" | "mutas" => Some(UnitTypeId::Mutalisk),
        "ultralisk" | "ultra" | "ultras" => Some(UnitTypeId::Ultralisk),
        "baneling" | "banelings" | "bane" => Some(UnitTypeId::Baneling),
        
        "zealot" | "zealots" => Some(UnitTypeId::Zealot),
        "stalker" | "stalkers" => Some(UnitTypeId::Stalker),
        "immortal" | "immortals" => Some(UnitTypeId::Immortal),
        "colossus" | "colossi" => Some(UnitTypeId::Colossus),
        "voidray" | "void" | "voidrays" => Some(UnitTypeId::VoidRay),
        "carrier" | "carriers" => Some(UnitTypeId::Carrier),
        "tempest" => Some(UnitTypeId::Tempest),
        "observer" | "obs" => Some(UnitTypeId::Observer),
        
        _ => None,
    }
}
