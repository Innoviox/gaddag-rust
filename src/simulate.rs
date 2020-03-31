use crate::board::STATE;
use crate::game::Game;

pub fn main(rack: String) {
    let mut game = Game::default();

    let mut done = false;
    while !done {
        println!("input ready");
        let mut copy_state = STATE.clone();
        for i in 0..15 {
            let mut row = String::new();
            std::io::stdin().read_line(&mut row).unwrap();
            for (idx, c) in row.trim().chars().enumerate() {
                if idx < 15 && c != '.' {
                    copy_state[i][idx] = c.to_uppercase().nth(0).unwrap();
                }
            }
        }
        game.set_board(copy_state);
        println!("{}", game.get_board());
        done = true;
    }

    game.get_player_mut(0).set_rack(rack.chars().collect());

    let (m1, sm1, _, _nmoves1) = game.do_move();

    println!(
        "{}",
        format!(
            "{:<7}/{:<3}: {:<12} +{:<03}/{:<03}",
            rack,
            m1.position.to_str(m1.direction),
            sm1,
            m1.score,
            game.get_player(0).score,
        )
    );
    println!("{}", game.get_board());
}

/*
...............
...............
...............
...............
...............
.............v.
.............it
......toy..tear
.......halter.i
......cow..n..o
...........t..x
...........eh.i
......gan..de.d
.....zap....w.e
......bongoes..
*/