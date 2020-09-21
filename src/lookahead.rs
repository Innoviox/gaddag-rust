use crate::board::{Board, S};
use crate::player::Player;
use crate::game::Game;
use std::{thread, time};

impl Board {
    pub fn lookahead(&mut self, n: u32, leave: &Vec<char>, pre_state: &S) -> (u32, u32) {
        /*
        Simulate the next n turns and return the diff in scores of the two players.

        To keep this knowledgeless, the opponent will start with a random rack.

        n: number of turns, set =-1 for "until game ends"
        leave: the part of the rack that is left over after the last play, before drawing.

        returns: (player score diff, opponent score diff)
        */

        let mut opponent = Player { rack: self.bag.draw_tiles(7), name: "opp".to_string(), score: 0 };
        let mut player   = Player { rack: leave.to_vec(),         name: "me!".to_string(), score: 0 };

        player.draw_up(self);

        let mut i = 0;
        while !Game::state_is_over(self, &opponent, &player) && i < n {
            opponent.do_move(self, 1, false);
            player.do_move(self, 1, false);
            // thread::sleep(time::Duration::from_secs(1));

            i += 1;
        }

        self.set_state(pre_state);

        (player.score, opponent.score)
    }

    pub fn simulate(&mut self, n: u32, leave: &Vec<char>, k: usize) -> (f64, f64) {
        /*
        Lookahead n turns, k times, and average the results
        */
        let pre_state: S = self.save_state();

        self.bag.remove_blanks(); // make things quicker

        let looks = vec![(0, 0); k].into_iter().map(|_| self.lookahead(n, leave, &pre_state));

        let sums = looks.into_iter().fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));

        let k = k as f64; // shadow k to do division

        (f64::from(sums.0) / k, f64::from(sums.1) / k)
    }
}