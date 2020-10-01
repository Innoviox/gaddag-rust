use termion::color;
use termion::clear;
use termion::cursor;
use crate::game::Game;

use termion::event::*;
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;

use std::io::{self, Write, stdout, stdin};

fn write_screen(g: &mut Game) {
    let s1 = format!("{}", g.get_board());

    /*
    Format:

    XX XXXXXXXXXX +0XX/XXX | 
    */
    // let s2 = format!("{}", g);

    
}

pub fn main() {
    let mut game = Game::default();

    let stdin = stdin();
    let mut stdout = MouseTerminal::from(io::stdout().into_raw_mode().unwrap());

    write!(stdout, "{}{}q to exit. Type stuff, use alt, and so on.{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1),
           termion::cursor::Hide).unwrap();
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
                        write!(stdout, "{}", cursor::Goto(a, b)).unwrap();
                    }
                }
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}