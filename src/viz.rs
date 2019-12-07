use crate::player::Player;
use crate::utils::{Position, Move, to_word, alph};
use crate::board::{Board, STATE};
use crate::game::Game;
use std::time::SystemTime;
use std::collections::HashMap;
use std::convert::TryInto;

use relm_derive::Msg;
use relm::{Widget, Relm, Update, interval, timeout};
use gtk::prelude::*;
use gtk::{Inhibit, Window, WindowType};
use gtk::Orientation::{Vertical, Horizontal};
use gtk::{
    Label, Grid, Button, ScrolledWindow, Viewport, DrawingArea
};
use gdk::RGBA;
use itertools::Itertools;
use std::cmp::max;


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
    rack: Grid,
    graph: DrawingArea,

    // internal fields
    last_move: Move,
    colors: HashMap::<char, RGBA>,
    back_colors: HashMap::<char, RGBA>,
    relm: Relm<Win>
}

impl Win {
    fn get(&mut self, col: i32, row: i32) -> Label {
        self.board.get_child_at(col, row).unwrap().dynamic_cast::<Label>().ok().unwrap()
    }

    fn lset(&mut self, l: Label, c: &str, a: char, s: i32, b: &RGBA) {
        l.override_background_color(gtk::StateFlags::empty(), Some(b));
        let mut st = s.to_string();
        if s == -1 {
            st = "".to_string();
        }
        l.set_markup(&format!("<span face=\"sans\" color=\"{}\">{}</span><span color=\"{0}\" face=\"sans\"><sub>{}</sub></span>", c, a, st));
    }

    fn set(&mut self, p: Position, color: &str) {
        let mut at = self.model.get_board().at_position(p);
        let mut score = self.model.get_board().bag.score(at);
        let l = self.get(p.col as i32, p.row as i32);
        if self.model.get_board().blanks.contains(&p) { // blank
            at = (at as u32 + 127215).try_into().unwrap(); // make square character https://unicode.org/charts/nameslist/n_1F100.html
            score = 0;
        }
        let b = self.back_colors[&STATE[p.row][p.col]];
        self.lset(l, color, at, score, &b);
    } 

    fn place(&mut self, m: &Move, color: &str, force: bool) {
        let mut p = m.position.clone();
        let last = self.model.get_last_state();
        for i in m.word.chars() {
            if force || "#^+-*.".contains(last.0[p.row][p.col]) {
                self.set(p, color);
            }
            p.tick(m.direction);
        }
    }

