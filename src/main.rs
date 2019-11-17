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

    board.play_word(position, word, dir);

    println!("{}", board);
}