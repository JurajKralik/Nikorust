use rust_sc2::prelude::*;
//use lazy_static::lazy_static;
//use std::collections::HashMap;


pub const BIO: &'static [UnitTypeId] = &[
    UnitTypeId::Marine,
    UnitTypeId::Marauder,
    UnitTypeId::Reaper,
    UnitTypeId::Ghost,
];
pub const PRIORITY_ZERO: &'static [UnitTypeId] = &[
    UnitTypeId::Larva,
    UnitTypeId::Egg,
    UnitTypeId::AdeptPhaseShift,
    UnitTypeId::Interceptor,
    UnitTypeId::Overlord,
    UnitTypeId::OverlordCocoon,
    UnitTypeId::Overseer,
    UnitTypeId::OverseerSiegeMode,
    UnitTypeId::OverlordTransport,
    UnitTypeId::Observer,
    UnitTypeId::ObserverSiegeMode,
    UnitTypeId::Medivac,
    UnitTypeId::Phoenix,
];
pub const PRODUCTION: &'static [UnitTypeId] = &[
    UnitTypeId::Barracks,
    UnitTypeId::Factory,
    UnitTypeId::Starport
];
pub const MECH: &'static [UnitTypeId] = &[
    UnitTypeId::Hellion,
    UnitTypeId::HellionTank,
    UnitTypeId::SiegeTank,
    UnitTypeId::SiegeTankSieged,
    UnitTypeId::WidowMine,
    UnitTypeId::WidowMineBurrowed,
    UnitTypeId::Cyclone,
    UnitTypeId::ThorAP,
    UnitTypeId::Thor,
    UnitTypeId::VikingAssault,
    UnitTypeId::VikingFighter,
    UnitTypeId::Raven,
    UnitTypeId::Banshee,
    UnitTypeId::Battlecruiser,
];
pub const EXCLUDE_MAIN_ARMY: &'static [UnitTypeId] = &[
    UnitTypeId::VikingAssault,
    UnitTypeId::VikingFighter,
    UnitTypeId::Raven,
    UnitTypeId::Banshee,
    UnitTypeId::Reaper,
    UnitTypeId::Medivac,
    UnitTypeId::SCV,
    UnitTypeId::MULE,
    UnitTypeId::WidowMine,
    UnitTypeId::WidowMineBurrowed,
];
pub const MEMORY_IGNORETYPES: &'static [UnitTypeId] = &[
    UnitTypeId::SCV,
    UnitTypeId::Drone,
    UnitTypeId::DroneBurrowed,
    UnitTypeId::Probe,
    UnitTypeId::AdeptPhaseShift,
    UnitTypeId::Observer,
    UnitTypeId::Overlord,
    UnitTypeId::OverlordTransport,
    UnitTypeId::Overseer,
    UnitTypeId::OverlordCocoon,
    UnitTypeId::Larva,
    UnitTypeId::Egg,
    UnitTypeId::BroodLordCocoon,
    UnitTypeId::BanelingCocoon,
    UnitTypeId::LurkerMPEgg,
    UnitTypeId::Changeling,
    UnitTypeId::Broodling,
    UnitTypeId::LocustMP,
    UnitTypeId::LocustMPFlying,
    UnitTypeId::MULE,
    UnitTypeId::ChangelingMarine,
    UnitTypeId::ChangelingMarineShield,
];
pub const CLOAK_AND_BURROW: &'static [UnitTypeId] = &[
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
pub const CLOAK_STRUCTURES: &'static [UnitTypeId] = &[
    UnitTypeId::StarportTechLab,
    UnitTypeId::GhostAcademy,
    UnitTypeId::LurkerDenMP,
    UnitTypeId::DarkShrine,
];
pub const FLIERS_IGNORE: &'static [UnitTypeId] = &[
    UnitTypeId::Overlord,
    UnitTypeId::OverlordCocoon,
    UnitTypeId::Overseer,
    UnitTypeId::OverseerSiegeMode,
    UnitTypeId::OverlordTransport,
    UnitTypeId::Observer,
    UnitTypeId::ObserverSiegeMode,
    UnitTypeId::Medivac,
    UnitTypeId::Phoenix,
];
pub const HEAVY_FLIERS: &'static [UnitTypeId] = &[
    UnitTypeId::Tempest,
    UnitTypeId::Carrier,
    UnitTypeId::Mothership,
    UnitTypeId::Battlecruiser,
    UnitTypeId::BroodLord,
    UnitTypeId::Broodling,
];
pub const FLYING_PRODUCTION_STRUCTURES: &'static [UnitTypeId] = &[
    UnitTypeId::StarportTechLab,
    UnitTypeId::Starport,
    UnitTypeId::StarportReactor,
    UnitTypeId::Spire,
    UnitTypeId::GreaterSpire,
    UnitTypeId::Stargate,
    UnitTypeId::FleetBeacon,
];
pub const HEAVY_FLYING_PRODUCTION_STRUCTURES: &'static [UnitTypeId] = &[
    UnitTypeId::FusionCore,
    UnitTypeId::FleetBeacon,
    UnitTypeId::GreaterSpire,
];
pub const DEFENSIVE_IGNORETYPES: &'static [UnitTypeId] = &[
    UnitTypeId::SCV,
    UnitTypeId::Drone,
    UnitTypeId::DroneBurrowed,
    UnitTypeId::Probe,
    UnitTypeId::Observer,
    UnitTypeId::Overlord,
    UnitTypeId::Overseer,
    UnitTypeId::Larva,
    UnitTypeId::Changeling,
    UnitTypeId::MULE,
    UnitTypeId::ChangelingMarine,
    UnitTypeId::ChangelingMarineShield,
];
/*
lazy_static! {
    pub static ref SPEED_MINING: bool = true;
    pub static ref SPEED_MINING_WORKERS: u32 = 74;
    pub static ref DISABLE_PRODUCTION: bool = false;
    pub static ref THREATENING_REFERENCE: f32 = 1.2f32;
    pub static ref ADVANCED_REFERENCE: f32 = 0.9f32;
    pub static ref OFFENSIVE_REFERENCE: f32 = 0.8f32;
    pub static ref UNLOCK_ROACH_WORKERS: HashMap<Race, usize> = {
        let mut m = HashMap::new();
        m.insert(Race::Zerg, 25);
        m.insert(Race::Protoss, 42);
        m.insert(Race::Terran, 36);
        m.insert(Race::Random, 28);
        m
    };
}*/
