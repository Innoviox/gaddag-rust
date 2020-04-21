use crate::game::Game;
use crate::utils::Type;
use std::time::SystemTime;

pub fn main(turns: u32) {
    let mut game = Game::default();
    for _ in 0..turns {
        game.do_move();
    }

    println!("{}", game.get_board());
}
