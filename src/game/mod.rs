mod config;
mod tile;

use config::ConfigError;
pub use config::{Config, Result};
use rand::Rng;
use tile::Tile;

pub struct Game {
    config: Config,

    board: Vec<Vec<Tile>>,
}

impl Game {
    pub fn new(config: Config) -> Result<Self> {
        if config.mines > config.x * config.y {
            return Err(ConfigError::new(config));
        }

        Ok(Self {
            config: config,
            board: vec![vec![Tile::new(); config.y]; config.x],
        })
    }

    pub fn generate(&mut self) {
        for i in 0..self.config.mines {
            loop {
                let (x, y) = self.gen_rand_coords();
                if let tile::Value::Mine = self.board[x][y].value {
                    continue;
                }

                self.board[x][y].set_mine();
                self.inc_adjacent(x, y);
                break;
            }
        }
    }

    pub fn print(&self) {
        for y in 0..self.config.y {
            let mut row_string = String::new();

            for x in 0..self.config.x {
                match self.board[x][y].value {
                    tile::Value::Mine => row_string.push('X'),
                    tile::Value::Empty(n) => {
                        if n == 0 {
                            row_string.push(' ');
                        } else {
                            row_string.push_str(&n.to_string());
                        }
                    }
                };
            }

            println!("{}", row_string);
        }
    }

    fn gen_rand_coords(&self) -> (usize, usize) {
        let num = rand::thread_rng().gen_range(0..self.config.tile_count());
        (num % self.config.x, num / self.config.x)
    }

    fn inc_adjacent(&mut self, x: usize, y: usize) {
        self.inc_adjacent_in_row(x, y);

        if x > 0 {
            self.inc_adjacent_in_row(x - 1, y);
        }
        if x < self.config.x - 1 {
            self.inc_adjacent_in_row(x + 1, y);
        }
    }

    fn inc_adjacent_in_row(&mut self, x: usize, y: usize) {
        self.board[x][y].inc_mine_count();

        if y > 0 {
            self.board[x][y - 1].inc_mine_count();
        }

        if y < self.config.y - 1 {
            self.board[x][y + 1].inc_mine_count();
        }
    }
}
