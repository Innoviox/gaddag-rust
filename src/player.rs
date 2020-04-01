use crate::board::Board;
use crate::utils::ItemRemovable;
use crate::utils::{Move, Type};

pub struct Player {
    pub rack: Vec<char>,
    pub name: String,
    pub score: u32,
}

impl Player {
    pub fn gen_moves(&self, board: &mut Board) -> (Vec<Move>, f32) {
        let mut gen = board.gen_all_moves(&self.rack);
        let eval_val = self.get_val(board.bag.distribution.len()); // todo implement if bag is empty, empty rack
        gen.sort_by(Move::cmp_with(1.0, eval_val));
        gen.reverse();

        (gen, eval_val)
    }

    /*
    Returns: the move object, move as a human-readable string, move as a gcg string, number of moves considered
    */
    pub fn do_move(&mut self, board: &mut Board) -> (Move, String, String, usize) {
        let moves = self.gen_moves(board).0;
        let len = moves.len();
        let best_m = moves.iter().nth(0);

        if let Some(m) = best_m {
            self.score += m.score as u32;
            match m.typ {
                Type::Play => {
                    let chars = board.reals(&m);
                    let s1 = board.format(&m, true);
                    let s2 = board.format(&m, false);
                    board.place_move(m);

                    for c in chars {
                        if self.rack.contains(&c) {
                            self.rack._remove_item(c);
                        } else {
                            self.rack._remove_item('?');
                        }
                    }

                    self.draw_up(board);

                    return (Move::of(m), s1.clone(), s2.clone(), len);
                }
                Type::Exch => {
                    let word = m.complement(&self.rack);
                    for c in &word {
                        self.rack._remove_item(*c);
                        board.bag.distribution.push(*c);
                    }

                    self.draw_up(board);

                    let mut nm = Move::of(m);
                    nm.word = word.iter().collect();

                    return (nm, String::new(), String::new(), len);
                }
            }
        }

        (Move::none(), String::new(), String::new(), len)
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
            return x;
        }
        0.0
    }

    pub fn set_rack(&mut self, rack: Vec<char>) {
        // for debugging
        self.rack = rack;
    }

    pub fn leave(&self, chars: Vec<char>) -> Vec<char> { // pass call of board.reals(&m)
        let mut rack = self.rack.clone();
        for c in chars {
            if rack.contains(&c) {
                rack._remove_item(c);
            } else {
                rack._remove_item('?');
            }
        }
        rack
    }

    pub fn clone(&self) -> Player {
        Player {
            rack: rack.clone(),
            name: name.clone(),
            score
        }
    }
}
