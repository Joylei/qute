pub struct FanConfig {
    pub min_speed: u16,
    pub max_speed: u16,
}

pub struct Config {
    pub fan: Vec<FanConfig>,
}
