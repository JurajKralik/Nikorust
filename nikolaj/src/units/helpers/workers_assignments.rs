#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkerRole {
    Mineral,
    Gas,
    Repair,
    Busy,
    Idle
}

#[derive(Debug, Clone)]
pub struct RepairAllocation {
    pub tag: u64,
    pub is_structure: bool,
    pub workers: Vec<u64>,
    pub max_workers: usize,
}

#[derive(Debug)]
pub struct ResourceAllocation {
    pub resource_tag: u64,
    pub worker_role: WorkerRole,
    pub workers: Vec<u64>,
}

#[derive(Debug, Default)]
pub struct ResourceSaturation {
    pub mineral_tags_undersaturated: Vec<u64>,
    pub mineral_tags_oversaturated: Vec<u64>,
    pub refinery_tags_undersaturated: Vec<u64>,
    pub refinery_tags_oversaturated: Vec<u64>,
}