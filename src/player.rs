use crate::utils::ItemRemovable;
use crate::board::Board;
use crate::utils::{Move, Type};

use std::cmp;

pub struct Player {
    pub rack: Vec<char>,
    pub name: String
}

impl Player {
    pub fn do_move(&mut self, board: &mut Board, human: bool) -> (Move, String){
        let gen = board.gen_all_moves(&self.rack);
        let best_m = gen.iter().max_by(Move::cmp_with(1.0, self.get_val(board.bag.distribution.len())));

        if let Some(m) = best_m {
            match m.typ {
                Type::Play => {
                    let chars = board.reals(&m);
                    let skips = board.format(&m, human);
                    board.place_move(m);

                    for c in chars {
                        if self.rack.contains(&c) {
                            self.rack._remove_item(c);
                        } else {
                            self.rack._remove_item('?');
                        }
                    }

                    self.draw_up(board);

                    return (Move::of(m), skips)
                },
                Type::Exch => {
                    let word = m.complement(&self.rack);
                    for c in &word {
                        self.rack._remove_item(*c);
                        board.bag.distribution.push(*c);
                    }

                    self.draw_up(board);

                    let mut nm = Move::of(m);
                    nm.word = word.iter().collect();

                    return (nm, String::new())
                }
            }
        }

        (Move::none(), String::new())
    }

    fn draw_up(&mut self, board: &mut Board) {
        for c in board.bag.draw_tiles(7 - self.rack.len()) {
            self.rack.push(c);
        }
    }

    fn get_val(&self, len: usize) -> f32 {
        /*
        https://www.desmos.com/calculator/lkrdbcoiqt
        Essentially, the idea is for eval to be roughly 1 the whole game, 
        but be lower as the bag decreases and exchanging/longetivity becomes impossible.
        This method will have its primary effect to counteract blankholding.
        */
        let x = 1.0 - 1.0 / (4.0 * len as f32);
        if x > 0.0 {
            return x
        }
        0.0
    }
}