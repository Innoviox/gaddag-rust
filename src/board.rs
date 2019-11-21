use crate::utils::*;
use crate::dictionary::Dictionary;
use crate::dictionary::Trie;
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
                        result.push(Move { word, direction: *d, position: *p });
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
            if self.is_letter(n) { return false }
        }

        true
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
                direction: d
            })
        }

        None
    }

    pub fn score(&self, m: Move) {
        
    }
}

impl Board {
    pub fn gen_all_moves(&mut self, rack: Vec<char>, trie: &Trie, dict: &Dictionary) -> Vec<Move> {
        let mut result = Vec::new();

        // todo crossscores
        let mut cross_checks: [[Vec<char>; 225]; 2] = [array_init(|_| Vec::new()), array_init(|_| Vec::new())]; // : HashMap<Position, Vec<char>> = HashMap::new();
        for (di, d) in Direction::iter().enumerate() {
            for p in positions().iter() {
                // cross_checks.insert(*p, chars(self.valid_at(*p, dict)));
                cross_checks[di][p.to_int()] = chars(self.valid_at(*p, dict, *d));
            }
        }
        
        let root = trie.root();

        let rword = to_word(&rack);

        // for p in self.anchors() {
        //     for (di, d) in Direction::iter().enumerate() {
        //         let di_opp: usize = (-_as(di) + 1).try_into().unwrap();
        //         self.left_part(p, Vec::new(), root, trie, rword, &cross_checks[di_opp], *d, &mut result);
        //     }
        // }


        for (di, d) in Direction::iter().enumerate() {
            let di_opp: usize = (-_as(di) + 1).try_into().unwrap();
            for row in 0..15 {
                let last_anchor_col = 0;
                for col in 0..15 {
                    let p = Position { row, col };
                    if self.is_anchor(p) {
                        self.left_part(p, Vec::new(), root, trie, 
                                       rword, &cross_checks[di_opp], 
                                       *d, &mut result, 
                                       (col - last_anchor_col).try_into().unwrap(), 
                                       String::new());
                        last_anchor_col = col;
                    }
                }  
            }
        }

        result
    }

    fn left_part(&self, position: Position, part: Vec<char>, node: NodeIndex, 
                 trie: &Trie, rack: Vec<usize>, cross_checks: &[Vec<char>; 225], 
                 direction: Direction, moves: &mut Vec<Move>, limit: u32, word: String) {
        self.extend_right(&part, node, position, cross_checks, direction, rack.to_vec(), trie, moves, word);

        if limit > 0 {
            for next in trie.nexts(node) {
                let unext = alph.find(next).unwrap();
                if rack[unext] > 0 {
                    let mut new_rack = rack.clone();
                    new_rack[unext] -= 1;

                    let next_node = trie.follow(node, next).unwrap();
                    
                    let mut new_part = part.clone();
                    new_part.push(next);

                    let mut new_word = next.to_string() + &word;

                    self.left_part(position, new_part, next_node, 
                                   trie, new_rack, cross_checks, direction,
                                   moves, limit - 1, new_word);
                }
            }
        }
    }

    fn extend_right(&self, part: &Vec<char>, node: NodeIndex, position: Position, cross_checks: &[Vec<char>; 225], direction: Direction, rack: Vec<usize>, trie: &Trie, moves: &mut Vec<Move>, word: String) {
        if !self.is_letter(position) {
            if let Some(terminal) = trie.can_next(node, '@') {
                // return move
                moves.push(Move { word, position, direction });
            }

            for next in trie.nexts(node) {
                let unext = alph.find(next).unwrap();
                if rack[unext] > 0 && cross_checks[position.to_int()].contains(&next) {
                    let mut np = part.clone();
                    np.push(next);
                    let mut nr = rack.clone();
                    nr[unext] -= 1;
                    let mut npp = position.clone();

                    if npp.tick(direction) {
                        self.extend_right(&np, trie.follow(node, next).unwrap(), npp, cross_checks, direction, nr, trie, moves, word + &next.to_string());
                    }
                }
            }
        } else {
            let next = self.at_position(position);
            let mut np = part.clone();
            np.push(next);
            let mut npp = position.clone();
            if npp.tick(direction) {
                self.extend_right(&np, trie.follow(node, next).unwrap(), npp, cross_checks, direction, rack, trie, moves, word + &next.to_string());
            }
        }
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