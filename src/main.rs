#[macro_use] extern crate itertools;
use crate::player::Player;
use crate::utils::Type;
use std::time::SystemTime;

mod bag;
mod utils;
mod board;
mod dictionary;
mod player;

fn two_player_game(gcg: bool) {
    let mut board = board::Board::default();
    // board.bag = bag::Bag::with(&vec!['S', 'D', 'L', 'A', 'N', '?', 'A', 'U', 'E', 'M', 'S', 'R', 'A', 'C', 'Z', 'E', 'P', 'F', 'T', 'I', 'R', 'O', 'E', 'N', 'F', 'O', 'O', 'Y', 'A', 'N', 'I', 'U', 'L', 'M', 'R', 'E', 'B', 'E', 'A', 'U', 'B', 'A', 'T', 'I', 'L', 'W', 'V', 'N', 'E', 'A', 'G', 'T', 'O', 'O', 'E', 'H', 'A', 'K', 'U', 'R', 'D', 'I', 'I', '?', 'D', 'T', 'V', 'Y', 'N', 'I', 'E', 'Q', 'J', 'S', 'D', 'L', 'E', 'R', 'O', 'E', 'X', 'A', 'I', 'H', 'W', 'O', 'I', 'C', 'P', 'T', 'S', 'R', 'N', 'E', 'T', 'O', 'G', 'G', 'I', 'E']);

    let mut player_1 = Player { rack: board.bag.draw_tiles(7), name: "p1".to_string() };
    let mut player_2 = Player { rack: board.bag.draw_tiles(7), name: "p2".to_string() };

    let mut score_1 = 0;
    let mut score_2 = 0;

    let mut out = String::new();
    
    if gcg {
        out = format!("#character-encoding UTF-8\n#player1 {n1} {n1}\n#player2 {n2} {n2}",
                      n1=player_1.name, n2=player_2.name);
    }

    let mut turn = 1;

    while board.bag.distribution.len() > 0 || (player_1.rack.len() > 0 && player_2.rack.len() > 0) {
        let rack_1: String = player_1.rack.iter().collect();
        let start1 = SystemTime::now();
        let (m1, sm1) = player_1.do_move(&mut board, !gcg);
        let time1 = start1.elapsed().expect("Time went backwards").as_millis();
        
        if sm1 == String::new() && m1.typ == Type::Play {
            break
        }

        score_1 += m1.score;

        if gcg {
            out = match m1.typ {
                Type::Play => format!("{}\n>{}: {} {} {} +{} {}", out, player_1.name, rack_1, 
                                      m1.position.to_str(m1.direction), sm1, m1.score, score_1),
                Type::Exch => format!("{}\n>{}: {} -{} +0 {}", out, player_1.name, rack_1,
                                      m1.word, score_1)
            }
        } else {
            out = format!("{}\n{:<02}. {:<7}/{:<3}: {:<12} +{:<03}/{:<03}", out, turn, 
                            rack_1, m1.position.to_str(m1.direction), sm1, m1.score, score_1);
        }

        out = format!("{}\n#note Time: {}", out, time1);

        if player_1.rack.len() == 0 {
            break
        }

        let rack_2: String = player_2.rack.iter().collect();
        let start2 = SystemTime::now();
        let (m2, sm2) = player_2.do_move(&mut board, !gcg);
        let time2 = start2.elapsed().expect("Time went backwards").as_millis();

        if sm2 == String::new() && m2.typ == Type::Play {
            break
        }

        score_2 += m2.score;
        if gcg {
            // out = format!("{}\n>{}: {} {} {} +{} {}", out, player_2.name, rack_2, 
            //               m2.position.to_str(m2.direction), sm2, m2.score, score_2);
            out = match m2.typ {
                Type::Play => format!("{}\n>{}: {} {} {} +{} {}", out, player_2.name, rack_2, 
                                      m2.position.to_str(m2.direction), sm2, m2.score, score_2),
                Type::Exch => format!("{}\n>{}: {} -{} +0 {}", out, player_2.name, rack_2,
                                      m2.word, score_2)
            }
        } else {
            out = format!("{} | {:<7}/{:<3}: {:<12} +{:<03}/{:<03}", out, 
                            rack_2, m2.position.to_str(m2.direction), sm2, m2.score, score_2);
        }

        out = format!("{}\n#note Time: {}", out, time2);
        
        turn += 1;

        println!("{}", out);
    }

    if player_1.rack.len() == 0 {
        let mut end = 0;
        let mut end_s = String::new();
        for s in player_2.rack {
            end += board.bag.score(s);
            end_s.push(s);
        }
        end *= 2;
        score_1 += end;
        if gcg {
            out = format!("{}\n>{}:  ({}) +{} {}", out, player_1.name, 
                          end_s, end, score_1);
        } else {
            out = format!("{}\n 2*({}) +{}/{}", out, end_s, end, score_1);
        }
    } else {
        let mut end = 0;
        let mut end_s = String::new();
        for s in player_1.rack {
            end += board.bag.score(s);
            end_s.push(s);
        }
        end *= 2;
        score_2 += end;
        if gcg {
            out = format!("{}\n>{}:  ({}) +{} {}", out, player_2.name, 
                          end_s, end, score_2);            
        } else {
            out = format!("{}\n {} 2*({}) +{}/{}", out, " ".repeat(40), end_s, end, score_2);    
        }   
    }
    if !gcg {
        out = format!("{}\n{}", out, board);
    }
    println!("{}", out);
}

fn test() {
    let mut board = board::Board::default();

board.play_word(utils::Position { row: 6, col: 7 }, String::from("pREANAL"), utils::Direction::Down, true);
board.play_word(utils::Position { row: 12, col: 7 }, String::from(".ARYNX"), utils::Direction::Across, true);
board.play_word(utils::Position { row: 8, col: 10 }, String::from("CASK."), utils::Direction::Down, true);
board.play_word(utils::Position { row: 9, col: 9 }, String::from("G.UDY"), utils::Direction::Across, true);
board.play_word(utils::Position { row: 8, col: 1 }, String::from("COMITI.S"), utils::Direction::Across, true);
board.play_word(utils::Position { row: 2, col: 12 }, String::from("REPASTE."), utils::Direction::Down, true);
board.play_word(utils::Position { row: 3, col: 11 }, String::from("OUZO"), utils::Direction::Down, true);
board.play_word(utils::Position { row: 7, col: 2 }, String::from("H.TLINER"), utils::Direction::Down, true);
board.play_word(utils::Position { row: 11, col: 10 }, String::from(".OALA"), utils::Direction::Across, true);
board.play_word(utils::Position { row: 14, col: 0 }, String::from("BE.G"), utils::Direction::Across, true);
   
    println!("{}", board);

    let mut player_1 = Player { rack: vec!['D', 'T', 'R', 'E', 'M', 'I', 'I'], name: String::new() };

    let (m1, sm1) = player_1.do_move(&mut board, false);

    println!("{:?} {}", m1, sm1);
    println!("{}", board);
}

fn main() {
    for i in 0..10 {
        two_player_game(true);
    }
    // let mut b = bag::Bag::default();
    // println!("Score for z is: {}", bag.score('z'));

    // let mut dict = utils::Dictionary::default();
    // println!("This should be true: {}", dict.check_word("HELLO".to_string()));
    // println!("This should be false: {}", dict.check_word("REGOIJPREGOJ".to_string()));

    // let t = dictionary::Trie::default();
    // let d = dictionary::Dictionary::default();

    // test();

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