use crate::game::Game;
use crate::utils::{Position, Direction};

use termion::color;
use termion::clear;
use termion::cursor;
use termion::event::*;
use termion::input::{TermRead, MouseTerminal};
use termion::raw::{IntoRawMode, RawTerminal};

use itertools::{
    Itertools,
    EitherOrBoth::*,
};

use std::io::{self, Write, stdout, Stdout, stdin};
use std::cmp;

pub type TTY = MouseTerminal<RawTerminal<Stdout>>;

pub struct TermionGame<'a> {
    game: &'a mut Game,

    pos:  Option<Position>,
    dir: Direction,
    word: String,

    mouse_position: Position,
}

impl<'a> TermionGame<'a> {
    pub fn of(g: &'a mut Game) -> TermionGame {
        TermionGame {
            game: g,
            pos: None,
            dir: Direction::Across,
            word: String::new(),

            mouse_position: Position { row: 1, col: 1 },
        }
    }

    pub fn display(&mut self, stdout: &mut TTY) {
        let s = self.game.to_str().replace("\n", "\n\r");

        write!(stdout, "{}", termion::clear::All);
        write!(stdout, "{}{}", cursor::Goto(1, 1), s);

        if let Some(pos) = self.pos {
            let mut x = (pos.col * 4 + 7) as u16;
            let mut y = (pos.row + 4) as u16;

            for c in self.word.chars() {
                write!(stdout, "{} {} ", cursor::Goto(x as u16, y as u16), c);

                if self.dir == Direction::Across {
                    x += 4;
                } else {
                    y += 2;
                }
            }
            
            write!(stdout, "{} {} {}", cursor::Goto(x as u16, y as u16), 
                                       self.dir.to_str(), 
                                       termion::cursor::Hide);
        }
    }

    pub fn handle_click(&mut self, x: u16, y: u16) {
        if y % 2 == 1 || y < 4 || 
           x < 7 || (x - 6) % 4 == 0 { return } // clicked somewhere that isnt a square

        let new_pos = Position { row : (y - 4) as usize, col : ((x - 6) / 4) as usize};

        if let Some(old_pos) = self.pos {
            if old_pos == new_pos { // https://github.com/rust-lang/rust/issues/53667
                self.dir = self.dir.flip();
            } else {
                self.word = String::new(); // reset word b/c new position was chosen
            }
        }
        
        self.pos = Some(new_pos);
        self.mouse_position = Position { row : y as usize, col : x as usize }
    }

    pub fn handle_char(&mut self, c: char) { // todo: shift to place blank
        if 'a' <= c && c <= 'z' {
            self.word.push(c.to_ascii_uppercase());
        }
    }
}

pub fn main() {
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());

    let mut g = Game::default();
    let mut game = TermionGame::of(&mut g);

    game.display(&mut stdout);
    stdout.flush().unwrap();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Ctrl('c')) => break,
            Event::Key(Key::Char(c)) => {
                game.handle_char(c);
            },
            Event::Mouse(me) => {
                match me {
                    // MouseEvent::Press(_, a, b) |
                    MouseEvent::Release(a, b) /* |
                    MouseEvent::Hold(a, b) */ => {
                        game.handle_click(a, b);
                    },

                    _ => ()
                }
            }
            _ => {}
        }

        game.display(&mut stdout);
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}