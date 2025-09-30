use rust_sc2::prelude::*;


#[derive(Debug, Clone)]
pub struct MacroManager {
    pub expand_priority: bool,
    pub barracks_priority: Option<UnitTypeId>,
    pub factory_priority: Option<UnitTypeId>,
    pub starport_priority: Option<UnitTypeId>,
    pub starter_reaper: bool,
    pub starter_banshee: bool,
}

impl Default for MacroManager {
    fn default() -> Self {
        Self {
            expand_priority: false,
            barracks_priority: None,
            factory_priority: None,
            starport_priority: None,
            starter_reaper: true,
            starter_banshee: true,
        }
    }
}