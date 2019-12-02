use crate::board::{Board, S};
use crate::player::Player;
use crate::utils::Move;

use std::vec::Vec;

pub struct Game {
    players: [Player; 2],
    board: Board,
    pub current: usize,
    turn: u32,
    pub finished: bool,
    states: Vec<S>
}

impl Game {
    pub fn default() -> Game {
        let mut board = Board::default();
        let mut player_1 = Player { rack: board.bag.draw_tiles(7), name: "p1".to_string(), score: 0 };
        let mut player_2 = Player { rack: board.bag.draw_tiles(7), name: "p2".to_string(), score: 0 };
        let players = [player_1, player_2];

        Game { players, board, current: 0, turn: 1, finished: false, states: Vec::new() }
    }

    pub fn do_move(&mut self) -> (Move, String) {
        let m = self.players[self.current].do_move(&mut self.board, true);
        self.states.push(self.board.save_state());
        self.current = (self.current + 1) % 2;
        if self.current == 0 { self.turn += 1; }
        m
    }

    pub fn finish(&mut self) -> (String, i32, i32) {
        let mut n = 0;
        if self.get_player(1).rack.len() == 0 {
            n = 1;
        }

        let mut end = 0;
        let mut end_s = String::new();

        for s in self.get_player((n + 1) % 2).rack.clone() {
            end += self.board.bag.score(s);
            end_s.push(s);
        }

        end *= 2;
        let p = self.get_player(n);
        p.score += end as u32;

        self.finished = true;

        (end_s, end, n)
    }

    pub fn is_over(&self) -> bool {
        !(self.board.bag.distribution.len() > 0 || (self.players[0].rack.len() > 0 && self.players[1].rack.len() > 0))
    }

    pub fn get_board(&self) -> &Board { &self.board }
    pub fn get_turn(&self) -> u32 { self.turn }

    pub fn current_player(&self) -> &Player {
        &self.players[self.current]
    }

    pub fn get_player(&mut self, n: i32) -> &mut Player {
        &mut self.players[n as usize]
    }

    pub fn set_state(&mut self, to: usize) {
        self.board.set_state(self.states[to])
    }
}