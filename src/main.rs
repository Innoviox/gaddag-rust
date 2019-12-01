#[macro_use] extern crate itertools;
#[macro_use] extern crate relm;
extern crate gdk;
extern crate gdk_sys;

use crate::player::Player;
use crate::utils::Position;
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


#[derive(Msg)]
pub enum Msg {
    // Decrement,
    // Increment,
    Tick,
    Quit,
}

struct Win {
    // …
    model: Game,
    window: Window,

    grid: Grid
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
                    let m = self.model.do_move().0;

                    let mut p = m.position.clone();
                    for i in m.word.chars() {
                        let at = self.model.get_board().at_position(p);
                        if let Some(w) = self.grid.get_child_at(p.row as i32, p.col as i32) {
                            if let Ok(l) = w.dynamic_cast::<Label>() {
                                l.set_text(&at.to_string());
                            }
                        }
                        p.tick(m.direction);
                    }
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

        let grid = gtk::Grid::new();
        grid.set_row_homogeneous(true);
        grid.set_column_homogeneous(true); 
        grid.set_row_spacing(2);
        grid.set_column_spacing(2);
        grid.set_border_width(1);     

        for row in 0..15 {
            for col in 0..15 {
                let label = Label::new(Some(" "));
                let at = model.get_board().at_position(Position { row, col });
                label.override_background_color(gtk::StateFlags::empty(), Some(&colors[&at]));
                grid.attach(&label, row as i32, col as i32, 1, 1);
            }
        }

        let window = Window::new(WindowType::Toplevel);
        window.add(&grid);
        window.set_default_size(400, 400);

        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));
        interval(relm.stream(), 1000, || Msg::Tick);

        window.show_all();

        let mut win = Win {
            model,
            window,
            grid
        };

        // win.update(Msg::Tick);
        win
    }
}

fn main() {
    Win::run(()).unwrap();
}