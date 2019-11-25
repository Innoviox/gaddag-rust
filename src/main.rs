#[macro_use] extern crate itertools;
use crate::utils::ItemRemovable;
use crate::utils::Direction::{Across, Down};
use crate::utils::{Move, Position, Direction};
mod bag;
mod utils;
mod board;
mod dictionary;
use std::time::{SystemTime, UNIX_EPOCH};
use std::time::Duration;
use itertools::izip;

fn main() {
    // let mut b = bag::Bag::default();
    // println!("Score for z is: {}", bag.score('z'));

    // let mut dict = utils::Dictionary::default();
    // println!("This should be true: {}", dict.check_word("HELLO".to_string()));
    // println!("This should be false: {}", dict.check_word("REGOIJPREGOJ".to_string()));

    // let t = dictionary::Trie::default();
    // let d = dictionary::Dictionary::default();

    let mut board = board::Board::default();

    let position = utils::Position { row: 7, col: 7 };
    let word = String::from("HELLO");
    let dir = utils::Direction::Across;

    // board.play_word(position, word, dir, false);
    // board.play_word(utils::Position { row: 8, col: 7 }, 
    //                 String::from("AM"),
    //                 utils::Direction::Down, false);

    println!("{}", board);

    
    // println!("{:?}", utils::chars(board.valid_at(utils::Position { row: 6, col: 7 })));

    // println!("{:?}", board.anchors());

    // let rack = vec!['S', 'A', 'T', 'I', 'R', 'E', 'S'];

    
    // println!("{:?}", t.graph);
    // println!("{} {}", t.graph.node_count(), t.graph.edge_count());

    // t.seed(vec!['H']);

    // println!("{:?}", t.graph);

    // board.place(utils::Position{ row: 7, col: 7}, utils::Direction::Down, vec!['A', 'C'], vec!['B'], &utils::Dictionary::default());

    // println!("{}", board);

    let mut rack = board.bag.draw_tiles(7);
    let mut moves = Vec::<(Position, Direction, i32, f32)>::new();
    let mut skips = Vec::<String>::new();
    let mut times = Vec::<u128>::new();

    for i in 0..20 {
        // rack = vec!['N', 'L', 'U', 'U', 'O', 'D', 'A'];
        // rack = vec!['I', 'U', 'N', 'E', 'T', 'O', 'E'];
        println!("Rack is: {:?}", rack.clone());
        let start = SystemTime::now();
        let gen = board.gen_all_moves(&rack);
        let best_move = gen.iter().max_by(Move::cmp).unwrap();
        let time = start.elapsed().expect("Time went backwards").as_millis();

        println!("Best move: {:?} (skipped: {:?})\n{}", best_move, board.put_skips(best_move), board.place_move_cloned(&best_move));
        moves.push((best_move.position, best_move.direction, best_move.score, best_move.evaluation));
        skips.push(board.put_skips(best_move));
        times.push(time);
        let chars = board.reals(&best_move);

        board.place_move(best_move);
        for c in chars {
            rack._remove_item(c);
        }
        for c in board.bag.draw_tiles(7 - rack.len()) {
            rack.push(c);
        }
    } 

    println!("| {:^12} | {} | {} | {} | {} | {} |", "Move", "Position", "Direction", "Score", "Evaluation", "Time");
    println!("{}", "-".repeat(67));
    for (m, s, t) in izip!(moves.iter(), skips.iter(), times.iter()) {
        println!("| {:<12} | {:<8} | {:<9} | {:<5} | {:<10} | {:>4} |", s, m.0.to_str(), m.1.to_str(), m.2, m.3, t);
    }

    println!("{}", board);
    // for m in board.gen_all_moves(rack, &t, &d) {
    //     println!("{} {:?}", board.place_move_cloned(&m), m);
    // }
    // board.generate_all_moves(rack, &utils::Dictionary::default()).iter();

    // println!("{:?}", board.get_words());
    // println!("{:?}", board.valid());
}