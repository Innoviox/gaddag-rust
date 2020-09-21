#![allow(dead_code)]

#[macro_use]
extern crate itertools;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate clap;
extern crate gdk;

use clap::App;

mod bag;
mod board;
mod dictionary;
mod game;
mod player;
mod puzzle;
mod simulate;
mod text;
mod utils;
mod viz;

fn main() {
    let yaml = load_yaml!("../cmd.yml");
    let matches = App::from(yaml).get_matches();

    if let Some(ref matches) = matches.subcommand_matches("text") {
        text::main(matches.value_of("number").unwrap().parse::<u32>().unwrap());
    } else if let Some(ref _matches) = matches.subcommand_matches("viz") {
        viz::main();
    } else if let Some(ref matches) = matches.subcommand_matches("simulate") {
        simulate::main(matches.value_of("rack").unwrap().to_string());
    } else if let Some(ref matches) = matches.subcommand_matches("puzzle") {
        puzzle::main(
            matches.value_of("turns").unwrap().parse::<u32>().unwrap(),
            matches
                .value_of("difficulty")
                .unwrap()
                .parse::<usize>()
                .unwrap(),
        );
    }
}
