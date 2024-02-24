mod game;

use clap::Parser;
use game::Game;

fn main() -> game::Result<()> {
    let config = game::Config::parse();

    let game = Game::new(config)?;

    game.run();

    Ok(())
}
