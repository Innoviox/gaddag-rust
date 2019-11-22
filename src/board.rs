use crate::utils::*;
use crate::dictionary::Dictionary;
use crate::dictionary::Trie;
use crate::bag::Bag;
use std::fmt;
use std::collections::HashMap;
use std::collections::HashSet;
use std::slice::Iter;
use std::convert::TryFrom;
use std::convert::TryInto;
use array_init::array_init;
use std::ops::Sub;
use petgraph::graph::NodeIndex;

fn _as(v: usize) -> i32 {
    i32::try_from(v).unwrap()
}

pub struct Board {
    state: [[char; 15]; 15]
}

/*
#: TWS
^: DWS
+: TLS
-: DLS
*: center
*/

impl Board {
    pub fn default() -> Board {
        Board { state: [
            ['#', '.', '.', '-', '.', '.', '.', '#', '.', '.', '.', '-', '.', '.', '#'],
            ['.', '^', '.', '.', '.', '+', '.', '.', '.', '+', '.', '.', '.', '^', '.'],
            ['.', '.', '^', '.', '.', '.', '-', '.', '-', '.', '.', '.', '^', '.', '.'],
            ['-', '.', '.', '^', '.', '.', '.', '-', '.', '.', '.', '^', '.', '.', '-'],
            ['.', '.', '.', '.', '^', '.', '.', '.', '.', '.', '^', '.', '.', '.', '.'],
            ['.', '+', '.', '.', '.', '+', '.', '.', '.', '+', '.', '.', '.', '+', '.'],
            ['.', '.', '-', '.', '.', '.', '-', '.', '-', '.', '.', '.', '-', '.', '.'],
            ['#', '.', '.', '-', '.', '.', '.', '*', '.', '.', '.', '-', '.', '.', '#'],
            ['.', '.', '-', '.', '.', '.', '-', '.', '-', '.', '.', '.', '-', '.', '.'],
            ['.', '+', '.', '.', '.', '+', '.', '.', '.', '+', '.', '.', '.', '+', '.'],
            ['.', '.', '.', '.', '^', '.', '.', '.', '.', '.', '^', '.', '.', '.', '.'],
            ['-', '.', '.', '^', '.', '.', '.', '-', '.', '.', '.', '^', '.', '.', '-'],
            ['.', '.', '^', '.', '.', '.', '-', '.', '-', '.', '.', '.', '^', '.', '.'],
            ['.', '^', '.', '.', '.', '+', '.', '.', '.', '+', '.', '.', '.', '^', '.'],
            ['#', '.', '.', '-', '.', '.', '.', '#', '.', '.', '.', '-', '.', '.', '#'],
        ] }
    }

    pub fn at_position(&self, p: Position) -> char {
        self.state[p.row][p.col]
    }

    fn is_letter(&self, p: Position) -> bool {
        return !"#^+-*.".contains(self.at_position(p))
    }

    fn set(&mut self, p: Position, c: char) {
        self.state[p.row][p.col] = c;
    }

    pub fn play_word(&mut self, p: Position, word: String, dir: Direction, force: bool) -> bool {
        let mut current = p.clone();

        for c in word.chars() {
            if force { self.set(current, c); }
            else {
                match self.at_position(current) {
                    '.' | '*' | '-' | '+' | '^' | '#' => self.set(current, c),
                                                   _  => return false
                }
            }

            if !(current.tick(dir)) { return false }
        }

        true
    }

    pub fn place_move(&mut self, m: &Move) -> bool {
        self.play_word(m.position, m.word.clone(), m.direction, true)
    }

    pub fn place_move_cloned(&self, m: &Move) -> Board {
        let mut c = self.clone();
        c.place_move(m);
        c
    }

    pub fn valid_at(&mut self, p: Position, d: &Dictionary, dir: Direction) -> [bool; 26] {
        if self.is_letter(p) {
            return [false; 26];
        }

        if !p.neighbors().iter().any(|x| self.is_letter(*x)) {
            return [true; 26];
        }

        let mut cross = [false; 26];

        for (i, l) in alph.chars().enumerate() {
            let old = self.at_position(p);
            self.set(p, l);
            cross[i] = self.valid(d, &dir);
            self.set(p, old);
        }

        cross
    }

