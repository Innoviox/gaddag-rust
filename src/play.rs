use crate::game::Game;
use crate::utils::{Direction, ItemRemovable, Move, Position, RESET};

use std::io::{self, stdin, Stdout, Write};
use termion::color;
use termion::cursor;
use termion::event::*;
use termion::input::{MouseTerminal, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};

pub type TTY = MouseTerminal<RawTerminal<Stdout>>;

pub struct TermionGame<'a> {
    game: &'a mut Game,

    pos: Option<Position>,
    dir: Direction,
    word: String,

    mouse_position: Position,

    rack: Vec<char>,
}

impl<'a> TermionGame<'a> {
    pub fn of(g: &'a mut Game) -> TermionGame {
        let mut tg = TermionGame {
            game: g,
            pos: None,
            dir: Direction::Across,
            word: String::new(),

            mouse_position: Position { row: 1, col: 1 },
            rack: vec![],
        };

        tg.set_rack();

        tg
    }

    fn set_rack(&mut self) {
        self.rack = self.game.get_current_player().rack.clone();
    }

    pub fn display(&mut self, stdout: &mut TTY) {
        let s = self.game.to_str().replace("\n", "\n\r");

        write!(stdout, "{}", termion::clear::All).expect("fail");
        write!(stdout, "{}{}", cursor::Goto(1, 1), s).expect("fail");

        if let Some(pos) = self.pos {
            let mut x = (pos.col * 4 + 7) as u16;
            let mut y = (pos.row * 2 + 4) as u16;

            for c in self.word.chars() {
                write!(stdout, "{} {} ", cursor::Goto(x as u16, y as u16), c).expect("fail");

                if self.dir == Direction::Across {
                    x += 4;
                } else {
                    y += 2;
                }
            }

            write!(
                stdout,
                "{} {} {}",
                cursor::Goto(x as u16, y as u16),
                self.dir.to_str(),
                termion::cursor::Hide
            )
            .expect("fail");

            let s = self.game.states();
            let mut x = 75;
            if s % 2 == 1 {
                x += 29;
            }
            let y = 4 + s / 2;

            let m = Move::with(&self.word, pos, self.dir);

            write!(
                stdout,
                "{goto}{red}{pos} {word}{reset}",
                goto = cursor::Goto(x as u16, y as u16),
                red = color::Fg(color::Red),
                pos = pos.to_str(self.dir),
                word = self.game.get_board().format(&m, true),
                reset = RESET
            )
            .expect("fail");
        }
    }

    pub fn handle_click(&mut self, x: u16, y: u16) {
        if y % 2 == 1 || y < 4 || x < 7 || (x - 6) % 4 == 0 {
            return;
        } // clicked somewhere that isnt a square

        let new_pos = Position {
            row: ((y - 4) / 2) as usize,
            col: ((x - 6) / 4) as usize,
        };

        if let Some(old_pos) = self.pos {
            if old_pos == new_pos {
                // https://github.com/rust-lang/rust/issues/53667
                self.dir = self.dir.flip();
            } else {
                self.word = String::new(); // reset word b/c new position was chosen
                self.set_rack();
            }
        }
        self.pos = Some(new_pos);
        self.mouse_position = Position {
            row: y as usize,
            col: x as usize,
        }
    }

    pub fn handle_char(&mut self, c: char) {
        // todo: shift to place blank
        let c = c.to_ascii_uppercase();
        if 'A' <= c && c <= 'Z' {
            if self.rack.contains(&c) {
                self.rack._remove_item(c);
                self.word.push(c);
            } else if self.rack.contains(&'?') {
                self.rack._remove_item('?');
                self.word.push(c.to_ascii_lowercase());
            }
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
            }
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
