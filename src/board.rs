use crate::utils::*;
use std::fmt;
use std::collections::HashMap;
use std::slice::Iter;
use std::convert::TryInto;

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

    pub fn valid_at(&mut self, p: Position, d: &Dictionary) -> [bool; 26] {
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
            cross[i] = self.valid(d);
            self.set(p, old);
        }

        cross
    }

    pub fn get_words(&self) -> Vec<String> {
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
                        result.push(word);
                    }
                }
            }
        }

        result
    }

    pub fn valid(&self, d: &Dictionary) -> bool { // TODO check connectedness
        self.get_words().iter().all(|x| d.check_word(&x.to_string()))
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
    pub fn generate_all_moves(&mut self, rack: Vec<char>, dict: &Dictionary) -> Vec<Move> {
        let mut result = Vec::new();

        let mut cross_checks: HashMap<Position, Vec<char>> = HashMap::new();
        for p in positions().iter() {
            cross_checks.insert(*p, chars(self.valid_at(*p, dict)));
        }

        for p in self.anchors() {
            for d in Direction::iter() {
                // for (lp, rp) in gen_parts(rack.clone()).iter() {
                for part in gen_parts(rack.clone()).iter() {
                    for dist in (-part.len()-1)..(part.len()+1) {
                        if let some(pos) = p.add(dist.try_into().unwrap(), *d) {
                            if let Some(mv) = self.clone().place(pos, *d, part.to_vec(), dict, &cross_checks) {
                                result.push(mv);
                            }
                        }
                    }
                }
            }
        }

        result
    }

    pub fn place(&mut self, p: Position, d: Direction, part: Vec<char>, dict: &Dictionary, cross_checks: &HashMap<Position, Vec<char>>) -> Option<Move> {
        let mut word = Vec::new();

        let mut curr = p.clone();
        let mut i = 0;
        while i < part.len() {
            if !self.is_letter(curr) { 
                if cross_checks.get(&curr).unwrap().contains(&lp[i]) { 
                    self.set(curr, lp[i]);
                } else {
                    return None
                }
                i += 1;
            }
            word.push(self.at_position(curr));
            if !curr.tick(d) { return None }
        }

        // word.reverse();
        
        // let mut curr_right = p.clone();
        // i = 0;
        // while i < rp.len() {
        //     if !curr_right.tick(d) { return None }
        //     if !self.is_letter(curr_right) {
        //         if cross_checks.get(&curr_right).unwrap().contains(&rp[i]) { 
        //             self.set(curr_right, rp[i]);
        //         } else {
        //             return None
        //         }
        //         i += 1;
        //     }
        //     word.push(self.at_position(curr_right));
        // }

        // println!("{} {:?} {}", self, self.get_words(), self.valid(dict));

        let word = word.iter().collect();

        if !dict.check_word(&word) {
            return None
        }

        // println!("{} {:?} {} {:?} {:?} {:?} {:?}", self, self.get_words(), self.valid(dict), p, d, lp, rp);

        Some(Move {
            word,
            position: p,
            direction: d
        })
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

        write!(f, "")
	}
}
