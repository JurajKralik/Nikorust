use crate::units::helpers::targeting::*;
use crate::units::helpers::threat_detection::*;
use lazy_static::lazy_static;
use rust_sc2::prelude::*;
use std::collections::HashMap;

const WORKERS: &[UnitTypeId] = &[
    UnitTypeId::Drone,
    UnitTypeId::DroneBurrowed,
    UnitTypeId::SCV,
    UnitTypeId::Probe,
];

const HIGH_PRIORITY_EASY_TARGETS: &[UnitTypeId] = &[
    UnitTypeId::HighTemplar,
    UnitTypeId::DarkTemplar,
    UnitTypeId::WarpPrism,
    UnitTypeId::Observer,
    UnitTypeId::ObserverSiegeMode,
    UnitTypeId::Oracle,
    UnitTypeId::Sentry,
    UnitTypeId::Ghost,
    UnitTypeId::Infestor,
    UnitTypeId::InfestorBurrowed,
    UnitTypeId::Viper,
    UnitTypeId::Raven,
    UnitTypeId::Overseer,
    UnitTypeId::OverseerSiegeMode,
    UnitTypeId::SwarmHostMP,
    UnitTypeId::SwarmHostBurrowedMP,
    UnitTypeId::OverlordTransport,
    UnitTypeId::OverlordCocoon,
];

const LURKERS: &[UnitTypeId] = &[
    UnitTypeId::LurkerMPEgg,
    UnitTypeId::LurkerMP,
    UnitTypeId::LurkerMPBurrowed,
];

const STATIC_DEFENSE_AIR: &[UnitTypeId] = &[
    UnitTypeId::MissileTurret,
    UnitTypeId::Bunker,
    UnitTypeId::SporeCrawler,
    UnitTypeId::PhotonCannon,
    UnitTypeId::AutoTurret
];

const STATIC_DEFENSE_GROUND: &[UnitTypeId] = &[
    UnitTypeId::PlanetaryFortress,
    UnitTypeId::Bunker,
    UnitTypeId::SpineCrawler,
    UnitTypeId::PhotonCannon,
    UnitTypeId::SiegeTankSieged,
    UnitTypeId::AutoTurret
];

const HEAVY_UNITS: &[UnitTypeId] = &[
    UnitTypeId::Battlecruiser,
    UnitTypeId::Liberator,
    UnitTypeId::LiberatorAG,
    UnitTypeId::Thor,
    UnitTypeId::ThorAP,
    UnitTypeId::Carrier,
    UnitTypeId::Tempest,
    UnitTypeId::Mothership,
    UnitTypeId::Colossus,
    UnitTypeId::BroodLord,
    UnitTypeId::BroodLordCocoon,
    UnitTypeId::Ultralisk,
    UnitTypeId::UltraliskBurrowed,
];

pub const FLYING_UNITS: &[UnitTypeId] = &[
    UnitTypeId::VikingFighter,
    UnitTypeId::VikingAssault,
    UnitTypeId::Banshee,
    UnitTypeId::Raven,
    UnitTypeId::Liberator,
    UnitTypeId::LiberatorAG,
    UnitTypeId::Battlecruiser,
    UnitTypeId::Mutalisk,
    UnitTypeId::Corruptor,
    UnitTypeId::BroodLord,
    UnitTypeId::Phoenix,
    UnitTypeId::VoidRay,
    UnitTypeId::Carrier,
    UnitTypeId::Tempest,
    UnitTypeId::Oracle,
    UnitTypeId::Mothership,
];

