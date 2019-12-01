use crate::board::Board;
use crate::player::Player;
use crate::utils::Move;

pub struct Game {
    players: [Player; 2],
    board: Board,
    pub current: usize,
    turn: u32
}

impl Game {
    pub fn default() -> Game {
        let mut board = Board::default();
        let mut player_1 = Player { rack: board.bag.draw_tiles(7), name: "p1".to_string(), score: 0 };
        let mut player_2 = Player { rack: board.bag.draw_tiles(7), name: "p2".to_string(), score: 0 };
        let players = [player_1, player_2];

        Game { players, board, current: 0, turn: 1 }
    }

    pub fn do_move(&mut self) -> (Move, String) {
        let m = self.players[self.current].do_move(&mut self.board, true);
        self.current = (self.current + 1) % 2;
        if self.current == 0 { self.turn += 1; }
        m
    }

    pub fn is_over(&self) -> bool {
        !(self.board.bag.distribution.len() > 0 || (self.players[0].rack.len() > 0 && self.players[0].rack.len() > 0))
    }

    pub fn get_board(&self) -> &Board { &self.board }
    pub fn get_turn(&self) -> u32 { self.turn }

    pub fn current_player(&self) -> &Player {
        &self.players[self.current]
    }
}