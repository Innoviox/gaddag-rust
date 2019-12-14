use crate::utils::Type;
use crate::game::Game;
use std::time::SystemTime;

fn two_player_game(g: &mut Game, gcg: bool) {
    // b.bag = bag::Bag::with(&vec!['S', 'D', 'L', 'A', 'N', '?', 'A', 'U', 'E', 'M', 'S', 'R', 'A', 'C', 'Z', 'E', 'P', 'F', 'T', 'I', 'R', 'O', 'E', 'N', 'F', 'O', 'O', 'Y', 'A', 'N', 'I', 'U', 'L', 'M', 'R', 'E', 'B', 'E', 'A', 'U', 'B', 'A', 'T', 'I', 'L', 'W', 'V', 'N', 'E', 'A', 'G', 'T', 'O', 'O', 'E', 'H', 'A', 'K', 'U', 'R', 'D', 'I', 'I', '?', 'D', 'T', 'V', 'Y', 'N', 'I', 'E', 'Q', 'J', 'S', 'D', 'L', 'E', 'R', 'O', 'E', 'X', 'A', 'I', 'H', 'W', 'O', 'I', 'C', 'P', 'T', 'S', 'R', 'N', 'E', 'T', 'O', 'G', 'G', 'I', 'E']);
    let mut out = String::new();
    
    if gcg {
        let p1 = g.get_player(0).name.clone();
        let p2 = g.get_player(1).name.clone();
        out = format!("#character-encoding UTF-8\n#player1 {n1} {n1}\n#player2 {n2} {n2}",
                      n1=p1, n2=p2);
    }

    let mut turn = 1;

    while !g.is_over() && g.turn < 20 {
        let rack_1: String = g.get_player(0).rack.clone().iter().collect();
        let start1 = SystemTime::now();
        let (m1, sm1, _nmoves1) = g.do_move(false);
        let time1 = start1.elapsed().expect("Time went backwards").as_millis();
        
        if sm1 == String::new() && m1.typ == Type::Play {
            break
        }

        if gcg {
            let p = g.get_player(0);
            out = match m1.typ {
                Type::Play => format!("{}\n>{}: {} {} {} +{} {}", out, p.name, rack_1,
                                      m1.position.to_str(m1.direction), sm1, m1.score, p.score),
                Type::Exch => format!("{}\n>{}: {} -{} +0 {}", out, p.name, rack_1,
                                      m1.word, p.score)
            }
        } else {
            out = format!("{}\n{:<02}. {:<7}/{:<3}: {:<12} +{:<03}/{:<03} ({:<04})", out, turn, 
                            rack_1, m1.position.to_str(m1.direction), sm1, m1.score, g.get_player(0).score, time1);
        }

        if gcg { out = format!("{}\n#note Time: {}", out, time1); }

        if g.get_player(0).rack.len() == 0 {
            break
        }

        let rack_2: String = g.get_player(1).rack.clone().iter().collect();
        let start2 = SystemTime::now();
        let (m2, sm2, _nmoves2) = g.do_move(false);
        let time2 = start2.elapsed().expect("Time went backwards").as_millis();

        if sm2 == String::new() && m2.typ == Type::Play {
            break
        }

        if gcg {
            let p = g.get_player(1);
            out = match m2.typ {
                Type::Play => format!("{}\n>{}: {} {} {} +{} {}", out, p.name, rack_2,
                                      m2.position.to_str(m2.direction), sm2, m2.score, p.score),
                Type::Exch => format!("{}\n>{}: {} -{} +0 {}", out, p.name, rack_2,
                                      m2.word, p.score)
            }
        } else {
            out = format!("{} | {:<7}/{:<3}: {:<12} +{:<03}/{:<03} ({:<04})", out, 
                            rack_2, m2.position.to_str(m2.direction), sm2, m2.score, g.get_player(1).score, time2);
        }

        if gcg { out = format!("{}\n#note Time: {}", out, time2); }
        
        turn += 1;

//         println!("{}", out);
//         println!("{}", g.get_board());
    }

    let (end_s, end, n) = g.finish();
    if n == 0 {
        if gcg {
            let p = g.get_player(0);
            out = format!("{}\n>{}:  ({}) +{} {}", out, p.name,
                          end_s, end, p.score);
        } else {
            out = format!("{}\n 2*({}) +{}/{}", out, end_s, end, g.get_player(0).score);
        }
    } else {
        if gcg {
            let p = g.get_player(1);
            out = format!("{}\n>{}:  ({}) +{} {}", out, p.name,
                          end_s, end, p.score);
        } else {
            out = format!("{}\n {} 2*({}) +{}/{}", out, " ".repeat(45), end_s, end, g.get_player(1).score);
        }   
    }
    if !gcg {
        out = format!("{}\n{}", out, g.get_board());
    }
    println!("{}", out);
}

pub fn main(n: u32, w1: f32, w2: f32) {
    let mut game = Game::test_weights((w1, w2));
    for _ in 0..n {
        two_player_game(&mut game, true);
        game.reset();
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