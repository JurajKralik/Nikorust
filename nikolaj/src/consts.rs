use crate::units::helpers::targeting::*;
use crate::units::helpers::threat_detection::*;
use lazy_static::lazy_static;
use rust_sc2::prelude::*;
use std::collections::HashMap;


pub const WORKERS: &[UnitTypeId] = &[
    UnitTypeId::Drone,
    UnitTypeId::DroneBurrowed,
    UnitTypeId::SCV,
    UnitTypeId::Probe,
];

lazy_static! {
    pub static ref TARGETING_PRIORITIES: TargetingPrioritiesList = {
        let mut list: HashMap<UnitTypeId, TargetingPriorities> = HashMap::new();

        // === BANSHEE
        add_to_targeting(&mut list, UnitTypeId::Banshee, PriorityLevel::High, WORKERS);

        // === BATTLECRUISER
        add_to_targeting(&mut list, UnitTypeId::Battlecruiser, PriorityLevel::High,
            &[UnitTypeId::Probe, UnitTypeId::Drone, UnitTypeId::SCV]);
        add_to_targeting(&mut list, UnitTypeId::Battlecruiser, PriorityLevel::VeryHigh,
            &[UnitTypeId::Marine, UnitTypeId::Reaper, UnitTypeId::Zergling, UnitTypeId::Baneling,
              UnitTypeId::Zealot, UnitTypeId::Adept, UnitTypeId::Hellion, UnitTypeId::HellionTank]);

        // === CYCLONE
        add_to_targeting(&mut list, UnitTypeId::Cyclone, PriorityLevel::High,
            &[UnitTypeId::WarpPrism, UnitTypeId::Zealot, UnitTypeId::Ghost, UnitTypeId::Viper,
              UnitTypeId::Bunker, UnitTypeId::PhotonCannon, UnitTypeId::SpineCrawler, UnitTypeId::LurkerMPEgg, UnitTypeId::LurkerMP, UnitTypeId::LurkerMPBurrowed]);
        add_to_targeting(&mut list, UnitTypeId::Cyclone, PriorityLevel::VeryHigh,
            &[UnitTypeId::Infestor, UnitTypeId::SiegeTank, UnitTypeId::SiegeTankSieged,
              UnitTypeId::HighTemplar, UnitTypeId::DarkTemplar, UnitTypeId::WarpPrismPhasing]);

        // === GHOST
        add_to_targeting(&mut list, UnitTypeId::Ghost, PriorityLevel::High,
            &[UnitTypeId::Zergling, UnitTypeId::OverlordTransport, UnitTypeId::OverlordCocoon]);
        add_to_targeting(&mut list, UnitTypeId::Ghost, PriorityLevel::VeryHigh,
            &[UnitTypeId::OverseerSiegeMode, UnitTypeId::Overseer, UnitTypeId::Viper, UnitTypeId::Corruptor,
              UnitTypeId::BroodLord, UnitTypeId::BroodLordCocoon, UnitTypeId::Baneling,
              UnitTypeId::SwarmHostMP, UnitTypeId::SwarmHostBurrowedMP, UnitTypeId::Mutalisk]);
        add_to_targeting(&mut list, UnitTypeId::Ghost, PriorityLevel::VeryHigh,
            &[UnitTypeId::Ultralisk, UnitTypeId::Infestor, UnitTypeId::LurkerMPEgg, UnitTypeId::LurkerMP, UnitTypeId::LurkerMPBurrowed,
              UnitTypeId::SporeCrawler, UnitTypeId::Queen, UnitTypeId::SpineCrawler]);

        // === HELLION
        add_to_targeting(&mut list, UnitTypeId::Hellion, PriorityLevel::High,
            &[UnitTypeId::Probe, UnitTypeId::Drone, UnitTypeId::SCV]);
        add_to_targeting(&mut list, UnitTypeId::Hellion, PriorityLevel::VeryHigh,
            &[UnitTypeId::Marine, UnitTypeId::Reaper, UnitTypeId::Zergling, UnitTypeId::Baneling,
              UnitTypeId::Hydralisk, UnitTypeId::Zealot, UnitTypeId::Adept, UnitTypeId::Hellion,
              UnitTypeId::HellionTank, UnitTypeId::Queen]);

        // === HELLIONTANK
        add_to_targeting(&mut list, UnitTypeId::HellionTank, PriorityLevel::High,
            &[UnitTypeId::Probe, UnitTypeId::Drone, UnitTypeId::SCV]);
        add_to_targeting(&mut list, UnitTypeId::HellionTank, PriorityLevel::VeryHigh,
            &[UnitTypeId::Marine, UnitTypeId::Reaper, UnitTypeId::Zergling, UnitTypeId::Baneling,
              UnitTypeId::Hydralisk, UnitTypeId::Zealot, UnitTypeId::Adept, UnitTypeId::Hellion,
              UnitTypeId::HellionTank, UnitTypeId::Queen]);

        // === MARAUDER
        add_to_targeting(&mut list, UnitTypeId::Marauder, PriorityLevel::High,
            &[UnitTypeId::Zealot, UnitTypeId::Adept, UnitTypeId::Ghost, UnitTypeId::Viper,
              UnitTypeId::Queen, UnitTypeId::Bunker, UnitTypeId::PhotonCannon,
              UnitTypeId::SpineCrawler, UnitTypeId::LurkerMPEgg, UnitTypeId::LurkerMP, UnitTypeId::LurkerMPBurrowed]);
        add_to_targeting(&mut list, UnitTypeId::Marauder, PriorityLevel::VeryHigh,
            &[UnitTypeId::Baneling, UnitTypeId::Infestor, UnitTypeId::SiegeTank, UnitTypeId::SiegeTankSieged,
              UnitTypeId::HighTemplar, UnitTypeId::DarkTemplar]);

        // === MARINE
        add_to_targeting(&mut list, UnitTypeId::Marine, PriorityLevel::High,
            &[UnitTypeId::Zealot, UnitTypeId::Adept, UnitTypeId::Ghost, UnitTypeId::Viper,
              UnitTypeId::Queen, UnitTypeId::Bunker, UnitTypeId::PhotonCannon,
              UnitTypeId::SpineCrawler, UnitTypeId::LurkerMPEgg, UnitTypeId::LurkerMP, UnitTypeId::LurkerMPBurrowed]);
        add_to_targeting(&mut list, UnitTypeId::Marine, PriorityLevel::VeryHigh,
            &[UnitTypeId::Baneling, UnitTypeId::Infestor, UnitTypeId::SiegeTank, UnitTypeId::SiegeTankSieged,
              UnitTypeId::HighTemplar, UnitTypeId::DarkTemplar]);

        // === REAPER
        add_to_targeting(&mut list, UnitTypeId::Reaper, PriorityLevel::High, WORKERS);
        add_to_targeting(&mut list, UnitTypeId::Reaper, PriorityLevel::VeryHigh,
            &[UnitTypeId::Marine, UnitTypeId::Reaper, UnitTypeId::Zergling, UnitTypeId::Baneling, UnitTypeId::Zealot]);

        // === SIEGETANK
        add_to_targeting(&mut list, UnitTypeId::SiegeTank, PriorityLevel::High, WORKERS);
        add_to_targeting(&mut list, UnitTypeId::SiegeTank, PriorityLevel::VeryHigh,
            &[UnitTypeId::Marine, UnitTypeId::Reaper, UnitTypeId::Zergling, UnitTypeId::Baneling, UnitTypeId::Zealot]);
        add_to_targeting(&mut list, UnitTypeId::SiegeTank, PriorityLevel::VeryHigh,
            &[UnitTypeId::SiegeTank, UnitTypeId::SiegeTankSieged, UnitTypeId::Roach, UnitTypeId::Ravager,
               UnitTypeId::LurkerMPEgg, UnitTypeId::LurkerMP, UnitTypeId::LurkerMPBurrowed, UnitTypeId::Stalker]);

        // === SIEGETANKSIEGED
        add_to_targeting(&mut list, UnitTypeId::SiegeTankSieged, PriorityLevel::High, WORKERS);
        add_to_targeting(&mut list, UnitTypeId::SiegeTankSieged, PriorityLevel::VeryHigh,
            &[UnitTypeId::Marine, UnitTypeId::Reaper, UnitTypeId::Zergling, UnitTypeId::Baneling, UnitTypeId::Zealot]);
        add_to_targeting(&mut list, UnitTypeId::SiegeTankSieged, PriorityLevel::VeryHigh,
            &[UnitTypeId::SiegeTank, UnitTypeId::SiegeTankSieged, UnitTypeId::Roach, UnitTypeId::Ravager,
               UnitTypeId::LurkerMPEgg, UnitTypeId::LurkerMP, UnitTypeId::LurkerMPBurrowed, UnitTypeId::Stalker]);

        // === VIKINGFIGHTER
        add_to_targeting(&mut list, UnitTypeId::VikingFighter, PriorityLevel::High,
            &[UnitTypeId::Medivac, UnitTypeId::Oracle, UnitTypeId::WarpPrism, UnitTypeId::WarpPrismPhasing,
              UnitTypeId::Liberator, UnitTypeId::LiberatorAG, UnitTypeId::BroodLord]);
        add_to_targeting(&mut list, UnitTypeId::VikingFighter, PriorityLevel::VeryHigh,
            &[UnitTypeId::VikingFighter, UnitTypeId::Raven, UnitTypeId::Battlecruiser, UnitTypeId::Phoenix,
              UnitTypeId::VoidRay, UnitTypeId::Tempest, UnitTypeId::Carrier, UnitTypeId::Mothership,
              UnitTypeId::Corruptor, UnitTypeId::Mutalisk, UnitTypeId::Viper]);

        // === VIKINGASSAULT
        add_to_targeting(&mut list, UnitTypeId::VikingAssault, PriorityLevel::High,
            &[UnitTypeId::Medivac, UnitTypeId::Oracle, UnitTypeId::WarpPrism, UnitTypeId::WarpPrismPhasing,
              UnitTypeId::Liberator, UnitTypeId::LiberatorAG, UnitTypeId::BroodLord]);
        add_to_targeting(&mut list, UnitTypeId::VikingAssault, PriorityLevel::VeryHigh,
            &[UnitTypeId::VikingFighter, UnitTypeId::Raven, UnitTypeId::Battlecruiser, UnitTypeId::Phoenix,
              UnitTypeId::VoidRay, UnitTypeId::Tempest, UnitTypeId::Carrier, UnitTypeId::Mothership,
              UnitTypeId::Corruptor, UnitTypeId::Mutalisk, UnitTypeId::Viper]);

        TargetingPrioritiesList { list }
    };
pub static ref THREAT_LEVELS: ThreatLevelsList = {
        let mut list: HashMap<UnitTypeId, ThreatLevels> = HashMap::new();

        // === BANSHEE
        add_threats(&mut list, UnitTypeId::Banshee, ThreatLevel::Danger, &[
            UnitTypeId::MissileTurret,
            UnitTypeId::SporeCrawler,
            UnitTypeId::PhotonCannon,
            UnitTypeId::Thor,
            UnitTypeId::ThorAALance,
            UnitTypeId::ThorAAWeapon,
            UnitTypeId::Battlecruiser,
            UnitTypeId::Tempest,
            UnitTypeId::Carrier,
            UnitTypeId::Corruptor,
            UnitTypeId::Infestor,
            UnitTypeId::InfestorBurrowed,
            UnitTypeId::Viper,
        ]);

        // === BATTLECRUISER
        add_threats(&mut list, UnitTypeId::Battlecruiser, ThreatLevel::Danger, &[
            UnitTypeId::MissileTurret,
            UnitTypeId::SporeCrawler,
            UnitTypeId::PhotonCannon,
            UnitTypeId::Thor,
            UnitTypeId::ThorAALance,
            UnitTypeId::ThorAAWeapon,
            UnitTypeId::Battlecruiser,
            UnitTypeId::Tempest,
            UnitTypeId::Carrier,
            UnitTypeId::Corruptor,
            UnitTypeId::Infestor,
            UnitTypeId::InfestorBurrowed,
            UnitTypeId::Viper,
        ]);

        // === GHOST
        add_threats(&mut list, UnitTypeId::Ghost, ThreatLevel::Danger, &[
            UnitTypeId::Zergling,
            UnitTypeId::Zealot,
        ]);

        // === HELLION
        add_threats(&mut list, UnitTypeId::Hellion, ThreatLevel::Danger, &[
            UnitTypeId::PhotonCannon,
        ]);

        // === HELLIONTANK
        add_threats(&mut list, UnitTypeId::HellionTank, ThreatLevel::Danger, &[
            UnitTypeId::PhotonCannon,
        ]);

        // === MARAUDER
        add_threats(&mut list, UnitTypeId::Marauder, ThreatLevel::Danger, &[
            UnitTypeId::Baneling,
            UnitTypeId::Archon,
            UnitTypeId::Ultralisk,
        ]);

        // === MARINE
        add_threats(&mut list, UnitTypeId::Marine, ThreatLevel::Danger, &[
            UnitTypeId::Baneling,
            UnitTypeId::Archon,
            UnitTypeId::Ultralisk,
        ]);

        // === REAPER
        add_threats(&mut list, UnitTypeId::Reaper, ThreatLevel::Danger, &[
            UnitTypeId::Adept,
            UnitTypeId::Stalker,
            UnitTypeId::Queen,
            UnitTypeId::Roach,
            UnitTypeId::Hellion,
            UnitTypeId::Cyclone,
            UnitTypeId::Marauder,
            UnitTypeId::SpineCrawler,
            UnitTypeId::Bunker,
            UnitTypeId::PhotonCannon,
        ]);

        ThreatLevelsList { list }
    };
    pub static ref IGNORE_UNITS: Vec<UnitTypeId> = vec![
        UnitTypeId::AdeptPhaseShift,
        UnitTypeId::Egg,
        UnitTypeId::Larva
    ];
}

fn add_to_targeting(
    list: &mut HashMap<UnitTypeId, TargetingPriorities>,
    unit: UnitTypeId,
    level: PriorityLevel,
    targets: &[UnitTypeId],
) {
    list.entry(unit)
        .or_insert_with(|| TargetingPriorities { priorities: Vec::new() })
        .priorities
        .extend(targets.iter().copied().map(|t| TargetPriorityInfo {
            unit_type: t,
            priority_level: level,
        }));
}

fn add_threats(
    list: &mut HashMap<UnitTypeId, ThreatLevels>,
    unit: UnitTypeId,
    level: ThreatLevel,
    threats: &[UnitTypeId],
) {
    list.entry(unit)
        .or_insert_with(|| ThreatLevels { threats: Vec::new() })
        .threats
        .extend(threats.iter().copied().map(|t| ThreatLevelInfo {
            unit_type: t,
            threat_level: level,
        }));
}