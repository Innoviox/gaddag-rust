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
use std::convert::TryInto;

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

#[derive(Msg, Debug)]
pub enum Msg {
    Tick,
    Quit,
    SetMove(usize)
}

struct Win {
    // necessary fields
    model: Game,
    window: Window,

    // ui fields
    board: Grid,
    moves: Grid,

    // internal fields
    last_move: Move,
    colors: HashMap::<char, RGBA>,
    relm: Relm<Win>
}

impl Win {
    fn get(&mut self, col: i32, row: i32) -> Label {
        self.board.get_child_at(col, row).unwrap().dynamic_cast::<Label>().ok().unwrap()
    }

    fn set(&mut self, p: Position, color: &str) {
        let mut at = self.model.get_board().at_position(p);
        let mut score = self.model.get_board().bag.score(at);
        let l = self.get(p.col as i32, p.row as i32);
        l.override_background_color(gtk::StateFlags::empty(), Some(&GREY));

        if self.model.get_board().blanks.contains(&p) { // blank
            at = (at as u32 + 127215).try_into().unwrap(); // make square character https://unicode.org/charts/nameslist/n_1F100.html
            score = 0;
        }

        l.set_markup(&format!("<span face=\"sans\" color=\"{}\">{}</span><span color=\"{0}\" face=\"sans\"><sub>{}</sub></span>", color, at, score));
    } 

    fn place(&mut self, m: &Move, color: &str) {
        let mut p = m.position.clone();
        for i in m.word.chars() {
            self.set(p, color);
            p.tick(m.direction);
        }
    }

    fn setup_board(&mut self, first: bool) {
        for row in 0..15 {
            for col in 0..15 {
                let p = Position { row, col };
                let mut at = self.model.get_board().at_position(p);
                if first {
                    let l = Label::new(Some(" "));
                    l.override_background_color(gtk::StateFlags::empty(), Some(&self.colors[&at]));
                    self.board.attach(&l, row as i32, col as i32, 1, 1);
                } else if "#^+-*.".contains(at) {
                    let l = self.get(p.col as i32, p.row as i32);
                    l.override_background_color(gtk::StateFlags::empty(), Some(&self.colors[&at]));
                    l.set_text(" ");
                } else {
                    self.set(p, "white");
                }
            }
        }

        self.window.show_all();
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
            Msg::Tick => {
                let c = self.model.current as i32;
                let t = self.model.get_turn() as i32;
                if !self.model.is_over() {
                    // why do i have to do this??? why cant i do
                    // self.place(&self.last_move...)? idk
                    let lm = Move::of(&self.last_move);
                    self.place(&lm, "white");

                    let p = self.model.current_player();
                    let rack: String = p.rack.iter().collect();
                    let score = p.score as i32;

                    let (m, sm) = self.model.do_move();

                    self.place(&m, "yellow");

                    self.last_move = Move::of(&m);

                    let text = format!("{:<7}/{:<3}: {:<12} +{:<03}/{:<03}",
                            rack, m.position.to_str(m.direction), sm, m.score, score + m.score);

                    let label = Label::new(Some(&text));
                    let n = (t * 2 + c - 1) as usize;
                    label.set_markup(&format!("<span face=\"monospace\">{}. {}</span>", n, text));
                    let btn = Button::new();
                    btn.add(&label);
                    connect!(self.relm, btn, connect_clicked(_), Msg::SetMove(n - 1));
                    self.moves.attach(&btn, c, t, 1, 1);                   
                } else if !self.model.finished {
                    let (end_s, end, n) = self.model.finish();
                    let text = format!("2*({}) +{}/{}", end_s, end, self.model.get_player(n).score);
                    let label = Label::new(Some(&text));
                    self.moves.attach(&label, n, t + 1, 1, 1);
                }
                self.window.show_all();
            },
            Msg::SetMove(n) => {
                if self.model.is_over() {
                    let (m, r) = self.model.set_state(n);
                    self.setup_board(false);
                    self.place(&m, "yellow");
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

        let moves = gtk::Grid::new();
        moves.set_row_spacing(10);
        moves.set_column_spacing(10);
        moves.set_border_width(3);

        let l1 = Label::new(Some("Player 1"));
        moves.attach(&l1, 0, 0, 1, 1);
        let l2 = Label::new(Some("Player 2"));
        moves.attach(&l2, 1, 0, 1, 1);

        let no_adjustment: Option<gtk::Adjustment> = None;
        let scroll: Option<gtk::Adjustment> = Some(gtk::Adjustment::new(0.0, std::f64::MIN, std::f64::MAX, 1.0, 0.0, 0.0));
        let moves_container = gtk::ScrolledWindow::new(no_adjustment.as_ref(), scroll.as_ref());
        moves_container.add(&moves);

        let grid = gtk::Grid::new();
        grid.set_hexpand(true);
        grid.set_vexpand(true);
        grid.set_row_homogeneous(true);
        grid.set_column_homogeneous(true); 
        grid.set_halign(gtk::Align::Fill);
        grid.set_valign(gtk::Align::Fill);

        grid.attach(&board, 0, 0, 1, 1);
        grid.attach(&moves_container, 1, 0, 1, 1);

        let window = Window::new(WindowType::Toplevel);
        window.add(&grid);
        window.set_default_size(2500, 2500);

        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));
        interval(relm.stream(), 100, || Msg::Tick);
        
        window.show_all();

        let mut win = Win {
            model,
            window,
            board,
            moves,
            last_move: Move::none(),
            colors,
            relm: relm.clone()
        };

        // win.update(Msg::Tick);
        win.setup_board(true);
        win
    }
}

fn main() {
    Win::run(()).unwrap();
}