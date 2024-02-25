mod game;

use clap::Parser;
use game::Game;

fn main() -> game::Result<()> {
    let config = game::Config::parse();

    let mut game = Game::new(config)?;

    game.generate();
    game.open(5, 5);
    game.print();

    Ok(())
}
