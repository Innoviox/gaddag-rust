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
}

impl Board {
    pub fn generate_all_moves(&mut self, rack: Vec<char>, trie: &Trie, dict: &Dictionary) -> Vec<Move> {
        let mut result = Vec::new();

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
                            if let Some(mv) = self.clone().place(pos, *d, &part, trie, &cross_checks[di_opp], true) {
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
                word.push(self.at_position(curr));
            }
            if !curr.tick(d) { return None }
        }

        while self.is_letter(curr) { // get stuff after word
            word.push(self.at_position(curr));
            if !curr.tick(d) { break }
        }

        let word = word.iter().collect();

        // if p.row == 7 && p.col == 9 {
            println!("{} {}", self, word);
        // }
        

        // if !dict.check_word(&word) {
        //     return None
        // }


        Some(Move {
            word,
            // part,
            position: p,
            direction: d
        })
    }

    pub fn score(&self, m: Move) {
        
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