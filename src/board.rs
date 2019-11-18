use crate::utils::*;
use std::fmt;
use std::collections::HashMap;
use std::slice::Iter;

pub struct Board {
    state: [[char; 15]; 15],
    dictionary: Dictionary
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
        ], dictionary: Dictionary::default() }
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

    pub fn play_word(&mut self, p: Position, word: String, dir: Direction) -> bool {
        let mut current = p.clone();

        for c in word.chars() {
            match self.at_position(current) {
                '.' | '*' | '-' | '+' | '^' | '#' => self.set(current, c),
                                                _ => return false
            }

            if !(current.tick(dir)) { return false }
        }

        true
    }

    pub fn valid_at(&mut self, p: Position) -> [bool; 26] {
        if self.is_letter(p) {
            return [false; 26];
        }

        let mut cross = [false; 26];

        for (i, l) in alph.chars().enumerate() {
            let old = self.at_position(p);
            self.set(p, l);
            cross[i] = self.valid();
            self.set(p, old);
        }

        cross
    }

    pub fn get_words(&self) -> Vec<String> {
        let mut result = Vec::new();

        let mut marked: HashMap<Position, [bool; 2]> = HashMap::new();

        for p in positions().iter() {
            for (di, d) in Direction::iter().enumerate() {
                if (!marked.contains_key(&p) || !marked[&p][di]) && self.is_letter(*p) {                    
                    let mut curr = p.clone();
                    let mut word = String::new();
                    while self.is_letter(curr) {
                        word.push(self.at_position(curr));
                        if !marked.contains_key(&curr) {
                            marked.insert(curr, [false, false]);
                        }
                        marked.get_mut(&curr).unwrap()[di] = true;
                        if !curr.tick(*d) { break }
                    }
                    
                    if word.len() > 1 {
                        result.push(word);
                    }
                }
            }
        }

        result
    }

    pub fn valid(&self) -> bool {
        self.get_words().iter().all(|x| self.dictionary.check_word(x.to_string()))
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
    pub fn generate_all_moves(&self, rack: Vec<char>) -> Vec<Move> {
        let mut result = Vec::new();

        for p in self.anchors() {
            println!("{:?}", p);
            for d in Direction::iter() {
                for (lp, rp) in gen_parts(rack.clone()).iter() {
                    if let Some(mv) = self.clone().place(p, *d, lp.to_vec(), rp.to_vec()) {
                        result.push(mv);
                    }
                }
            }
        }

        result
    }

    fn place(&mut self, p: Position, d: Direction, lp: Vec<char>, rp: Vec<char>) -> Option<Move> {
        let mut word = Vec::new();

        let mut curr_left = p.clone();
        let mut i = 0;
        while i < lp.len() {
            if !curr_left.tick_opp(d) { return None }
            if !self.is_letter(curr_left) { 
                self.set(curr_left, lp[i]);
                i += 1;
            }
            word.push(self.at_position(curr_left));
        }

        word = word.iter().rev().cloned().collect();
        
        let mut curr_right = p.clone();
        i = 0;
        while i < rp.len() {
            if !curr_right.tick(d) { return None }
            if self.is_letter(curr_right) { continue }
            self.set(curr_right, rp[i]);
            word.push(self.at_position(curr_right));
            i += 1;
        }

        Some(Move {
            word: word.iter().collect(),
            position: curr_left,
            direction: d
        })
    }
}

impl Board {
    fn clone(&self) -> Board {
        Board {
            state: self.state.clone(),
            dictionary: Dictionary::default() // todo: copy???
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
                //     write!(f, "AAA").expect("fail");
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

        write!(f, "\n")
	}
}
