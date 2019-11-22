#[macro_use] extern crate itertools;

mod bag;
mod utils;
mod board;
mod dictionary;

fn main() {
    let mut b = bag::Bag::default();
    // println!("Score for z is: {}", bag.score('z'));

    // let rack = bag.draw_tiles(7);
    // println!("Rack is: {:?}", rack);

    // let mut dict = utils::Dictionary::default();
    // println!("This should be true: {}", dict.check_word("HELLO".to_string()));
    // println!("This should be false: {}", dict.check_word("REGOIJPREGOJ".to_string()));

    let t = dictionary::Trie::default();
    let d = dictionary::Dictionary::default();

    let mut board = board::Board::default();

    let position = utils::Position { row: 7, col: 7 };
    let word = String::from("HELLO");
    let dir = utils::Direction::Across;

    board.play_word(position, word, dir, false);
    // board.play_word(utils::Position { row: 8, col: 8 }, 
    //                 String::from("HI"),
    //                 utils::Direction::Across, false);

    println!("{}", board);

    // println!("{:?}", utils::chars(board.valid_at(utils::Position { row: 6, col: 7 })));

    // println!("{:?}", board.anchors());

    let rack = vec!['A', 'B', 'C''];

    
    // println!("{:?}", t.graph);
    // println!("{} {}", t.graph.node_count(), t.graph.edge_count());

    // t.seed(vec!['H']);

    // println!("{:?}", t.graph);

    // board.place(utils::Position{ row: 7, col: 7}, utils::Direction::Down, vec!['A', 'C'], vec!['B'], &utils::Dictionary::default());

    // println!("{}", board);


    println!("{:?}", board.gen_all_moves(rack, &t, &d, &b).len());
    // for m in board.gen_all_moves(rack, &t, &d) {
    //     println!("{} {:?}", board.place_move_cloned(&m), m);
    // }
    // board.generate_all_moves(rack, &utils::Dictionary::default()).iter();

    // println!("{:?}", board.get_words());
    // println!("{:?}", board.valid());
}