    pub fn get_words(&self) -> Vec<Move> {
        let mut result = Vec::new();
        let mut marked: [[bool; 225]; 2] = [[false; 225]; 2];

        for p in positions().iter() {
            for (di, d) in Direction::iter().enumerate() {
                if !marked[di][p.to_int()] && self.is_letter(*p) {   
                    let mut curr = p.clone();
                    let mut word = String::new();
                    while self.is_letter(curr) {
                        word.push(self.at_position(curr));
                        marked[di][curr.to_int()] = true;
                        if !curr.tick(*d) { break }
                    }
                    
                    if word.len() > 1 {
                        result.push(Move { word, direction: *d, position: *p, score: 0 });
                    }
                }
            }
        }

        result
    }

    pub fn valid(&self, d: &Dictionary, dir: &Direction) -> bool { // TODO check connectedness
        self.get_words().iter().filter(|x| x.direction == *dir).all(|x| d.check_word(&x.word))
    }

    pub fn anchors(&self) -> Vec<Position> {
        let mut result = Vec::new();

        for p in positions().iter() {
            if !self.is_letter(*p) { continue }
            for n in p.neighbors() {
                if !self.is_letter(n) {
                    result.push(n);
                }
            }
        }

        result
    }

    fn is_anchor(&self, p: Position) -> bool {
        if self.is_letter(p) { return false }

        for n in p.neighbors() {
            if self.is_letter(n) { return true }
        }

        false
    }
}

impl Board {
    pub fn generate_all_moves(&mut self, rack: Vec<char>, trie: &Trie, dict: &Dictionary) -> Vec<Move> {
        let mut result = Vec::new();

        // todo crossscores
        let mut cross_checks: [[Vec<char>; 225]; 2] = [array_init(|_| Vec::new()), array_init(|_| Vec::new())]; // : HashMap<Position, Vec<char>> = HashMap::new();
        for (di, d) in Direction::iter().enumerate() {
            for p in positions().iter() {
                // cross_checks.insert(*p, chars(self.valid_at(*p, dict)));
                cross_checks[di][p.to_int()] = chars(self.valid_at(*p, dict, *d));
            }
        }
        
        for p in self.anchors() {
            for (di, d) in Direction::iter().enumerate() {
                let di_opp: usize = (-_as(di) + 1).try_into().unwrap();
                // for (lp, rp) in gen_parts(rack.clone()).iter() {
                for part in gen_parts(&rack).iter() {
                    for dist in ((-_as(part.len())+1)..1) {
                        if let Some(pos) = p.add(dist.try_into().unwrap(), *d) {
                                // println!("{}, {:?}, {:?}", dist, p, pos);
                            if let Some(mv) = self.place(pos, *d, &part, trie, &cross_checks[di_opp], false) {
                                result.push(mv);
                            }
                        }
                    }
                }
            }
        }

        result
    }

    pub fn place(&mut self, p: Position, d: Direction, part: &Vec<char>, 
                 trie: &Trie, cross_checks: &[Vec<char>; 225], mutate: bool) -> Option<Move> {
        // todo: return vector of positions, not word
        if self.is_letter(p) { return None }
        
        let mut word = Vec::new(); // todo: efficiency - make string?

        let mut curr_left = p.clone();

        if curr_left.tick_opp(d) {
            while self.is_letter(curr_left) { // get stuff before word
                word.push(self.at_position(curr_left));
                if !curr_left.tick_opp(d) { break }
            }
        }

        word.reverse();

        let mut trie_node = trie.seed(&word);

        let mut curr = p.clone(); 
        let mut i = 0;
        while i < part.len() {
            if !self.is_letter(curr) { 
                if !cross_checks[curr.to_int()].contains(&part[i]) { 
                    return None
                } else if let Some(now) = trie.can_next(trie_node, part[i]) {
                    trie_node = now;
                    if (mutate) { self.set(curr, part[i]); }
                    word.push(part[i]);
                } else {
                    return None
                }
                i += 1;
            } else {
                let stumbled = self.at_position(curr);
                if let Some(st) = trie.can_next(trie_node, stumbled) {
                    trie_node = st;
                    word.push(stumbled);
                } else {
                    return None
                }
            }
            if !curr.tick(d) { return None }
        }

        while self.is_letter(curr) { // get stuff after word
            let stumbled = self.at_position(curr);
            if let Some(st) = trie.can_next(trie_node, stumbled) {
                trie_node = st;
                word.push(stumbled);
            } else {
                return None
            }
            if !curr.tick(d) { break }
        }

        let word = word.iter().collect();

        if let Some(_) = trie.can_next(trie_node, '@') { // valid word checker (speed!!!!)
            if (mutate && part.len() == 7) { println!("{} {}", self, word); }

            return Some(Move {
                word,
                // part,
                position: p,
                direction: d, 
                score: 0
            })
        }

        None
    }
}

