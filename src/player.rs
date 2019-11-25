use crate::utils::ItemRemovable;
use crate::board::Board;
use crate::utils::Move;

pub struct Player {
    pub rack: Vec<char>,
}

impl Player {
    pub fn do_move(&mut self, board: &mut Board) -> (Move, String){
        let gen = board.gen_all_moves(&self.rack);
        let best_m = gen.iter().max_by(Move::cmp);

        if let Some(m) = best_m {
            let chars = board.reals(&m);
            let skips = board.format(&m, &self.rack);
            board.place_move(m);

            for c in chars {
                if self.rack.contains(&c) {
                    self.rack._remove_item(c);
                } else {
                    self.rack._remove_item('?');
                }
            }
            for c in board.bag.draw_tiles(7 - self.rack.len()) {
                self.rack.push(c);
            }

            return (Move::of(m), skips)
        }

        (Move::none(), String::new())
    }
}