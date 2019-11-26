use crate::utils::ItemRemovable;
use crate::board::Board;
use crate::utils::{Move, Type};

pub struct Player {
    pub rack: Vec<char>,
    pub name: String
}

impl Player {
    pub fn do_move(&mut self, board: &mut Board, human: bool) -> (Move, String){
        let gen = board.gen_all_moves(&self.rack);
        let best_m = gen.iter().max_by(Move::cmp);

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
                    for c in word {
                        self.rack._remove_item(c);
                    }

                    self.draw_up(board);

                    let nm = Move::of(m);
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
}