    fn setup_board(&mut self, first: bool) {
        for row in 0..15 {
            for col in 0..15 {
                let p = Position { row, col };
                let at = self.model.get_board().at_position(p);
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
        self.update(Msg::Tick);
    }

    fn _update_rack(&mut self, r: &Vec<char>) {
        for i in 0..r.len() {
            let l = self.rack.get_child_at(i as i32, 0).unwrap().dynamic_cast::<Label>().ok().unwrap();
            let a = r[i as usize];
            let s = self.model.get_board().bag.score(a);
            self.lset(l, "white", a, s, &GREY);
        }
        for i in r.len()..7 {
            let l = self.rack.get_child_at(i as i32, 0).unwrap().dynamic_cast::<Label>().ok().unwrap();
            self.lset(l, "white", ' ', -1, &GREY);
        }
    }

    fn update_rack(&mut self) {
        self._update_rack(&self.model.current_player().rack.clone());
    }

    fn update_rack_for(&mut self, m: &Move) {
        let mut word = to_word(&m.word.chars().collect());

        for w in self.rack.get_children() {
            let l = w.dynamic_cast::<Label>().ok().unwrap();
            let c = l.get_text().unwrap().chars().nth(0).unwrap();
            if let Some(i) = alph.find(c) {
                if word[i] > 0 {
                    word[i] -= 1;
                    let s = self.model.get_board().bag.score(c);
                    self.lset(l, "yellow", c, s, &GREY);
                }
            }
        }
    }

    fn _handle(&mut self, m: &Move) {
        if !m.exch() {
            self.place(&m, "yellow", false);
        } else { // todo do it for placements to
            self.update_rack_for(&m);
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
            Msg::Tick => {
                let c = self.model.current as i32;
                let t = self.model.get_turn() as i32;
                if !self.model.is_over() {
                    self.update_rack();
                    // why do i have to do this??? why cant i do
                    // self.place(&self.last_move...)? idk
                    let lm = Move::of(&self.last_move);
                    if !lm.exch() { self.place(&lm, "white", true); }

                    let p = self.model.current_player();
                    let rack: String = p.rack.iter().collect();
                    let score = p.score as i32;

                    let (m, sm, partials) = self.model.do_move(true);
                    self.model.state -= 1; // dont know why this is necessary
                    self._handle(&m);
                    self.model.state += 1;
                    self.last_move = Move::of(&m);

                    let mut text = format!("{:<7}/{:<3}: {:<12} +{:<03}/{:<03}",
                            rack, m.position.to_str(m.direction), sm, m.score, score + m.score);
                    if m.exch() {
                        text = format!("{:<7}/EXC: -{:<11} +{:<03}/{:<03}",
                            rack, m.word, m.score, score + m.score);
                    } 

                    let label = Label::new(Some(&text));
                    let n = (t * 2 + c - 1) as usize;
                    if c == 0 {
                        label.set_markup(&format!("<span face=\"monospace\">{}. {}</span>", t, text));
                    } else {
                        label.set_markup(&format!("<span face=\"monospace\">{}</span>", text));
                    }
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
                self.graph.queue_draw();
                timeout(self.relm.stream(), 1, || Msg::Tick);
            },
            Msg::SetMove(n) => {
                if self.model.is_over() {
                    let (m, r) = self.model.set_state(n);
                    self.setup_board(false);
                    self._update_rack(&r.clone());
                    self._handle(&m);
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

        // colors for back, greyed out
        let mut back_colors = HashMap::<char, RGBA>::new();
        back_colors.insert('#', RGBA { red: 0.66, green: 0.20, blue: 0.20, alpha: 1.0} );
        back_colors.insert('.', RGBA { red: 0.38, green: 0.38, blue: 0.38, alpha: 1.0} );
        back_colors.insert('-', RGBA { red: 0.27, green: 0.50, blue: 0.52, alpha: 1.0} );
        back_colors.insert('^', RGBA { red: 0.71, green: 0.35, blue: 0.35, alpha: 1.0} );
        back_colors.insert('*', RGBA { red: 0.71, green: 0.35, blue: 0.35, alpha: 1.0} );
        back_colors.insert('+', RGBA { red: 0.25, green: 0.32, blue: 0.53, alpha: 1.0} );

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
        let moves_container = ScrolledWindow::new(no_adjustment.as_ref(), scroll.as_ref());
        moves_container.add(&moves);

        let rack = gtk::Grid::new();
        rack.set_hexpand(true); // todo make fn to generate grid
        rack.set_vexpand(true);
        rack.set_row_homogeneous(true);
        rack.set_column_homogeneous(true);
        rack.set_halign(gtk::Align::Fill);
        rack.set_border_width(5);
        for i in 0..7 {
            let l = Label::new(Some(" "));
            l.override_background_color(gtk::StateFlags::empty(), Some(&GREY));
            rack.attach(&l, i, 0, 1, 1);
        }

        let graph = gtk::DrawingArea::new();
        graph.connect_draw(move |widget,cr| {
            let children = widget
                // get parent as grid
                .get_parent().unwrap().dynamic_cast::<Grid>().ok().unwrap()
                // get moves window
                .get_children().iter().filter(|x| x.is::<ScrolledWindow>()).nth(0).unwrap().clone().dynamic_cast::<ScrolledWindow>().ok().unwrap()
                // get actual moves object
                .get_children()[0].clone().dynamic_cast::<Viewport>().ok().unwrap().get_children()[0].clone().dynamic_cast::<Grid>().ok().unwrap()
                // get children of moves object
                .get_children();
            let (s1, s2): (Vec<(usize, i32)>, Vec<(usize, i32)>) = children.iter()
                                    // get buttons, which contain the moves
                                    .map(|x| x.clone().dynamic_cast::<Button>()).filter(|x| match x { Ok(_) => true, _ => false })
                                    .map(|x| x
                                        // get text on label
                                        .ok().unwrap().get_children()[0].clone().dynamic_cast::<Label>().ok().unwrap().get_text().unwrap()
                                        // get score from text
                                        .split("+").nth(1).unwrap().split("/").nth(1).unwrap().parse::<i32>().unwrap())
                                    // split into player 1 and player 2
                                    .rev().enumerate().collect::<Vec<(usize, i32)>>().iter().partition(|(i, n)| i % 2 == 0);

            // remove partition artifacts
            let mut s1: Vec<i32> = s1.iter().map(|x| x.1).collect();
            let mut s2: Vec<i32> = s2.iter().map(|x| x.1).collect();

            let end = match children.iter() // match because end may not exist (e.g. game may not be over)
                // get all labels
                .map(|x| x.clone().dynamic_cast::<Label>()).filter(|x| match x { Ok(_) => true, _ => false })
                // filter for labels with "/" (there should only be one, the one we want)
                .map(|x| x.ok().unwrap().get_text().unwrap()).filter(|x| x.contains("/"))
                // take label we found
                .nth(0) {
                    // extract score
                    Some(s) => s.split("/").nth(1).unwrap().parse::<i32>().unwrap(),
                    _ => 0
            };

            // add end score to correct array
            if end != 0 { if s1.len() > s2.len() { s1.push(end); } else { s2.push(end); } }

            // make same length (fill with last value)
            for i in 0..2 {
                if s1.len() < s2.len() {
                    if let Some(l) = s1.last() { s1.push(*l); }
                } else if s1.len() > s2.len() {
                    if let Some(l) = s2.last() { s2.push(*l); }
                }
            }

            let top = max(s1.iter().max(), s2.iter().max()).unwrap() + 10;

            let width: f64 = widget.get_allocated_width() as f64;
            let height: f64 = widget.get_allocated_height() as f64;
            let m = height / (top as f64);

            cr.rectangle(0.0,0.0, width, height);
            cr.set_source_rgb(1.0,1.0, 1.0);
            cr.fill();

            cr.set_line_width(1.0);

            let draw = |list: Vec<i32>| {
                cr.move_to(0.0, height);
                let dx = width / (s1.len() as f64);
                for (i, n) in list.iter().enumerate() {
                    cr.line_to(dx * ((i + 1) as f64), height - m * (*n as f64));
                }
                cr.stroke();
            };

            cr.set_source_rgb(1.0,0.0, 0.0);
            draw(s1.clone());


            cr.set_source_rgb(0.0, 0.0, 1.0);
            draw(s2.clone());

            Inhibit(false)
        });

        let grid = gtk::Grid::new();
        grid.set_hexpand(true);
        grid.set_vexpand(true);
        grid.set_row_homogeneous(true);
        grid.set_column_homogeneous(true); 
        grid.set_halign(gtk::Align::Fill);
        grid.set_valign(gtk::Align::Fill);

        grid.attach(&board, 0, 0, 13, 15);
        grid.attach(&moves_container, 13, 0, 10, 10);
        grid.attach(&rack, 4, 16, 7, 1);
        grid.attach(&graph, 13, 10, 10, 5);

        let window = Window::new(WindowType::Toplevel);
        window.add(&grid);
        window.set_default_size(1280, 800);

        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));
        
        window.show_all();

        let mut win = Win {
            model,
            window,
            board,
            moves,
            rack,
            graph,
            last_move: Move::none(),
            colors,
            back_colors,
            relm: relm.clone()
        };
        win.setup_board(true);
        win
    }
}

pub fn main() {
    Win::run(()).unwrap();
}