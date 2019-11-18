#[macro_use] extern crate itertools;
use cpuprofiler::PROFILER;

mod bag;
mod utils;
mod board;

fn main() {
    // let mut bag = bag::Bag::default();
    // println!("Score for z is: {}", bag.score('z'));

    // let rack = bag.draw_tiles(7);
    // println!("Rack is: {:?}", rack);

    // let mut dict = utils::Dictionary::default();
    // println!("This should be true: {}", dict.check_word("HELLO".to_string()));
    // println!("This should be false: {}", dict.check_word("REGOIJPREGOJ".to_string()));

    let mut board = board::Board::default();

    let position = utils::Position { row: 7, col: 7 };
    let word = String::from("HELLO");
    let dir = utils::Direction::Across;

    board.play_word(position, word, dir, false);
    // board.play_word(utils::Position { row: 8, col: 8 }, 
    //                 String::from("WORLD"),
    //                 utils::Direction::Across);

    println!("{}", board);

    // println!("{:?}", utils::chars(board.valid_at(utils::Position { row: 6, col: 7 })));

    // println!("{:?}", board.anchors());

    let rack = vec!['A', 'B', 'C'];

    // board.place(utils::Position{ row: 7, col: 7}, utils::Direction::Down, vec!['A', 'C'], vec!['B'], &utils::Dictionary::default());

    // println!("{}", board);

    PROFILER.lock().unwrap().start("./my-prof.profile").expect("Couldn't start");

    board.generate_all_moves(rack, &utils::Dictionary::default()).iter();

    PROFILER.lock().unwrap().stop().expect("Couldn't stop");
    // println!("{:?}", board.get_words());
    // println!("{:?}", board.valid());
}