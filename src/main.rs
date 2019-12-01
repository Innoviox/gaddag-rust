#[macro_use] extern crate itertools;
#[macro_use] extern crate relm;
extern crate gdk;

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
        let mut colors = HashMap::<char, &str>::new();
        colors.insert('#', "red");
        colors.insert('.', "white");
        colors.insert('-', "light blue");
        colors.insert('^', "pink");
        colors.insert('*', "pink");
        colors.insert('+', "dark blue");

        // let vbox = gtk::Box::new(Vertical, 0);
        // for row in 0..15 {
        //     let hbox = gtk::Box::new(Horizontal, 0);
        //     for col in 0..15 {
        //         let label = Label::new(None);
        //         let at = model.at_position(Position { row, col });
        //         label.set_markup(&format!("<span face=\"monospace\" background=\"{}\"></span>", 
        //                          colors[&at]));
        //         // label.set_border_width(2);
        //         // label.set_size_request(50, 25);
        //         let border = Border::default();
        //         border.add(&label);
        //         hbox.add(&border);
        //     }
        //     vbox.add(&hbox);
        // }
        let grid = gtk::Grid::new();
        grid.set_row_homogeneous(true);
        grid.set_column_homogeneous(true); 
        grid.set_row_spacing(2);
        grid.set_column_spacing(2);
        grid.set_border_width(1);     
        grid.set_hexpand(true);
        grid.set_vexpand(true);
        grid.set_halign(gtk::Align::Fill);
        grid.set_valign(gtk::Align::Fill);

        for row in 0..15 {
            for col in 0..15 {
                let label = Label::new(None);
                let at = model.at_position(Position { row, col });
                label.set_markup(&format!("<span face=\"monospace\" background=\"{}\"> </span>", 
                                 colors[&at]));

                grid.attach(&label, row as i32, col as i32, 1, 1);
                grid.set_cell_width(&label, 50);
                grid.set_cell_height(&label, 50);
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