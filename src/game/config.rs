use std::fmt;

use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about=None)]
pub struct Config {
    /// Tile count in x axis
    #[arg(short, long, default_value_t = 15)]
    pub x: u32,

    /// Tile count in y axis
    #[arg(short, long, default_value_t = 15)]
    pub y: u32,

    /// Number of mines placed in the grid
    #[arg(short, long, default_value_t = 20)]
    pub mines: u32,
}

pub type Result<T> = std::result::Result<T, ConfigError>;

#[derive(Clone)]
pub struct ConfigError {
    config: Config,
}

impl ConfigError {
    pub fn new(config: Config) -> Self {
        Self { config: config }
    }
}

impl fmt::Debug for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "invalid mine count: {} > {} * {} ({})",
            self.config.mines,
            self.config.x,
            self.config.y,
            self.config.x * self.config.y
        )
    }
}
