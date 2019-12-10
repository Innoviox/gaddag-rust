#![allow(dead_code)]

#[macro_use] extern crate itertools;
#[macro_use] extern crate relm;
#[macro_use] extern crate clap;
extern crate gdk;

use clap::{Arg, App, SubCommand};
use std::env;

mod bag;
mod utils;
mod board;
mod dictionary;
mod player;
mod game;
mod text;
mod viz;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

//    let mut args = env::args();
    let typ = match args.nth(1) {
        Some(s) => s,
        None => String::from("viz")
    };

    if typ == "text" {
        text::main(10);
    } else if typ == "viz" {
        viz::main();
    } else {
        println!("Unknown type: {}, must be text or viz (default viz)", typ);
    }
}