use crate::board::{Board, S};
use crate::player::Player;
use crate::game::Game;

impl Board {
    pub fn lookahead(&mut self, n: u32, leave: &Vec<char>) -> (u32, u32) {
        /*
        Simulate the next n turns and return the diff in scores of the two players.

        To keep this knowledgeless, the opponent will start with a random rack.

        n: number of turns, set =-1 for "until game ends"
        leave: the part of the rack that is left over after the last play, before drawing.

        returns: (player score diff, opponent score diff)
        */

        let pre_state: &S = &self.save_state();

        let mut opponent = Player { rack: vec!['S', 'O', 'V', 'L', 'E', 'L', 'A'], name: "opp".to_string(), score: 0 };
        let mut player   = Player { rack: leave.to_vec(),         name: "me!".to_string(), score: 0 };

        player.draw_up(self);

        let mut i = 0;
        while !Game::state_is_over(self, &opponent, &player) && i < n {
            let rack: String = opponent.rack.clone().iter().collect();
            let mo = opponent.do_move(self, 1, false).0;
            // if mo.exch() {
                // opponent.score += mo.evaluation as u32;
            // }

            let rack2: String = player.rack.clone().iter().collect();
            let mp = player.do_move(self, 1, false).0;
            // if mp.exch() {
                // player.score += mp.evaluation as u32;
            // }

            i += 1;
        }

        self.set_state(pre_state);

        // todo do a unit test with this
        // println!("{:?}", (player.score, opponent.score));

        (player.score, opponent.score)
    }

    pub fn simulate(&mut self, leave: &Vec<char>, n: u32, k: usize) -> (f64, f64) {
        /*
        Lookahead n turns, k times, and average the results
        */
        self.bag.remove_blanks(); // make things quicker

        let sums = vec![(0, 0); k].into_iter()
                                   .map(|_| self.lookahead(n, leave))
                                   .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

        let k = k as f64; // shadow k to do division

        (f64::from(sums.0) / k, f64::from(sums.1) / k)
    }
}

pub fn l_score(value: (f64, f64)) -> f64 {
    (value.0 - value.1) / 35f64
}

pub fn main() {
    let mut game = Game::default();

    game.get_player_mut(0).set_rack(vec!['A', 'B', 'C', 'D', 'E', 'F', 'G']);
    // game.get_player_mut(1).set_rack(vec!['S', 'O', 'V', 'L', 'E', 'L', 'A']);

    let pre_state = game.get_last_state(); // preserve an empty state
    let player = game.current_player().clone();
    let board = game.get_board_mut();

    let moves = player.gen_moves(board).0;
    // let post_state = game.get_last_state();

    for m in moves.iter().take(1) {
        board.set_state(&pre_state);
        let leave = &player.leave(board.reals(&m));

        board.place_move(&m);

        println!("{:?} {:?}", m.word, l_score(board.simulate(leave, 1, 2)));
    }
}