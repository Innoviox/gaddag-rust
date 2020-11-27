use crate::bag;
use crate::game::Game;
use crate::utils;
use std::time::SystemTime;

pub fn main() {
    let mut g = Game::with_distr(&vec![
        'S', 'D', 'L', 'A', 'N', 'L', 'A', 'U', 'E', 'M', 'S', 'R', 'A', 'C', 'Z', 'E', 'P', 'F',
        'T', 'I', 'R', 'O', 'E', 'N', 'F', 'O', 'O', 'Y', 'A', 'N', 'I', 'U', 'L', 'M', 'R', 'E',
        'B', 'E', 'A', 'U', 'B', 'A', 'T', 'I', '?', '?', 'V', 'N', 'E', 'A', 'G', 'T', 'O', 'O',
        'E', 'H', 'A', 'K', 'U', 'R', 'D', 'I', 'I', 'W', 'D', 'T', 'V', 'Y', 'N', 'I', 'E', 'Q',
        'J', 'S', 'D', 'L', 'E', 'R', 'O', 'E', 'X', 'A', 'I', 'H', 'W', 'O', 'I', 'C', 'P', 'T',
        'S', 'R', 'N', 'E', 'T', 'O', 'G', 'G', 'I', 'E',
    ]);

    for _ in 0..12 {
        g.do_move(1, false);
    }

    println!("{}", g.to_str());
}
