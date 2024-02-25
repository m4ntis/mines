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
        if config.mines > config.x * config.y || config.x * config.y == 0 {
            return Err(ConfigError::new(config));
        }

        Ok(Self {
            config: config,
            board: vec![vec![Tile::new(); config.y]; config.x],
        })
    }

    pub fn generate(&mut self) {
        for _i in 0..self.config.mines {
            loop {
                let (x, y) = self.gen_rand_coords();
                if let tile::Value::Mine = self.board[x][y].value {
                    continue;
                }

                self.board[x][y].set_mine();
                Self::for_adjacent(x, y, self.width(), self.height(), |x, y| {
                    self.board[x][y].inc_mine_count()
                });
                break;
            }
        }
    }

    pub fn open(&mut self, x: usize, y: usize) {
        if x >= self.width() || y >= self.height() {
            return;
        }

        if let Some(v) = self.board[x][y].open() {
            match v {
                tile::Value::Mine => println!("Game over :("),
                tile::Value::Empty(n) => {
                    // Open empty spaces with no mines adjacent recursively
                    if n == 0 {
                        Self::for_adjacent(x, y, self.width(), self.height(), |x, y| {
                            self.open(x, y)
                        });
                    }
                }
            }
        }
    }

    pub fn print(&self) {
        for y in 0..self.height() {
            let mut row_string = String::new();

            for x in 0..self.width() {
                row_string.push_str(&self.board[x][y].to_string())
            }

            println!("{}", row_string);
        }
    }

    fn width(&self) -> usize {
        self.board.len()
    }

    fn height(&self) -> usize {
        self.board[0].len()
    }

    fn gen_rand_coords(&self) -> (usize, usize) {
        let num = rand::thread_rng().gen_range(0..self.config.tile_count());
        (num % self.width(), num / self.width())
    }

    fn for_adjacent(
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        mut func: impl FnMut(usize, usize),
    ) {
        Self::for_adjacent_in_row(x, y, width, height, &mut func);

        if x > 0 {
            Self::for_adjacent_in_row(x - 1, y, width, height, &mut func);
        }
        if x < width - 1 {
            Self::for_adjacent_in_row(x + 1, y, width, height, &mut func);
        }
    }

    fn for_adjacent_in_row(
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        func: &mut impl FnMut(usize, usize),
    ) {
        func(x, y);

        if y > 0 {
            func(x, y - 1);
        }

        if y < height - 1 {
            func(x, y + 1);
        }
    }
}
