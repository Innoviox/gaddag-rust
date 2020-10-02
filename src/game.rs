use crate::bag::Bag;
use crate::board::{Board, S, STATE};
use crate::player::Player;
use crate::utils::Move;

use array_init::array_init;
use itertools::{EitherOrBoth::*, Itertools};

use std::cmp;
use std::vec::Vec;

pub struct Game {
    players: [Player; 2],
    board: Board,
    pub current: usize,
    turn: u32,
    pub finished: bool,
    states: Vec<(S, Move, Vec<char>)>,
    pub state: usize,
}

impl Game {
    pub fn default() -> Game {
        let mut board = Board::default();
        let player_1 = Player {
            rack: board.bag.draw_tiles(7),
            name: "p1".to_string(),
            score: 0,
        };
        let player_2 = Player {
            rack: board.bag.draw_tiles(7),
            name: "p2".to_string(),
            score: 0,
        };
        let players = [player_1, player_2];

        Game {
            players,
            board,
            current: 0,
            turn: 1,
            finished: false,
            states: Vec::new(),
            state: 0,
        }
    }

    pub fn set_board(&mut self, board: [[char; 15]; 15]) {
        // for simulation
        self.board.set_board(board);
    }

    pub fn do_move(&mut self, difficulty: usize) -> (Move, String, String, usize) {
        let r = self.current_player().rack.clone();
        let m = self.players[self.current].do_move(&mut self.board, difficulty);
        self.states
            .push((self.board.save_state(), Move::of(&m.0), r));
        self.current = (self.current + 1) % 2;
        if self.current == 0 {
            self.turn += 1;
        }
        self.state += 1;
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
        let p = &mut self.players[n as usize];
        if !self.finished {
            p.score += end as u32;
        }

        self.finished = true;

        (end_s, end, n)
    }

    pub fn is_over(&self) -> bool {
        self.finished
            || !(self.board.bag.distribution.len() > 0
                || (self.players[0].rack.len() > 0 && self.players[1].rack.len() > 0))
    }

    pub fn get_board(&self) -> &Board {
        &self.board
    }
    pub fn get_board_mut(&mut self) -> &mut Board {
        &mut self.board
    }
    pub fn get_turn(&self) -> u32 {
        self.turn
    }

    pub fn current_player(&self) -> &Player {
        &self.players[self.current]
    }

    pub fn get_player(&self, n: i32) -> &Player {
        &self.players[n as usize]
    }

    pub fn get_player_mut(&mut self, n: i32) -> &mut Player {
        &mut self.players[n as usize]
    }

    pub fn set_state(&mut self, to: usize) -> (Move, Vec<char>) {
        let (s, m, r) = &self.states[to];

        self.board.set_state(s);
        self.state = to;
        self.current = to % 2;

        (Move::of(m), r.clone())
    }

    pub fn get_rack(&self, n: usize) -> Vec<char> {
        self.states[n].2.clone()
    }

    pub fn get_last_state(&self) -> S {
        if self.state == 0 {
            return (
                STATE,
                vec![],
                [array_init(|_| Vec::new()), array_init(|_| Vec::new())],
                Bag::default().distribution,
            );
        }

        self.states[self.state - 1].0.clone()
    }

    pub fn reset(&mut self) {
        self.board.reset();
        for p in &mut self.players {
            p.score = 0;
            p.rack = self.board.bag.draw_tiles(7);
        }
        self.current = 0;
        self.turn = 1;
        self.finished = false;
        self.states = Vec::new();
        self.state = 0;
    }

    pub fn states(&self) -> usize {
        self.states.len()
    }

    fn states_str(&mut self) -> String {
        let mut res = format!(
            "{:^27}│{:^27}\n{}┼{}\n",
            self.get_player(0).name,
            self.get_player(1).name,
            "─".repeat(27),
            "─".repeat(27)
        );
        let mut scores = [0, 0];

        for i in 0..self.states() {
            let (m, _) = self.set_state(i);
            self.board.set_state(&self.get_last_state());

            scores[i % 2] += m.score;

            res = format!(
                "{}{:<3}: {:<12} +{:<03}/{:<03} ",
                res,
                m.position.to_str(m.direction),
                self.board.format(&m, true),
                m.score,
                scores[i % 2]
            );

            res = format!("{}{}", res, ["│ ", "\n"][i % 2]);
        }

        if self.is_over() {
            let (end_s, end, n) = self.finish();
            let mut text = format!("2*({}) +{}/{}\n", end_s, end, self.get_player(n).score);

            if self.states() % 2 == 0 {
                text = format!("{}│ {}", " ".repeat(27), text);
            } else {
                text = format!("\n{}", text);
            }

            res = format!("{}{}", res, text);
        }
        res
    }

    pub fn to_str(&mut self) -> String {
        let board = format!("{}", self.board);

        let board = board.split("\n");
        let res = res.split("\n");
        let mut out = format!("{}", "");

        for pair in board.zip_longest(res) {
            match pair {
                Both(l, r) => out = format!("{}{} {}\n", out, l, r),
                Left(l) => out = format!("{}{}\n", out, l),
                Right(r) => out = format!("{}{}{}\n", out, "-".repeat(66), r),
            }
        }

        out
    }
}
