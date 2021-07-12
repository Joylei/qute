use crate::config::Config;
use anyhow::Result;
use qute_ctrl::platform::Platform;

/// global options
#[derive(Default)]
pub struct Options {
    pub help: bool,
    /// Silence all output
    pub quiet: bool,
    pub verbose: usize,
}

pub struct Context {
    config: Config,
    opts: Options,
}

impl Context {
    pub fn new(config: Config, opts: Options) -> Self {
        Self { config, opts }
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn get_opts(&self) -> &Options {
        &self.opts
    }

    pub fn get_platform(&self) -> Result<Platform> {
        Ok(Platform::with_default()?)
    }
}
