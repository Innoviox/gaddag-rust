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


    board.place_move( &Move { word: "WATER".to_string(), position: Position { row: 3, col: 7 }, direction: Down, score: 24 }  );
board.place_move( &Move { word: "FOULARDS".to_string(), position: Position { row: 4, col: 3 }, direction: Across, score: 98 }  );
board.place_move( &Move { word: "JAVELINA".to_string(), position: Position { row: 6, col: 4 }, direction: Across, score: 73 }  );
board.place_move( &Move { word: "JURANT".to_string(), position: Position { row: 6, col: 4 }, direction: Down, score: 26 }  );
board.place_move( &Move { word: "BATED".to_string(), position: Position { row: 3, col: 0 }, direction: Across, score: 45 }  );
board.place_move( &Move { word: "YAIRD".to_string(), position: Position { row: 9, col: 5 }, direction: Down, score: 38 }  );
board.place_move( &Move { word: "YEZ".to_string(), position: Position { row: 5, col: 11 }, direction: Across, score: 40 }  );
board.place_move( &Move { word: "ILEX".to_string(), position: Position { row: 6, col: 9 }, direction: Down, score: 27 }  );
board.place_move( &Move { word: "QIS".to_string(), position: Position { row: 5, col: 1 }, direction: Across, score: 38 }  );
board.place_move( &Move { word: "VOES".to_string(), position: Position { row: 3, col: 12 }, direction: Down, score: 28 }  );
board.place_move( &Move { word: "WEET".to_string(), position: Position { row: 8, col: 8 }, direction: Down, score: 29 }  );
board.place_move( &Move { word: "SON".to_string(), position: Position { row: 4, col: 10 }, direction: Down, score: 19 }  );
board.place_move( &Move { word: "LEG".to_string(), position: Position { row: 7, col: 9 }, direction: Across, score: 19 }  );
board.place_move( &Move { word: "IONIUM".to_string(), position: Position { row: 7, col: 0 }, direction: Across, score: 31 }  );
board.place_move( &Move { word: "HER".to_string(), position: Position { row: 2, col: 1 }, direction: Across, score: 32 }  );
board.place_move( &Move { word: "OS".to_string(), position: Position { row: 1, col: 0 }, direction: Across, score: 28 }  );
// board.place_move( &Move { word: "BI".to_string(), position: Position { row: 2, col: 0 }, direction: Down, score: 13 }  );
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

    // for i in 0..20 {
        rack = vec!['I', 'I', 'O', 'I', 'T', 'O', 'I'];
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
    // } 
    // for m in board.gen_all_moves(rack, &t, &d) {
    //     println!("{} {:?}", board.place_move_cloned(&m), m);
    // }
    // board.generate_all_moves(rack, &utils::Dictionary::default()).iter();

    // println!("{:?}", board.get_words());
    // println!("{:?}", board.valid());
}