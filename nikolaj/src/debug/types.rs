#[derive(Debug, Clone, PartialEq)]
pub enum WorkersMiningSteps {
    MineralFarAway,
    MineralOffsetWalk,
    MineralGather,
    BaseFarAway,
    BaseOffsetWalk,
    BaseReturn,
    None,
}

#[derive(Debug, Clone)]
pub struct WorkersCurrentMiningStep {
    pub tag: u64,
    pub step: WorkersMiningSteps,
}
