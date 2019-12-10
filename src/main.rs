#![allow(dead_code)]

#[macro_use] extern crate itertools;
#[macro_use] extern crate relm;
#[macro_use] extern crate clap;
extern crate gdk;

use clap::App;

mod bag;
mod utils;
mod board;
mod dictionary;
mod player;
mod game;
mod text;
mod viz;

fn main() {
    let yaml = load_yaml!("../cmd.yml");
    let matches = App::from(yaml).get_matches();

    if let Some(ref matches) = matches.subcommand_matches("text") {
        if let Some(n) = matches.value_of("number") {
            text::main(n.parse::<u32>().unwrap());
        } else {
            text::main(1);
        }
    } else {
        viz::main();
    }
}