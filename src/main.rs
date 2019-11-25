#[macro_use] extern crate itertools;
use crate::utils::ItemRemovable;
use crate::utils::Direction::{Across, Down};
use crate::utils::{Move, Position, Direction};
use crate::player::Player;
mod bag;
mod utils;
mod board;
mod dictionary;
mod player;
use std::time::{SystemTime, UNIX_EPOCH};
use std::time::Duration;
use itertools::izip;

fn two_player_game() {
    let mut board = board::Board::default();
    println!("{}", board);


    let mut player_1 = Player { rack: board.bag.draw_tiles(7) };
    let mut player_2 = Player { rack: board.bag.draw_tiles(7) };

    let mut score_1 = 0;
    let mut score_2 = 0;

    let mut out = String::new();

    let mut turn = 1;

    while board.bag.distribution.len() > 0 || (player_1.rack.len() > 0 || player_2.rack.len() > 0) {
        let rack_1: String = player_1.rack.iter().collect();
        let (m1, sm1) = player_1.do_move(&mut board);
        
        if sm1 == String::new() {
            break
        }

        score_1 += m1.score;

        let rack_2: String = player_2.rack.iter().collect();
        let (m2, sm2) = player_2.do_move(&mut board);

        if sm2 == String::new() {
            break
        }

        score_2 += m2.score;

        out = format!("{}\n{:<02}. {:<7}/{:<3}: {:<12} +{:<03}/{:<03} | {:<7}/{:<3}: {:<12} +{:<03}/{:<03}", out, turn, 
                        rack_1, m1.position.to_str(), sm1, m1.score, score_1,
                        rack_2, m2.position.to_str(), sm2, m2.score, score_2);
        turn += 1;

        println!("{}", out);
    }

    out = format!("{}\n{}", out, board);
    println!("{}", out);
}

fn main() {
    // let mut b = bag::Bag::default();
    // println!("Score for z is: {}", bag.score('z'));

    // let mut dict = utils::Dictionary::default();
    // println!("This should be true: {}", dict.check_word("HELLO".to_string()));
    // println!("This should be false: {}", dict.check_word("REGOIJPREGOJ".to_string()));

    // let t = dictionary::Trie::default();
    // let d = dictionary::Dictionary::default();

    let mut board = board::Board::default();

    board.place_move( &Move { word: "REWAX".to_string(), 
                              position: Position { row: 7, col: 7 },
                              direction: Direction::Down,
                              score: 0, evaluation: 0.0 } );

    println!("{}", board);

    let mut player_1 = Player { rack: vec!['B' ,'?', 'L', 'E', 'U', 'I', 'N'] };

    let (m1, sm1) = player_1.do_move(&mut board);

    println!("{:?} {}", m1, sm1);

    // let position = utils::Position { row: 7, col: 7 };
    // let word = String::from("HELLO");
    // let dir = utils::Direction::Across;

    // // board.play_word(position, word, dir, false);
    // // board.play_word(utils::Position { row: 8, col: 7 }, 
    // //                 String::from("AM"),
    // //                 utils::Direction::Down, false);

    

    
    // // println!("{:?}", utils::chars(board.valid_at(utils::Position { row: 6, col: 7 })));

    // // println!("{:?}", board.anchors());

    // // let rack = vec!['S', 'A', 'T', 'I', 'R', 'E', 'S'];

    
    // // println!("{:?}", t.graph);
    // // println!("{} {}", t.graph.node_count(), t.graph.edge_count());

    // // t.seed(vec!['H']);

    // // println!("{:?}", t.graph);

    // // board.place(utils::Position{ row: 7, col: 7}, utils::Direction::Down, vec!['A', 'C'], vec!['B'], &utils::Dictionary::default());

    // // println!("{}", board);

    // let mut rack = board.bag.draw_tiles(7);
    // let mut moves = Vec::<(Position, Direction, i32, f32)>::new();
    // let mut skips = Vec::<String>::new();
    // let mut times = Vec::<u128>::new();

    // for i in 0..20 {
    //     // rack = vec!['N', 'L', 'U', 'U', 'O', 'D', 'A'];
    //     // rack = vec!['I', 'U', 'N', 'E', 'T', 'O', 'E'];
    //     println!("Rack is: {:?}", rack.clone());
    //     let start = SystemTime::now();
    //     let gen = board.gen_all_moves(&rack);
    //     let best_move = gen.iter().max_by(Move::cmp).unwrap();
    //     let time = start.elapsed().expect("Time went backwards").as_millis();

    //     println!("Best move: {:?} (skipped: {:?})\n{}", best_move, board.put_skips(best_move), board.place_move_cloned(&best_move));
    //     moves.push((best_move.position, best_move.direction, best_move.score, best_move.evaluation));
    //     skips.push(board.put_skips(best_move));
    //     times.push(time);
    //     let chars = board.reals(&best_move);

    //     board.place_move(best_move);
    //     for c in chars {
    //         rack._remove_item(c);
    //     }
    //     for c in board.bag.draw_tiles(7 - rack.len()) {
    //         rack.push(c);
    //     }
    // } 

    // println!("| {:^12} | {} | {} | {} | {} | {} |", "Move", "Position", "Direction", "Score", "Evaluation", "Time");
    // println!("{}", "-".repeat(67));
    // for (m, s, t) in izip!(moves.iter(), skips.iter(), times.iter()) {
    //     println!("| {:<12} | {:<8} | {:<9} | {:<5} | {:<10} | {:>4} |", s, m.0.to_str(), m.1.to_str(), m.2, m.3, t);
    // }

    // println!("{}", board);
    // for m in board.gen_all_moves(rack, &t, &d) {
    //     println!("{} {:?}", board.place_move_cloned(&m), m);
    // }
    // board.generate_all_moves(rack, &utils::Dictionary::default()).iter();

    // println!("{:?}", board.get_words());
    // println!("{:?}", board.valid());
}