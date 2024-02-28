mod config;
mod tile;

use config::ConfigError;
use tile::Tile;

pub use config::{Config, Result};
pub use tile::{TileMarking, TileState, TileValue};

use rand::Rng;

pub struct Game {
    board: Vec<Vec<Tile>>,
}

impl Game {
    /// Creates a new instance of a `Game`, generating a board according to the
    /// supplied config.
    pub fn new(config: Config) -> Result<Self> {
        if config.mines > config.x * config.y || config.x * config.y == 0 {
            return Err(ConfigError::new(config));
        }

        Ok(Self {
            board: generate_board(config),
        })
    }

    /// Opens a tile on the game's board at a given coordinate.
    ///
    /// Returns whether the opened tile was a mine, determining the end of the
    /// game.
    pub fn open(&mut self, x: usize, y: usize) -> bool {
        if !self.coordinate_in_bounds(x, y) {
            return false;
        }

        if let Some(v) = self.board[x][y].open() {
            match v {
                TileValue::Mine => return true,
                TileValue::Empty(n) => {
                    if n == 0 {
                        // Recursively open empty spaces with no adjacent mines
                        for_adjacent(x, y, self.width(), self.height(), |x, y| {
                            self.open(x, y);
                        });
                    }
                }
            }
        }

        return false;
    }

    /// Cycles a tile's marking between the different markings.
    ///
    /// Cycling an opened tile will do nothing.
    pub fn cycle(&mut self, x: usize, y: usize) {
        if !self.coordinate_in_bounds(x, y) {
            return;
        }

        self.board[x][y].cycle()
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<(TileState, TileValue)> {
        if !self.coordinate_in_bounds(x, y) {
            return None;
        }

        Some((self.board[x][y].state, self.board[x][y].value))
    }

    pub fn width(&self) -> usize {
        self.board.len()
    }

    pub fn height(&self) -> usize {
        self.board[0].len()
    }

    fn coordinate_in_bounds(&self, x: usize, y: usize) -> bool {
        return x < self.width() && y < self.height();
    }
}

fn generate_board(config: Config) -> Vec<Vec<Tile>> {
    let mut board = vec![vec![Tile::new(); config.y]; config.x];

    for _i in 0..config.mines {
        loop {
            let (x, y) = gen_rand_coords(config);
            if let TileValue::Mine = board[x][y].value {
                continue;
            }

            board[x][y].set_mine();
            for_adjacent(x, y, config.x, config.y, |x, y| {
                board[x][y].inc_mine_count()
            });
            break;
        }
    }

    board
}

fn gen_rand_coords(config: Config) -> (usize, usize) {
    let num = rand::thread_rng().gen_range(0..config.tile_count());
    (num % config.x, num / config.x)
}

fn for_adjacent(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    mut func: impl FnMut(usize, usize),
) {
    for_adjacent_in_row(x, y, height, &mut func);

    if x > 0 {
        for_adjacent_in_row(x - 1, y, height, &mut func);
    }
    if x < width - 1 {
        for_adjacent_in_row(x + 1, y, height, &mut func);
    }
}

fn for_adjacent_in_row(x: usize, y: usize, height: usize, func: &mut impl FnMut(usize, usize)) {
    func(x, y);

    if y > 0 {
        func(x, y - 1);
    }

    if y < height - 1 {
        func(x, y + 1);
    }
}