lazy_static! {
    pub static ref TARGETING_PRIORITIES: TargetingPrioritiesList = {
        let mut list: HashMap<UnitTypeId, TargetingPriorities> = HashMap::new();

        // === BANSHEE
        add_to_targeting(&mut list, UnitTypeId::Banshee, PriorityLevel::High, &[
            UnitTypeId::Baneling,
            UnitTypeId::BanelingBurrowed,
            UnitTypeId::Adept,
            UnitTypeId::Reaper
        ]);
        add_to_targeting(&mut list, UnitTypeId::Banshee, PriorityLevel::High, LURKERS);
        add_to_targeting(&mut list, UnitTypeId::Banshee, PriorityLevel::VeryHigh, WORKERS);
        add_to_targeting(&mut list, UnitTypeId::Banshee, PriorityLevel::Max, HIGH_PRIORITY_EASY_TARGETS);

        // === BATTLECRUISER
        add_to_targeting(&mut list, UnitTypeId::Battlecruiser, PriorityLevel::High, WORKERS);
        add_to_targeting(&mut list, UnitTypeId::Battlecruiser, PriorityLevel::High, &[
            UnitTypeId::Marine,
            UnitTypeId::Reaper,
            UnitTypeId::Zergling,
            UnitTypeId::ZerglingBurrowed,
            UnitTypeId::Baneling,
            UnitTypeId::BanelingBurrowed,
            UnitTypeId::Zealot,
            UnitTypeId::Adept,
            UnitTypeId::Hellion,
            UnitTypeId::HellionTank,
        ]);
        add_to_targeting(&mut list, UnitTypeId::Battlecruiser, PriorityLevel::High, STATIC_DEFENSE_GROUND);
        add_to_targeting(&mut list, UnitTypeId::Battlecruiser, PriorityLevel::VeryHigh, STATIC_DEFENSE_AIR);
        add_to_targeting(&mut list, UnitTypeId::Cyclone, PriorityLevel::Max, HIGH_PRIORITY_EASY_TARGETS);

        // === CYCLONE
        add_to_targeting(&mut list, UnitTypeId::Cyclone, PriorityLevel::High, HIGH_PRIORITY_EASY_TARGETS);
        add_to_targeting(&mut list, UnitTypeId::Cyclone, PriorityLevel::VeryHigh, STATIC_DEFENSE_GROUND);
        add_to_targeting(&mut list, UnitTypeId::Cyclone, PriorityLevel::VeryHigh, HEAVY_UNITS);

        // === GHOST
        add_to_targeting(&mut list, UnitTypeId::Ghost, PriorityLevel::High, WORKERS);
        add_to_targeting(&mut list, UnitTypeId::Ghost, PriorityLevel::High, &[
            UnitTypeId::Marine,
            UnitTypeId::Marauder,
            UnitTypeId::HellionTank,
            UnitTypeId::Zergling,
            UnitTypeId::ZerglingBurrowed,

        ]);
        add_to_targeting(&mut list, UnitTypeId::Ghost, PriorityLevel::VeryHigh, &[
            UnitTypeId::Reaper,
            UnitTypeId::Adept,
            UnitTypeId::Roach,
            UnitTypeId::RoachBurrowed,
            UnitTypeId::Ravager,
            UnitTypeId::RavagerBurrowed,
            UnitTypeId::Hydralisk,
            UnitTypeId::HydraliskBurrowed,
        ]);
        add_to_targeting(&mut list, UnitTypeId::Ghost, PriorityLevel::VeryHigh, LURKERS);
        add_to_targeting(&mut list, UnitTypeId::Ghost, PriorityLevel::Max, &[
            UnitTypeId::Baneling,
            UnitTypeId::BanelingBurrowed,
            UnitTypeId::HighTemplar,
            UnitTypeId::DarkTemplar,
            UnitTypeId::Ghost,
            UnitTypeId::Infestor,
            UnitTypeId::InfestorBurrowed,
            UnitTypeId::Viper,
            UnitTypeId::Overseer,
            UnitTypeId::OverseerSiegeMode,
            UnitTypeId::SwarmHostMP,
            UnitTypeId::SwarmHostBurrowedMP,
            UnitTypeId::OverlordTransport,
            UnitTypeId::OverlordCocoon,
        ]);

        // === HELLION
        add_to_targeting(&mut list, UnitTypeId::Hellion, PriorityLevel::High, &[
            UnitTypeId::Marine,
            UnitTypeId::Reaper,
            UnitTypeId::Zergling,
            UnitTypeId::Baneling,
            UnitTypeId::Hydralisk,
            UnitTypeId::Zealot,
            UnitTypeId::Adept,
            UnitTypeId::Hellion,
            UnitTypeId::HellionTank,
            UnitTypeId::Queen
        ]);
        add_to_targeting(&mut list, UnitTypeId::Hellion, PriorityLevel::VeryHigh, HIGH_PRIORITY_EASY_TARGETS);
        add_to_targeting(&mut list, UnitTypeId::Hellion, PriorityLevel::Max, WORKERS);

        // === HELLIONTANK
        add_to_targeting(&mut list, UnitTypeId::HellionTank, PriorityLevel::High, &[
            UnitTypeId::Marine,
            UnitTypeId::Reaper,
            UnitTypeId::Zergling,
            UnitTypeId::Baneling,
            UnitTypeId::Hydralisk,
            UnitTypeId::Zealot,
            UnitTypeId::Adept,
            UnitTypeId::Hellion,
            UnitTypeId::HellionTank,
            UnitTypeId::Queen
        ]);
        add_to_targeting(&mut list, UnitTypeId::HellionTank, PriorityLevel::VeryHigh, HIGH_PRIORITY_EASY_TARGETS);
        add_to_targeting(&mut list, UnitTypeId::HellionTank, PriorityLevel::Max, WORKERS);

        // === MARAUDER
        add_to_targeting(&mut list, UnitTypeId::Marauder, PriorityLevel::High, STATIC_DEFENSE_GROUND);
        add_to_targeting(&mut list, UnitTypeId::Marauder, PriorityLevel::VeryHigh, HIGH_PRIORITY_EASY_TARGETS);
        add_to_targeting(&mut list, UnitTypeId::Marauder, PriorityLevel::Max, HEAVY_UNITS);
        add_to_targeting(&mut list, UnitTypeId::Marauder, PriorityLevel::Max, LURKERS);

        // === MARINE
        add_to_targeting(&mut list, UnitTypeId::Marine, PriorityLevel::High, HIGH_PRIORITY_EASY_TARGETS);
        add_to_targeting(&mut list, UnitTypeId::Marine, PriorityLevel::VeryHigh, STATIC_DEFENSE_GROUND);

        // === REAPER
        add_to_targeting(&mut list, UnitTypeId::Reaper, PriorityLevel::High, WORKERS);

        // === SIEGETANK
        add_to_targeting(&mut list, UnitTypeId::SiegeTank, PriorityLevel::High, WORKERS);
        add_to_targeting(&mut list, UnitTypeId::SiegeTank, PriorityLevel::VeryHigh, STATIC_DEFENSE_GROUND);
        add_to_targeting(&mut list, UnitTypeId::SiegeTank, PriorityLevel::Max, &[
            UnitTypeId::SiegeTank,
            UnitTypeId::SiegeTankSieged,
            UnitTypeId::Roach,
            UnitTypeId::RoachBurrowed,
            UnitTypeId::Ravager,
            UnitTypeId::RavagerBurrowed,
            UnitTypeId::LurkerMPEgg,
            UnitTypeId::LurkerMP,
            UnitTypeId::LurkerMPBurrowed,
            UnitTypeId::Stalker,
            UnitTypeId::Colossus
        ]);
        add_to_targeting(&mut list, UnitTypeId::SiegeTankSieged, PriorityLevel::Max, HIGH_PRIORITY_EASY_TARGETS);

        // === SIEGETANKSIEGED
        add_to_targeting(&mut list, UnitTypeId::SiegeTankSieged, PriorityLevel::High, WORKERS);
        add_to_targeting(&mut list, UnitTypeId::SiegeTankSieged, PriorityLevel::VeryHigh, &[
            UnitTypeId::Marine,
            UnitTypeId::Reaper,
            UnitTypeId::Zergling,
            UnitTypeId::ZerglingBurrowed,
            UnitTypeId::Baneling,
            UnitTypeId::BanelingBurrowed,
            UnitTypeId::Zealot
        ]);
        add_to_targeting(&mut list, UnitTypeId::SiegeTankSieged, PriorityLevel::Max, &[
            UnitTypeId::SiegeTank,
            UnitTypeId::SiegeTankSieged,
            UnitTypeId::Roach,
            UnitTypeId::RoachBurrowed,
            UnitTypeId::Ravager,
            UnitTypeId::RavagerBurrowed,
            UnitTypeId::LurkerMPEgg,
            UnitTypeId::LurkerMP,
            UnitTypeId::LurkerMPBurrowed,
            UnitTypeId::Stalker,
            UnitTypeId::Colossus
        ]);
        add_to_targeting(&mut list, UnitTypeId::SiegeTankSieged, PriorityLevel::Max, HIGH_PRIORITY_EASY_TARGETS);

        // === VIKINGFIGHTER (AIR)
        add_to_targeting(&mut list, UnitTypeId::VikingFighter, PriorityLevel::High, &[
            UnitTypeId::OverlordCocoon,
            UnitTypeId::Overseer,
            UnitTypeId::OverseerSiegeMode,
            UnitTypeId::Observer,
            UnitTypeId::ObserverSiegeMode,
        ]);
        add_to_targeting(&mut list, UnitTypeId::VikingFighter, PriorityLevel::VeryHigh, &[
            UnitTypeId::Medivac,
            UnitTypeId::Banshee,
            UnitTypeId::Raven,
            UnitTypeId::Liberator,
            UnitTypeId::LiberatorAG,
            UnitTypeId::OverlordTransport,
            UnitTypeId::Oracle,
            UnitTypeId::WarpPrism,
            UnitTypeId::WarpPrismPhasing,
            UnitTypeId::BroodLord,
            UnitTypeId::BroodLordCocoon,
        ]);
        add_to_targeting(&mut list, UnitTypeId::VikingFighter, PriorityLevel::Max, &[
            UnitTypeId::VikingFighter,
            UnitTypeId::Battlecruiser,
            UnitTypeId::Phoenix,
            UnitTypeId::VoidRay,
            UnitTypeId::Tempest,
            UnitTypeId::Carrier,
            UnitTypeId::Mothership,
            UnitTypeId::Corruptor,
            UnitTypeId::Mutalisk,
            UnitTypeId::Viper,
        ]);

        // === VIKINGASSAULT (GROUND)
        add_to_targeting(&mut list, UnitTypeId::VikingAssault, PriorityLevel::High, WORKERS);

        TargetingPrioritiesList { list }
    };
    pub static ref THREAT_LEVELS: ThreatLevelsList = {
        let mut list: HashMap<UnitTypeId, ThreatLevels> = HashMap::new();

        // === BANSHEE
        add_threats(&mut list, UnitTypeId::Banshee, ThreatLevel::Flee, &[
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
        add_threats(&mut list, UnitTypeId::Battlecruiser, ThreatLevel::Countered, &[
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
        add_threats(&mut list, UnitTypeId::Ghost, ThreatLevel::Countered, &[
            UnitTypeId::Zergling,
            UnitTypeId::Zealot,
        ]);

        // === HELLION
        add_threats(&mut list, UnitTypeId::Hellion, ThreatLevel::Countered, &[
            UnitTypeId::PhotonCannon,
        ]);

        // === HELLIONTANK
        add_threats(&mut list, UnitTypeId::HellionTank, ThreatLevel::Countered, &[
            UnitTypeId::PhotonCannon,
        ]);

        // === MARAUDER
        add_threats(&mut list, UnitTypeId::Marauder, ThreatLevel::Countered, &[
            UnitTypeId::Baneling,
            UnitTypeId::Archon,
            UnitTypeId::Ultralisk,
        ]);

        // === MARINE
        add_threats(&mut list, UnitTypeId::Marine, ThreatLevel::Countered, &[
            UnitTypeId::Baneling,
            UnitTypeId::Archon,
            UnitTypeId::Ultralisk,
        ]);

        // === REAPER
        add_threats(&mut list, UnitTypeId::Reaper, ThreatLevel::Countered, &[
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
    pub static ref UNITS_PRIORITY_IGNORE: Vec<UnitTypeId> = vec![
        UnitTypeId::AdeptPhaseShift,
        UnitTypeId::Egg,
        UnitTypeId::Larva,
        UnitTypeId::Changeling,
        UnitTypeId::ChangelingMarine,
        UnitTypeId::ChangelingZealot,
        UnitTypeId::ChangelingZergling,
        UnitTypeId::MULE
    ];
    pub static ref UNITS_PRIORITY_LOW: Vec<UnitTypeId> = vec![
        UnitTypeId::Changeling,
        UnitTypeId::ChangelingMarine,
        UnitTypeId::ChangelingZealot,
        UnitTypeId::ChangelingZergling,
        UnitTypeId::LocustMP,
        UnitTypeId::LocustMPFlying,
        UnitTypeId::Overlord
    ];
}

fn add_to_targeting(
    list: &mut HashMap<UnitTypeId, TargetingPriorities>,
    unit: UnitTypeId,
    level: PriorityLevel,
    targets: &[UnitTypeId],
) {
    list.entry(unit)
        .or_insert_with(|| TargetingPriorities {
            priorities: Vec::new(),
        })
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
        .or_insert_with(|| ThreatLevels {
            threats: Vec::new(),
        })
        .threats
        .extend(threats.iter().copied().map(|t| ThreatLevelInfo {
            unit_type: t,
            threat_level: level,
        }));
}

// Upgrades
lazy_static! {
    pub static ref ENGINEERING_BAY_UPGRADE_ORDER: Vec<AbilityId> = vec![
        AbilityId::EngineeringBayResearchTerranInfantryWeaponsLevel1,
        AbilityId::EngineeringBayResearchTerranInfantryArmorLevel1,
        AbilityId::EngineeringBayResearchTerranInfantryWeaponsLevel2,
        AbilityId::EngineeringBayResearchTerranInfantryArmorLevel2,
        AbilityId::EngineeringBayResearchTerranInfantryWeaponsLevel3,
        AbilityId::EngineeringBayResearchTerranInfantryArmorLevel3,
    ];
    pub static ref ARMORY_GROUND_UPGRADE_ORDER: Vec<AbilityId> = vec![
        AbilityId::ArmoryResearchTerranVehicleWeaponsLevel1,
        AbilityId::ArmoryResearchTerranVehicleAndShipPlatingLevel1,
        AbilityId::ArmoryResearchTerranVehicleWeaponsLevel2,
        AbilityId::ArmoryResearchTerranVehicleAndShipPlatingLevel2,
        AbilityId::ArmoryResearchTerranVehicleWeaponsLevel3,
        AbilityId::ArmoryResearchTerranVehicleAndShipPlatingLevel3,
    ];
    pub static ref ARMORY_AIR_UPGRADE_ORDER: Vec<AbilityId> = vec![
        AbilityId::ArmoryResearchTerranShipWeaponsLevel1,
        AbilityId::ArmoryResearchTerranVehicleAndShipPlatingLevel1,
        AbilityId::ArmoryResearchTerranShipWeaponsLevel2,
        AbilityId::ArmoryResearchTerranVehicleAndShipPlatingLevel2,
        AbilityId::ArmoryResearchTerranShipWeaponsLevel3,
        AbilityId::ArmoryResearchTerranVehicleAndShipPlatingLevel3,
    ];
}