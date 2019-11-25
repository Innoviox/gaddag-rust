use crate::utils::ItemRemovable;
use crate::board::Board;
use crate::utils::Move;

pub struct Player {
    pub rack: Vec<char>,
}

impl Player {
    pub fn do_move(&mut self, board: &mut Board) -> (Move, String) {
        let gen = board.gen_all_moves(&self.rack);
        let m = gen.iter().max_by(Move::cmp).unwrap();
        let chars = board.reals(&m);
        let skips = board.put_skips(&m);
        board.place_move(m);

        for c in chars {
            self.rack._remove_item(c);
        }
        for c in board.bag.draw_tiles(7 - self.rack.len()) {
            self.rack.push(c);
        }

        (Move::of(m), skips)
    }
}