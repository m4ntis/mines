mod console_runner;
mod game;

use clap::Parser;
use console_runner::ConsoleRunner;
use game::Game;

fn main() -> game::Result<()> {
    let config = game::Config::parse();

    let game = Game::new(config)?;
    let mut runner = ConsoleRunner::new(game);

    runner.run();

    Ok(())
}
