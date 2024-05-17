use lazy_static::lazy_static;
use rust_sc2::prelude::*;
use std::collections::HashMap;

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
    UnitTypeId::Starport,
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
pub const UNITS_NEED_TECHLAB: &'static [UnitTypeId] = &[
    UnitTypeId::Marauder,
    UnitTypeId::Ghost,
    UnitTypeId::SiegeTank,
    UnitTypeId::Thor,
    UnitTypeId::Banshee,
    UnitTypeId::Battlecruiser,
];

lazy_static! {
    pub static ref UNIT_SOURCE: HashMap<UnitTypeId, UnitTypeId> = {
        let mut m = HashMap::new();
        m.insert(UnitTypeId::Marine, UnitTypeId::Barracks);
        m.insert(UnitTypeId::Marauder, UnitTypeId::Barracks);
        m.insert(UnitTypeId::Reaper, UnitTypeId::Barracks);
        m.insert(UnitTypeId::Ghost, UnitTypeId::Barracks);
        m.insert(UnitTypeId::Hellion, UnitTypeId::Factory);
        m.insert(UnitTypeId::WidowMine, UnitTypeId::Factory);
        m.insert(UnitTypeId::Cyclone, UnitTypeId::Factory);
        m.insert(UnitTypeId::SiegeTank, UnitTypeId::Factory);
        m.insert(UnitTypeId::Thor, UnitTypeId::Factory);
        m.insert(UnitTypeId::VikingFighter, UnitTypeId::Starport);
        m.insert(UnitTypeId::Banshee, UnitTypeId::Starport);
        m.insert(UnitTypeId::Liberator, UnitTypeId::Starport);
        m.insert(UnitTypeId::Raven, UnitTypeId::Starport);
        m.insert(UnitTypeId::Medivac, UnitTypeId::Starport);
        m.insert(UnitTypeId::Battlecruiser, UnitTypeId::Starport);
        m
    };
    pub static ref TECHLABS: HashMap<UnitTypeId, UnitTypeId> = {
        let mut m = HashMap::new();
        m.insert(UnitTypeId::Barracks, UnitTypeId::BarracksTechLab);
        m.insert(UnitTypeId::Factory, UnitTypeId::FactoryTechLab);
        m.insert(UnitTypeId::Starport, UnitTypeId::StarportTechLab);
        m
    };
    pub static ref TECHLABS_ABILITY: HashMap<UnitTypeId, AbilityId> = {
        let mut m = HashMap::new();
        m.insert(UnitTypeId::BarracksTechLab, AbilityId::BarracksTechLabMorphTechLabBarracks);
        m.insert(UnitTypeId::FactoryTechLab, AbilityId::FactoryTechReactorMorphTechLabFactory);
        m.insert(UnitTypeId::StarportTechLab, AbilityId::StarportTechReactorMorphTechLabStarport);
        m
    };
    
    pub static ref TECH_REQUIREMENT: HashMap<UnitTypeId, UnitTypeId> = {
        let mut m = HashMap::new();
        //Units
        m.insert(UnitTypeId::Marine, UnitTypeId::Barracks);
        m.insert(UnitTypeId::Marauder, UnitTypeId::Barracks);
        m.insert(UnitTypeId::Reaper, UnitTypeId::Barracks);
        m.insert(UnitTypeId::Ghost, UnitTypeId::GhostAcademy);
        m.insert(UnitTypeId::Hellion, UnitTypeId::Factory);
        m.insert(UnitTypeId::WidowMine, UnitTypeId::Factory);
        m.insert(UnitTypeId::Cyclone, UnitTypeId::Factory);
        m.insert(UnitTypeId::SiegeTank, UnitTypeId::Factory);
        m.insert(UnitTypeId::Thor, UnitTypeId::Armory);
        m.insert(UnitTypeId::VikingFighter, UnitTypeId::Starport);
        m.insert(UnitTypeId::Banshee, UnitTypeId::Starport);
        m.insert(UnitTypeId::Liberator, UnitTypeId::Starport);
        m.insert(UnitTypeId::Raven, UnitTypeId::Starport);
        m.insert(UnitTypeId::Medivac, UnitTypeId::Starport);
        m.insert(UnitTypeId::Battlecruiser, UnitTypeId::FusionCore);
        //Structures
        m.insert(UnitTypeId::Barracks, UnitTypeId::SupplyDepot);
        m.insert(UnitTypeId::Factory, UnitTypeId::Barracks);
        m.insert(UnitTypeId::Starport, UnitTypeId::Factory);
        m.insert(UnitTypeId::BarracksTechLab, UnitTypeId::Barracks);
        m.insert(UnitTypeId::BarracksReactor, UnitTypeId::Barracks);
        m.insert(UnitTypeId::FactoryTechLab, UnitTypeId::Factory);
        m.insert(UnitTypeId::FactoryReactor, UnitTypeId::Factory);
        m.insert(UnitTypeId::StarportTechLab, UnitTypeId::Starport);
        m.insert(UnitTypeId::StarportReactor, UnitTypeId::Starport);
        m.insert(UnitTypeId::GhostAcademy, UnitTypeId::Barracks);
        m.insert(UnitTypeId::Armory, UnitTypeId::Factory);
        m.insert(UnitTypeId::FusionCore, UnitTypeId::Starport);
        m.insert(UnitTypeId::SupplyDepot, UnitTypeId::CommandCenter);
        m.insert(UnitTypeId::Refinery, UnitTypeId::CommandCenter);
        m.insert(UnitTypeId::EngineeringBay, UnitTypeId::CommandCenter);
        m.insert(UnitTypeId::Bunker, UnitTypeId::Barracks);
        m.insert(UnitTypeId::MissileTurret, UnitTypeId::EngineeringBay);
        m.insert(UnitTypeId::SensorTower, UnitTypeId::EngineeringBay);
        m
    };
}
