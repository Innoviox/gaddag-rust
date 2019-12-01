#[macro_use] extern crate itertools;
#[macro_use] extern crate relm;
extern crate gdk;
extern crate gdk_sys;

use crate::player::Player;
use crate::utils::Position;
use crate::board::{Board, STATE};
use std::time::SystemTime;
use std::collections::HashMap;

mod bag;
mod utils;
mod board;
mod dictionary;
mod player;

use relm_derive::{Msg, widget};
use relm::{Widget, Relm, Update};
use gtk::prelude::*;
use gtk::{Inhibit, Window, WindowType};
use gtk::Orientation::{Vertical, Horizontal};
use gtk::{
    Label, CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION, Border, Grid
};
use gdk_sys::GdkRGBA;
use gdk::RGBA;


#[derive(Msg)]
pub enum Msg {
    // Decrement,
    // Increment,
    Quit,
}

struct Win {
    // …
    model: Board,
    window: Window,
}

impl Update for Win {
    // Specify the model used for this widget.
    type Model = Board;
    // Specify the model parameter used to init the model.
    type ModelParam = ();
    // Specify the type of the messages sent to the update function.
    type Msg = Msg;

    // Return the initial model.
    fn model(_: &Relm<Self>, _: ()) -> Board {
        Board::default()
    }

    // The model may be updated when a message is received.
    // Widgets may also be updated in this function.
    fn update(&mut self, event: Msg) {
        match event {
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
        colors.insert('^', RGBA { red: 0.90, green: 0.49, blue: 0.49, alpha: 1.0} ); // "pink");
        colors.insert('*', RGBA { red: 0.90, green: 0.49, blue: 0.49, alpha: 1.0} ); // "pink");
        colors.insert('+', RGBA { red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0} ); // "dark blue");

        let grid = gtk::Grid::new();
        grid.set_row_homogeneous(true);
        grid.set_column_homogeneous(true); 
        grid.set_row_spacing(2);
        grid.set_column_spacing(2);
        grid.set_border_width(1);     

        for row in 0..15 {
            for col in 0..15 {
                let label = Label::new(Some(" "));
                let at = model.at_position(Position { row, col });
                label.override_background_color(gtk::StateFlags::empty(), Some(&colors[&at]));
                // label.set_markup(&format!("<span face=\"monospace\" background=\"{}\"> </span>", 
                //                  colors[&at]));

                grid.attach(&label, row as i32, col as i32, 1, 1);
            }
        }

        // GTK+ widgets are used normally within a `Widget`.
        let window = Window::new(WindowType::Toplevel);
        window.add(&grid);
        // Connect the signal `delete_event` to send the `Quit` message.
        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));
        // There is also a `connect!()` macro for GTK+ events that do not need a
        // value to be returned in the callback.

        window.show_all();

        Win {
            model,
            window: window,
        }
    }
}

fn main() {
    Win::run(()).unwrap();
}