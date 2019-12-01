#[macro_use] extern crate itertools;
#[macro_use] extern crate relm;
extern crate gdk;
extern crate gdk_sys;

use crate::player::Player;
use crate::utils::{Position, Move};
use crate::board::{Board, STATE};
use crate::game::Game;
use std::time::SystemTime;
use std::collections::HashMap;

mod bag;
mod utils;
mod board;
mod dictionary;
mod player;
mod game;

use relm_derive::{Msg, widget};
use relm::{Widget, Relm, Update, interval};
use gtk::prelude::*;
use gtk::{Inhibit, Window, WindowType};
use gtk::Orientation::{Vertical, Horizontal};
use gtk::{
    Label, Border, Grid, Button
};
use gdk_sys::GdkRGBA;
use gdk::RGBA;


const GREY: RGBA = RGBA { red: 0.38, green: 0.38, blue: 0.38, alpha: 1.0};
const WHITE: RGBA = RGBA { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0};

#[derive(Msg)]
pub enum Msg {
    // Decrement,
    // Increment,
    Tick,
    Quit,
}

struct Win {
    // necessary fields
    model: Game,
    window: Window,

    // ui fields
    board: Grid,

    // internal fields
    last_move: Move,
}

impl Win {
    fn place(&mut self, m: &Move, color: &str) {
        let mut p = m.position.clone();
        for i in m.word.chars() {
            let at = self.model.get_board().at_position(p);
            if let Some(w) = self.board.get_child_at(p.row as i32, p.col as i32) {
                if let Ok(l) = w.dynamic_cast::<Label>() {
                    l.override_background_color(gtk::StateFlags::empty(), Some(&GREY));
                    l.set_markup(&format!("<span color=\"{}\">{}</span>", color, at));
                }
            }
            p.tick(m.direction);
        }
    }
}

impl Update for Win {
    // Specify the model used for this widget.
    type Model = Game;
    // Specify the model parameter used to init the model.
    type ModelParam = ();
    // Specify the type of the messages sent to the update function.
    type Msg = Msg;

    // Return the initial model.
    fn model(_: &Relm<Self>, _: ()) -> Game {
        Game::default()
    }

    // The model may be updated when a message is received.
    // Widgets may also be updated in this function.
    fn update(&mut self, event: Msg) {
        match event {
            Tick => {
                if !self.model.is_over() {
                    // why do i have to do this??? why cant i do
                    // self.place(&self.last_move...)? idk
                    let lm = Move::of(&self.last_move);
                    self.place(&lm, "white");

                    let m = self.model.do_move().0;

                    self.place(&m, "yellow");

                    self.last_move = Move::of(&m);
                }
            },
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    // Specify the type of the root widget.
    type Root = Window;

    // Return the root widget.
    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    // Create the widgets.
    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let mut colors = HashMap::<char, RGBA>::new();
        colors.insert('#', RGBA { red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0} ); // "red");
        colors.insert('.', RGBA { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0} ); // "white");
        colors.insert('-', RGBA { red: 0.48, green: 0.79, blue: 0.90, alpha: 1.0} ); // "light blue");
        colors.insert('^', RGBA { red: 0.94, green: 0.73, blue: 0.73, alpha: 1.0} ); // "pink");
        colors.insert('*', RGBA { red: 0.94, green: 0.73, blue: 0.73, alpha: 1.0} ); // "pink");
        colors.insert('+', RGBA { red: 0.2, green: 0.38, blue: 0.92, alpha: 1.0} ); // "dark blue");

        let board = gtk::Grid::new();
        board.set_row_homogeneous(true);
        board.set_column_homogeneous(true); 
        board.set_row_spacing(2);
        board.set_column_spacing(2);
        board.set_border_width(1);     

        for row in 0..15 {
            for col in 0..15 {
                let label = Label::new(Some(" "));
                let at = model.get_board().at_position(Position { row, col });
                label.override_background_color(gtk::StateFlags::empty(), Some(&colors[&at]));
                board.attach(&label, row as i32, col as i32, 1, 1);
            }
        }

        let window = Window::new(WindowType::Toplevel);
        window.add(&board);
        window.set_default_size(400, 400);

        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));
        interval(relm.stream(), 1000, || Msg::Tick);

        window.show_all();

        let mut win = Win {
            model,
            window,
            board,
            last_move: Move::none()
        };

        // win.update(Msg::Tick);
        win
    }
}

fn main() {
    Win::run(()).unwrap();
}