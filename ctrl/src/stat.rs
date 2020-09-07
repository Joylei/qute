pub struct Processor {
    index: u8,
    usage: f32,
    mhz: f32,
}

pub struct CpuInfo {
    pub model: String,
    pub cores: Vec<Processor>,
    pub load: [f32; 3],
}

pub struct MemInfo {
    pub total: u32,
    pub free: u32,
    pub used: u32,
}
