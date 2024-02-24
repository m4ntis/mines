use std::fmt;

use clap::Parser;

#[derive(Parser, Debug, Clone, Copy)]
#[command(version, about, long_about=None)]
pub struct Config {
    /// Tile count in x axis
    #[arg(short, long, default_value_t = 15)]
    pub x: usize,

    /// Tile count in y axis
    #[arg(short, long, default_value_t = 15)]
    pub y: usize,

    /// Number of mines placed in the grid
    #[arg(short, long, default_value_t = 20)]
    pub mines: usize,
}

impl Config {
    pub fn tile_count(&self) -> usize {
        self.x * self.y
    }
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
            self.config.tile_count()
        )
    }
}
