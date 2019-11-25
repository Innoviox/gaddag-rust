#[macro_use] extern crate itertools;
use crate::utils::ItemRemovable;
use crate::utils::Direction::{Across, Down};
use crate::utils::{Move, Position};
mod bag;
mod utils;
mod board;
mod dictionary;

fn main() {
    let mut b = bag::Bag::default();
    // println!("Score for z is: {}", bag.score('z'));

    // let mut dict = utils::Dictionary::default();
    // println!("This should be true: {}", dict.check_word("HELLO".to_string()));
    // println!("This should be false: {}", dict.check_word("REGOIJPREGOJ".to_string()));

    let t = dictionary::Trie::default();
    let d = dictionary::Dictionary::default();

    let mut board = board::Board::default();

    let position = utils::Position { row: 7, col: 7 };
    let word = String::from("HELLO");
    let dir = utils::Direction::Across;

    // board.play_word(position, word, dir, false);
    // board.play_word(utils::Position { row: 8, col: 7 }, 
    //                 String::from("AM"),
    //                 utils::Direction::Down, false);


    // board.place_move(&Move { word: "FJORD".to_string(), position: Position { row: 3, col: 7 }, direction: Down, score: 40 } );
    // board.place_move(&Move { word: "JINX".to_string(), position: Position { row: 4, col: 7 }, direction: Across, score: 36 } );
    // board.place_move(&Move { word: "ZOO".to_string(), position: Position { row: 5, col: 5 }, direction: Across, score: 32 } );
    // board.place_move(&Move { word: "WRY".to_string(), position: Position { row: 6, col: 6 }, direction: Across, score: 26 } );
    // board.place_move(&Move { word: "OHM".to_string(), position: Position { row: 3, col: 10 }, direction: Across, score: 25 } );
    // board.place_move(&Move { word: "DIG".to_string(), position: Position { row: 2, col: 8 }, direction: Across, score: 18 } );
    // board.place_move(&Move { word: "OD".to_string(), position: Position { row: 2, col: 12 }, direction: Across, score: 22 } );
    // board.place_move(&Move { word: "UTE".to_string(), position: Position { row: 1, col: 11 }, direction: Across, score: 23 } );
    // board.place_move(&Move { word: "TERTIAL".to_string(), position: Position { row: 1, col: 3 }, direction: Across, score: 68 } );
    // board.place_move(&Move { word: "EAR".to_string(), position: Position { row: 0, col: 6 }, direction: Across, score: 33 } );
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

    let mut rack = b.draw_tiles(7);

    for i in 0..20 {
        // rack = vec!['I', 'U', 'N', 'E', 'T', 'O', 'E'];
        println!("Rack is: {:?}", rack.clone());
        let moves = board.gen_all_moves(&rack, &t, &d, &b);
        let best_move = moves.iter().max_by(|x, y| x.score.cmp(&y.score)).unwrap();
        println!("Best move: {:?} \n{}", best_move, board.place_move_cloned(&best_move));

        let chars = board.reals(&best_move);

        board.place_move(best_move);
        for c in chars {
            rack._remove_item(c);
        }
        for c in b.draw_tiles(7 - rack.len()) {
            rack.push(c);
        }
    } 
    // for m in board.gen_all_moves(rack, &t, &d) {
    //     println!("{} {:?}", board.place_move_cloned(&m), m);
    // }
    // board.generate_all_moves(rack, &utils::Dictionary::default()).iter();

    // println!("{:?}", board.get_words());
    // println!("{:?}", board.valid());
}