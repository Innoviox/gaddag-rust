use crate::game::Game;
use crate::utils::{self, Direction, ItemRemovable, Move, Position, RESET};

use itertools::Itertools;
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

    rack: Vec<char>,
    curr_move: Move,
    valid: bool,
    type_pos: Option<Position>,
    exch: Vec<usize>,
}

impl<'a> TermionGame<'a> {
    pub fn of(g: &'a mut Game) -> TermionGame {
        let mut tg = TermionGame {
            game: g,
            pos: None,
            dir: Direction::Across,
            word: String::new(),

            rack: vec![],
            curr_move: Move::none(),
            valid: false,
            type_pos: None,
            exch: vec![],
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

            let s = self.game.states() - 1;
            let mut x = 75;
            if s % 2 == 1 {
                x += 39;
            }
            let y = 4 + s / 2;

            let m = Move::with(&self.word, pos, self.dir);
            self.curr_move = Move::of(&m);
            self.valid = self.game.get_board_mut().valid_move(&m);

            // edge case because valid() doesn't check for 1-length words
            if self.game.state == 1 && self.word.len() == 1 {
                self.valid = false;
            }

            write!(stdout, "{goto}", goto = cursor::Goto(x as u16, y as u16)).expect("fail");
            if self.valid {
                write!(stdout, "{color}", color = color::Fg(color::Green)).expect("fail");
            } else {
                write!(stdout, "{color}", color = color::Fg(color::Red)).expect("fail");
            }

            write!(
                stdout,
                "{:<3}: {:<12}",
                pos.to_str(self.dir),
                self.game.get_board().format(&m, true),
            )
            .expect("fail");

            if self.valid {
                self.game
                    .get_board_mut()
                    .score_without_sums(&mut self.curr_move);
                let s = self.curr_move.score as u32;
                write!(
                    stdout,
                    " +{:<03}/{:<03}",
                    s,
                    self.game.get_current_player().score + s,
                )
                .expect("fail");
            }

            write!(stdout, "{reset}", reset = RESET).expect("fail");
        } else {
            for i in self.exch.iter() {
                write!(
                    stdout,
                    "{}{}{}{}{}",
                    cursor::Goto((*i as u16) * 5 + 18, 36),
                    color::Fg(color::Yellow),
                    utils::letter_with_score(&self.rack[*i], &self.game.get_board().bag),
                    RESET,
                    termion::cursor::Hide
                )
                .expect("fail");

                self.valid = true;
            }
        }
    }

    // pub fn curr_move(&self) -> Move {
    // Move::with(&self.word, self.pos.unwrap(), self.dir)
    // }

    pub fn handle_click(&mut self, x: u16, y: u16) {
        if y < 4 || x < 7 {
            return;
        } // clicked somewhere that isnt a square
        if y > 33 || x > 66 {
            if 34 < y && y < 38 && 16 < x && x < 51 {
                self.handle_exch(x, y);
            }
            return;
        } // clicked off board, todo: exchange
        if y % 2 == 1 || (x - 6) % 4 == 0 {
            return;
        } // clicked between squares
        let new_pos = Position {
            row: ((y - 4) / 2) as usize,
            col: ((x - 6) / 4) as usize,
        };

        if self.game.get_board().is_letter(new_pos) {
            return;
        }
        if let Some(old_pos) = self.type_pos {
            self.reset(true);
            if old_pos == new_pos {
                // https://github.com/rust-lang/rust/issues/53667
                self.dir = self.dir.flip();
            }
        }

        self.pos = Some(new_pos);

        self.pre_word();
    }

    pub fn handle_exch(&mut self, x: u16, _y: u16) {
        self.reset(false);

        let i = ((x - 16) / 5) as usize;
        if self.exch.contains(&i) {
            self.exch._remove_item(i);
        } else {
            self.exch.push(i);
        }
    }

    pub fn handle_char(&mut self, c: char) {
        if c == '\n' {
            return self.handle_move();
        }

        if let Some(pos) = self.pos {
            // must click before typing
            // todo: shift to place blank

            let u = c.to_ascii_uppercase();
            if ('A'..='Z').contains(&u) {
                if u != c && self.rack.contains(&u) {
                    self.rack._remove_item(u);
                    self.word.push(u);
                } else if self.rack.contains(&'?') {
                    self.rack._remove_item('?');
                    self.word.push(c.to_ascii_lowercase());
                }
            }

            let mut p = Position {
                row: pos.row,
                col: pos.col,
            };
            for _ in 0..self.word.len() {
                p.tick(self.dir);
            }
            while self.game.get_board().is_letter(p) {
                self.word.push(self.game.get_board().at_position(p));
                p.tick(self.dir);
            }
        }
    }

    pub fn handle_move(&mut self) {
        if self.valid {
            if !self.exch.is_empty() {
                self.curr_move = Move {
                    word: self
                        .exch
                        .iter()
                        .map(|i| self.rack[*i])
                        .sorted()
                        .collect::<String>(),
                    position: Position { row: 0, col: 0 },
                    direction: Direction::Down,
                    score: 0,
                    evaluation: 0f32,
                    typ: utils::Type::Exch,
                }
            }
            self.game.force_move(&self.curr_move);
            self.tick();
        }
    }

    pub fn handle_backspace(&mut self) {
        if let Some(c) = self.word.pop() {
            self.rack.push(c);
        }
    }

    pub fn handle(&mut self, evt: Event) {
        if self.game.finished {
            return;
        }

        match evt {
            Event::Key(Key::Char(c)) => {
                self.handle_char(c);
            }
            Event::Key(Key::Backspace) => {
                self.handle_backspace();
            }
            Event::Mouse(me) => {
                match me {
                    // MouseEvent::Press(_, a, b) |
                    MouseEvent::Release(a, b) /* |
                    MouseEvent::Hold(a, b) */ => {
                        self.handle_click(a, b);
                    },

                    _ => ()
                }
            }
            _ => {}
        }
    }

    fn pre_word(&mut self) {
        if let Some(pos) = self.pos {
            self.type_pos = Some(pos);
            let mut p = Position {
                row: pos.row,
                col: pos.col,
            };

            let mut edge = p.tick_opp(self.dir);
            while self.game.get_board().is_letter(p) {
                self.word = self.game.get_board().at_position(p).to_string() + &self.word;
                edge &= p.tick_opp(self.dir);
            }
            if edge {
                p.tick(self.dir);
            }

            self.pos = Some(p)
        }
    }

    fn reset(&mut self, exch: bool) {
        self.pos = None;
        self.word = String::new();
        self.valid = false;
        self.set_rack();

        if exch {
            self.exch = vec![];
        }
    }

    fn tick(&mut self) {
        if self.game.is_over() {
            self.game.finish();
        } else {
            let n = &self.game.get_current_player().name;
            if n.starts_with("AI") {
                let v: Vec<&str> = n.split('/').collect();
                let diff = if v.len() > 1 {
                    v[1].parse::<usize>().unwrap()
                } else {
                    0
                };
                self.game.force_move_k(diff);

                self.reset(true);
                self.tick();
            }
            self.reset(true);
        }
    }
}

pub fn main(p1: String, p2: String) {
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());

    let mut g = Game::with(p1, p2);
    let mut game = TermionGame::of(&mut g);

    game.tick();

    game.display(&mut stdout);
    stdout.flush().unwrap();

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Ctrl('c')) => break,
            x => game.handle(x),
        }

        game.display(&mut stdout);
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
