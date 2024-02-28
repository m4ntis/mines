extern crate termion;

use crate::game::{Game, TileMarking, TileState, TileValue};

use std::io::{self, Write};

use termion::event::*;
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

pub struct ConsoleRunner {
    game: Game,
}

impl ConsoleRunner {
    pub fn new(game: Game) -> Self {
        Self { game: game }
    }

    pub fn run(&mut self) {
        print!("{esc}c", esc = 27 as char);
        reset_cursor_position();

        loop {
            self.print();
            reset_cursor_position();

            let (x, y, click_type) = match read_mouse_click() {
                Some((x, y, click_type)) => (x, y, click_type),
                None => {
                    std::process::exit(0);
                }
            };

            match click_type {
                MouseButton::Left => {
                    if self.game.open(x, y) {
                        self.print_game_over();
                        return;
                    }
                }
                MouseButton::Right => {
                    self.game.cycle(x, y);
                }
                _ => {}
            };
        }
    }

    fn print(&self) {
        for y in 0..self.game.height() {
            let mut row_string = String::new();

            for x in 0..self.game.width() {
                let (state, value) = self.game.get_tile(x, y).unwrap();
                row_string.push_str(&get_tile_char(state, value));
            }

            println!("{}", row_string);
        }
    }

    fn print_game_over(&self) {
        self.print();
        println!("Game over :(");
    }
}

fn get_tile_char(state: TileState, value: TileValue) -> String {
    match state {
        TileState::Opened => match value {
            TileValue::Mine => "*".to_string(),
            TileValue::Empty(n) => {
                if n == 0 {
                    " ".to_string()
                } else {
                    n.to_string()
                }
            }
        },
        TileState::Closed(mark) => match mark {
            TileMarking::None => "⎕".to_string(),
            TileMarking::Flag => "⌻".to_string(),
            TileMarking::Question => "⍰".to_string(),
        },
    }
}

fn read_mouse_click() -> Option<(usize, usize, MouseButton)> {
    let stdin = io::stdin();
    let _stdout = io::stdout().into_raw_mode().unwrap();

    for c in stdin.events() {
        let event = c.unwrap();

        match event {
            Event::Key(Key::Char('q')) => {
                return None;
            }
            Event::Mouse(mouse_event) => match mouse_event {
                MouseEvent::Press(button, x, y) => {
                    if x == 0 || y < 2 {
                        continue;
                    }

                    return Some(((x - 1).into(), (y - 2).into(), button));
                }
                _ => {}
            },
            _ => {}
        }
    }

    return Some((0, 0, MouseButton::Left));
}

fn reset_cursor_position() {
    let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());

    writeln!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
}
