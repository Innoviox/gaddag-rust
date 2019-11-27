use crate::utils::*;
use crate::dictionary::Dictionary;
use crate::dictionary::Trie;
use crate::bag::Bag;
use std::fmt;
use std::convert::TryFrom;
use std::convert::TryInto;
use array_init::array_init;
use petgraph::graph::NodeIndex;
use itertools::Itertools;

fn _as(v: usize) -> i32 {
    i32::try_from(v).unwrap()
}

pub struct Board {
    state: [[char; 15]; 15],
    dict: Dictionary,
    trie: Trie, 
    pub bag: Bag, // public so can draw tiles
    blanks: Vec<Position>
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
        ], trie: Trie::default(), dict: Dictionary::default(), bag: Bag::default(), blanks: vec![] }
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
            let uc = c.to_uppercase().next().unwrap();
            if force { if uc != '.' { self.set(current, uc); } }
            else {
                match self.at_position(current) {
                    '.' | '*' | '-' | '+' | '^' | '#' => self.set(current, uc),
                                                   _  => return false
                }
            }

            if c.is_lowercase() {
                self.blanks.push(current);
            }

            if !(current.tick(dir)) { return false }
        }

        true
    }

    pub fn place_move(&mut self, m: &Move) -> bool {
        self.play_word(m.position, m.word.clone(), m.direction, true)
    }

    pub fn place_move_cloned(&mut self, m: &Move) -> String {
        let state = self.state.clone();
        self.place_move(m);
        let out = format!("{}", self);
        self.state = state;
        out
    }

    pub fn valid_at(&mut self, p: Position, dir: Direction) -> [bool; 26] {
        if self.is_letter(p) {
            return [false; 26];
        }

        if !p.neighbors().iter().any(|x| self.is_letter(*x)) {
            return [true; 26];
        }

        let mut cross = [false; 26];

        for (i, l) in alph.chars().enumerate() {
            if l == '?' { continue } 
            let old = self.at_position(p);
            self.set(p, l);
            cross[i] = self.valid(&dir);
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
                        result.push(Move { word, direction: *d, position: *p, score: 0, evaluation: 0.0, typ: Type::Play });
                    }
                }
            }
        }

        result
    }

    pub fn valid(&self, dir: &Direction) -> bool { // TODO check connectedness
        self.get_words().iter().filter(|x| x.direction == *dir).all(|x| self.dict.check_word(&x.word))
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
    pub fn gen_all_moves(&mut self, rack: &Vec<char>) -> Vec<Move> {
        /*
        This method generates all possible moves from the current state with the given rack.

        Note that the board passed must counterintuitively be mutable because this method calls valid_at, 
        which requires mutability even though it does not mutate. Therefore, this is a non-mutating method.

        This method follows the generation method outlined in https://www.cs.cmu.edu/afs/cs/academic/class/15451-s06/www/lectures/scrabble.pdf
        by Appel and Jacobson with a few minor changes.
        */

        let mut result = Vec::new(); // This will store the found moves and be passed-by-reference to the recursive methods.

        // todo only recalc for affected squares <<<<< IMPORTANT
        /*
        cross-checks say what letters are valid for a given space.
        Note that these are coded by direction because different letters are necessary to check in different directions.
        For example, if you are going across, you only care about the words that are going down, and vice-versa (hence *cross*-checks).
        cross-sums are similar, but they sum the values of contiguous letters to aid in scoring. (e.g., not important to the algorithm).
        */
        let mut cross_checks: [[Vec<char>; 225]; 2] = [array_init(|_| Vec::new()), array_init(|_| Vec::new())]; 
        let mut cross_sums: [[i32; 225]; 2] = [array_init(|_| 0), array_init(|_| 0)];
        for (di, d) in Direction::iter().enumerate() {
            for p in positions().iter() {
                cross_checks[di][p.to_int()] = chars(self.valid_at(*p, *d)); // note: requires mutability. also expensive method.

                let mut p_sums = p.clone(); // start at position
                let mut score = 0;
                let mut found = false;
                while p_sums.tick(*d) && self.is_letter(p_sums) { // go forward until find non letter
                    found = true; // found a letter so there are cross-sums. this is to distinguish finding a blank from finding nothing.
                    
                    if !self.blanks.contains(&p_sums) { // blanks are worth 0
                        score += self.bag.score(self.at_position(p_sums));
                    }
                }
                p_sums = p.clone();
                while p_sums.tick_opp(*d) && self.is_letter(p_sums) { // go backwards until find non letter
                    found = true;
                    if !self.blanks.contains(&p_sums) {
                        score += self.bag.score(self.at_position(p_sums));
                    }
                }

                if found {
                    cross_sums[di][p.to_int()] = score;
                } else {
                    cross_sums[di][p.to_int()] = -1; // no tiles across, so don't need to score cross-word
                }
            }
        }

        // Initialize some necessary variables.
        let root = self.trie.root();

        let rword = to_word(&rack); // convert it to a vector-word (see utils.rs) for ease of insertion and deletion.

        let n_center = !self.is_letter(Position{ row: 7, col: 7 }); // if we need to play at the center or not

        /*
        The following algorithm starts off the left-part algorithms at each anchor square.
        Note that it is repeated twice; the second loop is simply the same operation but on the
        transpose of the board.
        */
        let mut di = 0;
        let mut d = Direction::Across;
        let mut di_opp = 1; // opposite direction, for indexing cross_checks

        let mut last_anchor_col = 0; // last column of the anchor to calculate the distance between current and last anchor square

        for row in 0..15 { // iterate over positions
            last_anchor_col = 0; // reset last column
            for col in 0..15 {
                let p = Position { row, col };
                if self.is_anchor(p) || (n_center && (p.col == 7 && p.row == 7)) { // operate on either anchor, or middle piece *if* center is not *
                    let mut np = p.clone();
                    if np.tick_opp(d) && self.is_letter(np) { // if left is a letter, use left part already on board
                            self.left_on_board(np, &rword, &cross_checks[di_opp], 
                                                d, &mut result, &cross_sums[di_opp]);
                    } else { // make left part from rack. note that these arguments are really ugly but all fairly necessary (some for debugging)
                        self.left_part(p, Vec::new(), root, 
                                    &rword, &cross_checks[di_opp], 
                                    d, &mut result, 
                                    (p.col - last_anchor_col + 1).try_into().unwrap(), 
                                    String::new(), p, p, &cross_sums[di_opp]);
                    }
                    last_anchor_col = p.col;
                }
            }  
        }

        // Repetition with the transpose.
        di = 1;
        d = Direction::Down;
        di_opp = 0;
        for col in 0..15 {
            last_anchor_col = 0;   
            for row in 0..15 {
                let p = Position { row, col };
                if self.is_anchor(p) || (n_center && (p.col == 7 && p.row == 7)) {
                    let mut np = p.clone();
                    if np.tick_opp(d) && self.is_letter(np) { 
                            self.left_on_board(np, &rword, &cross_checks[di_opp], 
                                                d, &mut result, &cross_sums[di_opp]);
                    } else {
                        self.left_part(p, Vec::new(), root, 
                                    &rword, &cross_checks[di_opp], 
                                    d, &mut result, 
                                    (p.row - last_anchor_col + 1).try_into().unwrap(), 
                                    String::new(), p, p, &cross_sums[di_opp]);
                    }
                    last_anchor_col = p.row;
                }
            }  
        }
        
        /*
        Implement exchanges (dirtily) e.g., for all possible exchanges, evaluate.
        Note that this returns the complement; for example, rack 
        BCDFGHQ, a move with word of "C" means "exchange everything but C".
        This is for efficiency reasons: exchanging is quite light, but heavy
        if we must include rack-pruning as well. Therefore, this is done
        on the side.
        */
        if self.bag.distribution.len() > 7 { 
            for i in 0..rack.len() {
                for j in rack.iter().cloned().combinations(i) {
                    let jw = to_word(&j);
                    result.push(Move { word: j.iter().collect(), 
                                        position: Position { row: 0, col: 0 }, direction: Direction::Down, score: 0, 
                                        evaluation: *self.dict.evaluate(&jw).expect(&format!("{:?}", &jw)),
                                        typ: Type::Exch });
                }
            }
        }

        result
    }

    // todo: fix ugly arguments
    fn left_on_board(&self, position: Position, rack: &Vec<usize>, cross_checks: &[Vec<char>; 225],
                     direction: Direction, moves: &mut Vec<Move>, cross_sums: &[i32; 225]) {
        /*
        This method extends the current move as left as possible using only tiles on the board, and then
        passes it on to extend-right. For example, the following situation:
        |   | H | E |AAA|
        where AAA is the extending anchor square, requires this method. This method will then 
        take H and E, and generate all moves that extend right from that start.

        Note that this method is called with the position of the first letter (so E in the example).
        */
        // println!("Received call left-board with {:?}", position);
        let mut np = position.clone();

        let mut word = Vec::<char>::new();

        loop {
            word.push(self.at_position(np)); // add character found on board

            /*
            This method can end for two reasons.
            1) We can go no farther left in the given direction.
            2) We have run into a square that does not contain a letter.
            (1) introduces an important edge case that we will discuss later.
            */
            if !(np.tick_opp(direction) && self.is_letter(np)) { // end method
                word.reverse(); // reverse word - we were traversing left, so in the above example we would have found "EH", so we need to 
                                // reverse before passing to lower methods
                let mut nnp = position.clone(); // get position that extend-right will start at, which is one right of the given position.
                nnp.tick(direction);
                let mut nnnp = np.clone(); // we may need to tick the start position.
                /*
                In the above example, we would hit the space to the left of H and get a non-letter (case (2)). 
                In that case, the start position of the move is H, so we need to move one right before extending right.
                However, in case (1), if you have hit the edge of the board, you do not tick.
                */
                if !self.is_letter(np) ||
                    !((np.row == 0 && direction == Direction::Down) || 
                     (np.col == 0 && direction == Direction::Across)) {
                    nnnp.tick(direction);
                }
                // println!("Seeding with {:?} at {:?}", word, nnp);
                // pass to extend-right
                self.extend_right(&Vec::new(), self.trie.seed(&word), nnp, cross_checks, direction, rack.to_vec(), moves, &word.iter().collect(), nnnp, nnp, cross_sums);
                return
            }
        }
        
    }

    fn left_part(&self, position: Position, part: Vec<char>, node: NodeIndex, 
                 rack: &Vec<usize>, cross_checks: &[Vec<char>; 225], 
                 direction: Direction, moves: &mut Vec<Move>, limit: u32, word: String, curr_pos: Position, real_pos: Position, cross_sums: &[i32; 225]) {
        // if real_pos.row == 13 && real_pos.col == 6 {
        // println!("Received call left with {:?} {:?} {:?} {:?} {:?} {:?}", position, part, limit, curr_pos, real_pos, direction);
        // }
        // if let Some(seed) = self.trie.nrseed(&part) {
        if let Some(seed) = self.trie.follow(node, '#') { 
            self.extend_right(&part, seed, real_pos, cross_checks, direction, rack.to_vec(), moves, &word, curr_pos, real_pos, cross_sums);
        }

        if limit > 0 {
            let mut c = alph.chars();
            // println!("Lefting at {:?}, considering {:?}", curr_pos, cross_checks[curr_pos.to_int()]);
            let mut cp = position.clone();
            if cp.tick_opp(direction) {  // todo if not tick opp add check?
                for i in 0..26 {
                    let next = c.next().unwrap();
                
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
                            if let Some(nnode) = self.trie.follow(node, next) {
                                self.left_part(cp, new_part, nnode, 
                                // self.left_part(cp, new_part, node, 
                                            &new_rack, cross_checks, direction,
                                            moves, limit - 1, new_word, cp, real_pos, cross_sums);   
                            }
                        }
                    }               
                }
            }

            if rack[26] > 0 { // blank
                let mut new_rack = rack.clone();
                new_rack[26] -= 1;
                
                let mut cp = position.clone();
                if cp.tick_opp(direction) {
                    for c in cross_checks[cp.to_int()].clone() {
                        let mut new_part = part.clone();
                        new_part.push(c);

                        let new_word = c.to_lowercase().to_string() + &word;

                        let mut ccp = cp.clone();
                        ccp.tick_opp(direction);

                        if !self.is_letter(ccp) {
                            if let Some(nnode) = self.trie.follow(node, c) {
                                self.left_part(cp, new_part, nnode, 
                                // self.left_part(cp, new_part, node, 
                                            &new_rack, cross_checks, direction,
                                            moves, limit - 1, new_word, cp, real_pos, cross_sums);   
                            }
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

    fn extend_right(&self, part: &Vec<char>, node: NodeIndex, position: Position, cross_checks: &[Vec<char>; 225], direction: Direction, rack: Vec<usize>, moves: &mut Vec<Move>, word: &String, start_pos: Position, anchor: Position, cross_sums: &[i32; 225]) {
        // println!("Extending right at {:?} {:?} {:?} {}", position, anchor, part, word);
        if !self.is_letter(position) {
            if position != anchor {
                if let Some(_terminal) = self.trie.can_next(node, '@') {
                    // return move
                    let mut m = Move { word: word.to_string(), position: start_pos, 
                                       direction, score: 0, evaluation: *self.dict.evaluate(&rack).expect(&format!("{:?}", &rack)), 
                                       typ: Type::Play }; 
                    m.score = self.score(&m, cross_sums);
                    // if m.word == "EPOXY".to_string() { println!("Found move {:?}", m); }
                    moves.push(m);
                }
            }

            for next in self.trie.nexts(node) {
                // println!("\tconsidering next {} {:?} {:?}", next, self.trie.nexts(node), cross_checks[position.to_int()]);
                if let Some(unext) = alph.find(next) {
                    if cross_checks[position.to_int()].contains(&next) { // todo: blanks here?
                        // println!("\tvalid next");
                        if rack[unext] > 0 || rack[26] > 0 {
                            let mut np = part.clone();
                            np.push(next);
                            let mut nr = rack.clone();
                            let mut snext = next.to_string();
                            if rack[unext] > 0 { 
                                nr[unext] -= 1; 
                            } else { 
                                nr[26] -= 1; 
                                snext = next.to_lowercase().to_string();
                            }
                            let mut npp = position.clone();

                            let nnode = self.trie.follow(node, next).unwrap();
                            let nword = &(word.to_owned() + &snext);

                            if npp.tick(direction) {
                                self.extend_right(&np, nnode, npp, cross_checks, direction, nr, moves, 
                                                  nword, start_pos, anchor, cross_sums);
                            } else if let Some(_terminal) = self.trie.can_next(nnode, '@') {
                                let mut m = Move { word: nword.to_string(), position: start_pos, 
                                                direction, score: 0, evaluation: *self.dict.evaluate(&nr).expect(&format!("{:?}", &nr)),
                                                typ: Type::Play };  
                                m.score = self.score(&m, cross_sums);
                                moves.push(m);
                            }
                        }
                    }
                }
            }
        } else {
            let next = self.at_position(position);
            let mut np = part.clone();
            np.push(next);
            let mut npp = position.clone();

            let nword = &(word.to_owned() + &next.to_string());
            
            if let Some(next_node) = self.trie.follow(node, next) {
                if npp.tick(direction) {
                    self.extend_right(&np, next_node, npp, cross_checks, direction, rack, moves, nword, start_pos, anchor, cross_sums);
                }
            }
        }
    }

    pub fn reals(&self, m: &Move) -> Vec<char> {
        let mut curr_pos = m.position.clone();
        let mut result = Vec::new();
        for i in m.word.chars() {
            if !self.is_letter(curr_pos) {
                result.push(i);
            }
            curr_pos.tick(m.direction);
        }

        result
    }

    pub fn format(&self, m: &Move, human: bool) -> String {
        let mut res = String::new();
        let mut curr_pos = m.position.clone(); // todo make move iter method
        for i in m.word.chars() {
            if !self.is_letter(curr_pos) {
                res.push(i);
            } else {
                if human {
                    res.push('(');
                    res.push(i);
                    res.push(')');
                } else {
                    res.push('.');
                }
            }
            curr_pos.tick(m.direction);
        }

        res.replace(")(", "")
    }

    pub fn score(&self, m: &Move, cross_sums: &[i32; 225]) -> i32 {
        let mut curr_pos = m.position.clone();
        let mut true_score = 0;
        let mut total_cross_score = 0;
        let mut true_mult = 1;
        let mut n_played = 0;
        for i in m.word.chars() {
            let mut cross_mult = 1;
            let mut tile_mult = 1;
            match self.at_position(curr_pos) {
                '*' | '^' => { true_mult *= 2; cross_mult *= 2 },
                      '#' => { true_mult *= 3; cross_mult *= 3 },
                      '+' => { tile_mult *= 3; },
                      '-' => { tile_mult *= 2; },
                      '.' => {},
                        _ => { cross_mult = 0; n_played += 1; }, // char was already there, so don't score old words
            }

            let mut curr_score = 0;
            if !(i.is_lowercase() || self.blanks.contains(&curr_pos)) {
                curr_score = self.bag.score(i) * tile_mult;
            }

            let cross_sum = cross_sums[curr_pos.to_int()];

            if cross_sum >= 0 {
                let cross_score = curr_score + cross_sum;
                // println!("Found cross score {:?} {} {} {} {}", curr_pos, cross_score, cross_mult, curr_score, cross_sum);
                total_cross_score += cross_mult * cross_score;
            }

            true_score += curr_score;

            curr_pos.tick(m.direction); // no check here because must be true
        }

        // println!("Found true score {} {}", true_mult, true_score);

        let mut score = true_mult * true_score + total_cross_score;
        
        // println!("Final {}", score);

        if m.word.len() - n_played == 7 {
            score += 50;
        }

        score
    }
}

// impl Board {
//     pub fn clone(&self) -> Board {
//         Board {
//             state: self.state.clone(),
//             dict: self.dict,
//             trie: self.trie,
//             bag: self.bag
//         }
//     }
// }

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
                        _  => {
                            if self.blanks.contains(&Position{ row: num, col }) {
                                write!(f, " {} ", sq.to_lowercase()).expect("fail")
                            } else {
                                write!(f, " {} ", sq).expect("fail")
                            }
                        }
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