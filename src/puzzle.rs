use crate::game::Game;

pub fn main(turns: u32) {
    let mut game = Game::default();
    for _ in 0..turns {
        game.do_move();
    }

    let rack: String = game.current_player().rack.iter().collect();

    let mut s = game.get_board().to_string();
    s.push_str("\n");
    s.push_str(&rack);

    println!("{}", s);
}
