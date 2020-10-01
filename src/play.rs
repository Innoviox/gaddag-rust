use crate::game::Game;
use crate::utils::{Position, Direction};

use termion::color;
use termion::clear;
use termion::cursor;
use termion::event::*;
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;

use itertools::{
    Itertools,
    EitherOrBoth::*,
};

use std::io::{self, Write, stdout, stdin};
use std::cmp;

pub struct TermionGame<'a> {
    game: &'a mut Game,

    pos:  Option<Position>,
    dir: Direction,

    mouse_position: Position
}

impl<'a> TermionGame<'a> {
    pub fn of(g: &'a mut Game) -> TermionGame {
        TermionGame {
            game: g,
            pos: None,
            dir: Direction::Across,

            mouse_position: Position { row: 1, col: 1 }
        }
    }

    pub fn display(&mut self) {
        let s = self.game.to_str().replace("\n", "\n\r");

        write!(stdout, "{}", termion::clear::All);
        write!(stdout, "{}", s);

        // let (x, y) = self.mouse_position();
        // if x != 1 && y != 1 {
        //     write!(stdout, "{}{}", termion::cursor::Goto(x as u16, y as u16),
        //                            self.dir.to_str());
        // }
    }

    pub fn handle_click(&mut self, x: u16, y: u16) {
        if y % 2 == 1 || y < 4 || 
           x < 7 || (x - 6) % 4 == 0 { return } // clicked somewhere that isnt a square

        self.pos = Some(Position { row : (y - 4) as usize, col : ((x - 6) / 4) as usize});
        write!(stdout, "{}{}{}", cursor::Goto(x, y), y - 4, (x - 6) / 4);
    }
}

pub fn main() {
    let mut g = Game::default();
    let mut game = TermionGame::of(&mut g);

    let stdin = stdin();
    let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());

    stdout.flush().unwrap();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(_, a, b) |
                    MouseEvent::Release(a, b) |
                    MouseEvent::Hold(a, b) => {
                        // write!(stdout, "{}", cursor::Goto(a, b)).unwrap();
                        game.handle_click(a, b);
                    }
                }
            }
            _ => {}
        }

        game.display();
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}