impl Board {
    pub fn gen_all_moves(&mut self, rack: Vec<char>, trie: &Trie, dict: &Dictionary, bag: &Bag) -> Vec<Move> {
        let mut result = Vec::new();

        // todo crossscores
        // todo only recalc for affected squares
        let mut cross_checks: [[Vec<char>; 225]; 2] = [array_init(|_| Vec::new()), array_init(|_| Vec::new())]; // : HashMap<Position, Vec<char>> = HashMap::new();
        let mut cross_sums: [[i32; 225]; 2] = [array_init(|_| 0), array_init(|_| 0)];
        for (di, d) in Direction::iter().enumerate() {
            for p in positions().iter() {
                cross_checks[di][p.to_int()] = chars(self.valid_at(*p, dict, *d));

                let mut p_sums = p.clone();
                let mut letters = Vec::<char>::new();
                while p_sums.tick(*d) && self.is_letter(p_sums) {
                    letters.push(self.at_position(p_sums));
                }
                p_sums = p.clone();
                while p_sums.tick_opp(*d) && self.is_letter(p_sums) {
                    letters.push(self.at_position(p_sums));
                }
                cross_sums[di][p.to_int()] = letters.iter().map(|x| bag.score(*x)).sum();
            }
        }

        let root = trie.root();

        let rword = to_word(&rack);

        let n_center = !self.is_letter(Position{ row: 7, col: 7 });

        for (di, d) in Direction::iter().enumerate() {
            let di_opp: usize = (-_as(di) + 1).try_into().unwrap();
            for row in 0..15 {
                let mut last_anchor_col = 0;
                for col in 0..15 {
                    let p = Position { row, col };
                    if self.is_anchor(p) || (n_center && (col == 7 && row == 7)) {
                        // println!("Found anchor position {:?}", p);
                        let mut np = p.clone();
                        if np.tick_opp(*d) && self.is_letter(np) { 
                                // println!("Found left-on-board; lefting");
                                self.left_on_board(np, root, trie, &rword, &cross_checks[di_opp], 
                                                    *d, &mut result, &cross_sums[di_opp], bag);
                            // }
                        } else {
                            // println!("Generating l-parts");
                            self.left_part(p, Vec::new(), root, trie, 
                                        &rword, &cross_checks[di_opp], 
                                        *d, &mut result, 
                                        (col - last_anchor_col).try_into().unwrap(), 
                                        String::new(), p, p, &cross_sums[di_opp], bag);
                        }
                        // }
                        last_anchor_col = col;
                    }
                }  
            }
        }

        result
    }

    // todo: fix ugly ass arguments
    fn left_on_board(&self, position: Position, node: NodeIndex, trie: &Trie, rack: &Vec<usize>, cross_checks: &[Vec<char>; 225],
                     direction: Direction, moves: &mut Vec<Move>, cross_sums: &[i32; 225], bag: &Bag) {
        // println!("Received call left-board with {:?}", position);
        let mut np = position.clone();

        let mut word = Vec::<char>::new();

        let new_node = node;

        loop {
            let c = self.at_position(np);
            word.push(c);
            // let new_node = trie.follow(new_node, c).unwrap();

            if !(np.tick_opp(direction) && self.is_letter(np)) { 
                word.reverse();
                //todo: what the fuck is this
                let mut nnp = position.clone();
                nnp.tick(direction);
                let mut nnnp = np.clone();
                nnnp.tick(direction);
                // println!("Seeding with {:?} at {:?}", word, nnp);
                self.extend_right(&Vec::new(), trie.seed(&word), nnp, cross_checks, direction, rack.to_vec(), trie, moves, &word.iter().collect(), nnnp, nnp, cross_sums, bag);
                return
            }
        }
        
    }

