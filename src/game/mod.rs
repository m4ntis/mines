mod config;

use config::ConfigError;
pub use config::{Config, Result};

pub struct Game {
    config: Config,
}

impl Game {
    pub fn new(config: Config) -> Result<Self> {
        if config.mines > config.x * config.y {
            return Err(ConfigError::new(config));
        }

        Ok(Self { config: config })
    }

    pub fn run(&self) {
        println!(
            "Starting a game of {} by {} with {} mines",
            self.config.x, self.config.y, self.config.mines
        )
    }
}
