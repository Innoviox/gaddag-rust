use crate::game::Game;
use base64::encode;

pub fn main(turns: u32, difficulty: usize) {
    let mut game = Game::default();
    for _ in 0..turns {
        game.do_move(difficulty);
    }

    let p = game.get_current_player().clone();
    let rack: String = p.rack.iter().collect();

    let (moves, eval_val) = p.gen_moves(game.get_board_mut());

    let mut board = game.get_board().get_board().clone();
    for b in game.get_board().blanks.clone() {
        board[b.row][b.col] = board[b.row][b.col].to_lowercase().nth(0).unwrap();
    }
    let mut s = board
        .iter()
        .map(|i| i.iter().collect::<String>())
        .collect::<String>();
    s = format!("{}{}{}{}", s, "\n", rack, "\n");

    for m in moves.iter().take(500) {
        s = format!(
            "{}{}",
            s,
            format!(
                "{} {} {} {}\n",
                m.position.to_str(m.direction),
                game.get_board_mut().format(&m, true),
                m.score,
                m.eval(1.0, eval_val)
            )
        );
    }

    println!("{}", encode(s));
}