    fn left_part(&self, position: Position, part: Vec<char>, node: NodeIndex, 
                 trie: &Trie, rack: &Vec<usize>, cross_checks: &[Vec<char>; 225], 
                 direction: Direction, moves: &mut Vec<Move>, limit: u32, word: String, curr_pos: Position, real_pos: Position, cross_sums: &[i32; 225], bag: &Bag) {
        // println!("Received call left with {:?} {:?} {:?} {:?} {:?}", position, part, limit, curr_pos, real_pos);

        if let Some(seed) = trie.nrseed(&part) { 
            self.extend_right(&part, seed, real_pos, cross_checks, direction, rack.to_vec(), trie, moves, &word, curr_pos, real_pos, cross_sums, bag);
        }

        if limit > 0 {
            let mut c = alph.chars();
            // println!("Lefting at {:?}, considering {:?}", curr_pos, cross_checks[curr_pos.to_int()]);
            for i in 0..26 {
                let next = c.next().unwrap();
                let mut cp = position.clone();
                if cp.tick_opp(direction) { 
                    if rack[i] > 0 && cross_checks[cp.to_int()].contains(&next) { 
                        // println!("Lefting {}", next);

                        let mut new_rack = rack.clone();
                        new_rack[i] -= 1;
                        
                        let mut new_part = part.clone();
                        new_part.push(next);

                        let new_word = next.to_string() + &word;

                        let mut ccp = cp.clone();
                        ccp.tick_opp(direction);

                        if !self.is_letter(ccp) {

                            self.left_part(cp, new_part, node, 
                                        trie, &new_rack, cross_checks, direction,
                                        moves, limit - 1, new_word, cp, real_pos, cross_sums, bag);   
                        }
                    }               
                }
            }
        }
        /*
        extend-right(trie.find(current_part))
        if limit > 0 
            for char on rack
                left-part(rack - char, limit - 1, char + part)
        */
    }

    fn extend_right(&self, part: &Vec<char>, node: NodeIndex, position: Position, cross_checks: &[Vec<char>; 225], direction: Direction, rack: Vec<usize>, trie: &Trie, moves: &mut Vec<Move>, word: &String, start_pos: Position, anchor: Position, cross_sums: &[i32; 225], bag: &Bag) {
        // println!("extending right at {:?} with part {:?}, {} (real: {:?})", position, part, word, start_pos);
        if !self.is_letter(position) {
            if position != anchor {
                if let Some(terminal) = trie.can_next(node, '@') {
                    // return move
                    let mut m = Move { word: word.to_string(), position: start_pos, direction, score: 0 };
                    m.score = self.score(&m, cross_sums, bag);
                    // println!("Found move {:?} {:?} {:?} {}", word, start_pos, direction, m.score);
                    // println!("{}", self.place_move_cloned(&m));
                    moves.push(m);
                }
            }

            // println!("nexts: {:?}", trie.nexts(node));
            for next in trie.nexts(node) {
                match alph.find(next) {
                    Some(unext) => { 
                        // println!("At position {:?}, cc {:?}, considering {:?}", position, cross_checks[position.to_int()], next);
                        if rack[unext] > 0 && cross_checks[position.to_int()].contains(&next) {
                            // println!("\tFound nextable character {:?} {:?} {:?}", next, part, position);
                            let mut np = part.clone();
                            np.push(next);
                            let mut nr = rack.clone();
                            nr[unext] -= 1;
                            let mut npp = position.clone();

                            if npp.tick(direction) {
                                self.extend_right(&np, trie.follow(node, next).unwrap(), npp, cross_checks, direction, nr, trie, moves, &(word.to_owned() + &next.to_string()), start_pos, anchor, cross_sums, bag);
                            }
                        }
                    },
                    None => break
                }
            }
        } else {
            let next = self.at_position(position);
            let mut np = part.clone();
            np.push(next);
            let mut npp = position.clone();
            if npp.tick(direction) {
                if let Some(next_node) = trie.follow(node, next) {
                    self.extend_right(&np, next_node, npp, cross_checks, direction, rack, trie, moves, &(word.to_owned() + &next.to_string()), start_pos, anchor, cross_sums, bag);
                }
            }
        }
    }

    pub fn score(&self, m: &Move, cross_sums: &[i32; 225], bag: &Bag) -> i32 {
        let mut curr_pos = m.position.clone();
        let mut true_score = 0;
        let mut total_cross_score = 0;
        let mut true_mult = 1;
        let mut n_played = 0;
        for i in m.word.chars() {
            let mut cross_mult = 1;
            let mut cross_score = 0;
            let mut tile_mult = 1;
            match self.at_position(curr_pos) {
                '*' | '^' => { true_mult *= 2; cross_mult *= 2 },
                      '#' => { true_mult *= 3; cross_mult *= 3 },
                      '+' => { tile_mult *= 3; },
                      '-' => { tile_mult *= 2; },
                      '.' => {},
                        _ => { cross_mult = 0; n_played += 1; }, // char was already there, so don't score old words
            }

            let cross_sum = cross_sums[curr_pos.to_int()];
            let curr_score = bag.score(i) * tile_mult;

            if cross_sum > 0 {
                cross_score = cross_mult * (curr_score + cross_sum);
                total_cross_score += cross_mult * cross_score;
            }

            true_score += curr_score;

            curr_pos.tick(m.direction); // no check here because must be true
        }

        let mut score = true_mult * true_score + total_cross_score;

        if m.word.len() - n_played == 7 {
            score += 50;
        }

        score
    }
}

impl Board {
    pub fn clone(&self) -> Board {
        Board {
            state: self.state.clone()
        }
    }
}

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sep = "-".repeat(66);

        write!(f, "{}\n", sep).expect("fail");
        write!(f, "|    |").expect("fail");
        for row in alph.chars().take(15) {
            write!(f, "{}", format!(" {} |", row)).expect("fail");
        }
        write!(f, "\n{}\n", sep).expect("fail");

        // let a = self.anchors();

        for (num, row) in self.state.iter().enumerate() {
            write!(f, "| {} |", format!("{:0>2}", num+1)).expect("fail");
            // for sq in row.iter() {
            
            for (col, sq) in row.iter().enumerate() {
                // if a.contains(&Position{ row: num, col }) { 
                    // write!(f, "AAA").expect("fail");
                // } else { 
                    match sq {
                        '#' => write!(f, "TWS").expect("fail"),
                        '^' => write!(f, "DWS").expect("fail"),
                        '+' => write!(f, "TLS").expect("fail"),
                        '-' => write!(f, "DLS").expect("fail"),
                        '.' => write!(f, "   ").expect("fail"),
                        _  => write!(f, " {} ", sq).expect("fail")
                    };
                // }
                write!(f, "|").expect("fail");
            }
            write!(f, "\n{}\n", sep).expect("fail");
        }

        write!(f, "")
	}
}
/*
------------------------------------------------------------------
|    | A | B | C | D | E | F | G | H | I | J | K | L | M | N | O |
------------------------------------------------------------------
| 01 |TWS|   |   |DLS|   |   |   |TWS|   |   |   |DLS|   |   |TWS|
------------------------------------------------------------------
| 02 |   |DWS|   |   |   |TLS|   |   |   |TLS|   |   |   |DWS|   |
------------------------------------------------------------------
| 03 |   |   |DWS|   |   |   |DLS|   |DLS|   |   |   |DWS|   |   |
------------------------------------------------------------------
| 04 |DLS|   |   |DWS|   |   |   |DLS|   |   |   |DWS|   |   |DLS|
------------------------------------------------------------------
| 05 |   |   |   |   |DWS|   |   |   |   |   |DWS|   |   |   |   |
------------------------------------------------------------------
| 06 |   |TLS|   |   |   |TLS|   |   |   |TLS|   |   |   |TLS|   |
------------------------------------------------------------------
| 07 |   |   |DLS|   |   |   |   |   |DLS|   |   |   |DLS|   |   |
------------------------------------------------------------------
| 08 |TWS|   |   |DLS|   |   |   | H | E | L | L | O |   |   |TWS|
------------------------------------------------------------------
| 09 |   |   |DLS|   |   |   |DLS| A |DLS|   |   |   |DLS|   |   |
------------------------------------------------------------------
| 10 |   |TLS|   |   |   |TLS|   | M |   |TLS|   |   |   |TLS|   |
------------------------------------------------------------------
| 11 |   |   |   |   |DWS|   |   |   |   |   |DWS|   |   |   |   |
------------------------------------------------------------------
| 12 |DLS|   |   |DWS|   |   |   |DLS|   |   |   |DWS|   |   |DLS|
------------------------------------------------------------------
| 13 |   |   |DWS|   |   |   |DLS|   |DLS|   |   |   |DWS|   |   |
------------------------------------------------------------------
| 14 |   |DWS|   |   |   |TLS|   |   |   |TLS|   |   |   |DWS|   |
------------------------------------------------------------------
| 15 |TWS|   |   |DLS|   |   |   |TWS|   |   |   |DLS|   |   |TWS|
------------------------------------------------------------------
*/