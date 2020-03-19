use crate::bag::Bag;
use crate::dictionary::Dictionary;
use crate::dictionary::Trie;
use crate::utils::*;
use array_init::array_init;
use itertools::Itertools;
use petgraph::graph::NodeIndex;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt;

pub type S = ([[char; 15]; 15], Vec<Position>, [[Vec<char>; 225]; 2]);

fn _as(v: usize) -> i32 {
    // made this before i knew about the as keyword whoops
    i32::try_from(v).unwrap()
}

pub struct Board {
    state: [[char; 15]; 15],
    dict: Dictionary,
    trie: Trie,
    pub bag: Bag, // public so can draw tiles
    pub blanks: Vec<Position>,
    cross_checks: [[Vec<char>; 225]; 2],
    affected: Vec<Position>,
}

/*
#: TWS
^: DWS
+: TLS
-: DLS
*: center
*/

pub const STATE: [[char; 15]; 15] = [
    [
        '#', '.', '.', '-', '.', '.', '.', '#', '.', '.', '.', '-', '.', '.', '#',
    ],
    [
        '.', '^', '.', '.', '.', '+', '.', '.', '.', '+', '.', '.', '.', '^', '.',
    ],
    [
        '.', '.', '^', '.', '.', '.', '-', '.', '-', '.', '.', '.', '^', '.', '.',
    ],
    [
        '-', '.', '.', '^', '.', '.', '.', '-', '.', '.', '.', '^', '.', '.', '-',
    ],
    [
        '.', '.', '.', '.', '^', '.', '.', '.', '.', '.', '^', '.', '.', '.', '.',
    ],
    [
        '.', '+', '.', '.', '.', '+', '.', '.', '.', '+', '.', '.', '.', '+', '.',
    ],
    [
        '.', '.', '-', '.', '.', '.', '-', '.', '-', '.', '.', '.', '-', '.', '.',
    ],
    [
        '#', '.', '.', '-', '.', '.', '.', '*', '.', '.', '.', '-', '.', '.', '#',
    ],
    [
        '.', '.', '-', '.', '.', '.', '-', '.', '-', '.', '.', '.', '-', '.', '.',
    ],
    [
        '.', '+', '.', '.', '.', '+', '.', '.', '.', '+', '.', '.', '.', '+', '.',
    ],
    [
        '.', '.', '.', '.', '^', '.', '.', '.', '.', '.', '^', '.', '.', '.', '.',
    ],
    [
        '-', '.', '.', '^', '.', '.', '.', '-', '.', '.', '.', '^', '.', '.', '-',
    ],
    [
        '.', '.', '^', '.', '.', '.', '-', '.', '-', '.', '.', '.', '^', '.', '.',
    ],
    [
        '.', '^', '.', '.', '.', '+', '.', '.', '.', '+', '.', '.', '.', '^', '.',
    ],
    [
        '#', '.', '.', '-', '.', '.', '.', '#', '.', '.', '.', '-', '.', '.', '#',
    ],
];

impl Board {
    pub fn default() -> Board {
        let mut b = Board {
            state: STATE.clone(),
            trie: Trie::default(),
            dict: Dictionary::default(),
            bag: Bag::default(),
            blanks: vec![],
            cross_checks: [array_init(|_| Vec::new()), array_init(|_| Vec::new())],
            affected: vec![],
        };
        for di in 0..2 {
            for p in positions().iter() {
                b.cross_checks[di][p.to_int()] = chars([true; 26]);
            }
        }

        // for testing exchanges
        // note - at present, game ends 413-506, 9a DISHW.RE
        //        b.bag = Bag::with(&vec!['P', 'F', 'T', 'C', 'Z', 'S', 'D', 'L', 'A', 'N', '?', 'A', 'U', 'E', 'M', 'S', 'R', 'A', 'E', 'I', 'R', 'O', 'E', 'N', 'F', 'O', 'O', 'Y', 'A', 'N', 'I', 'U', 'L', 'M', 'R', 'E', 'B', 'E', 'A', 'U', 'B', 'A', 'T', 'I', 'L', 'W', 'V', 'N', 'E', 'A', 'G', 'T', 'O', 'O', 'E', 'H', 'A', 'K', 'U', 'R', 'D', 'I', 'I', '?', 'D', 'T', 'V', 'Y', 'N', 'I', 'E', 'Q', 'J', 'S', 'D', 'L', 'E', 'R', 'O', 'E', 'X', 'A', 'I', 'H', 'W', 'O', 'I', 'C', 'P', 'T', 'S', 'R', 'N', 'E', 'T', 'O', 'G', 'G', 'I', 'E']);

        b
    }

    pub fn reset(&mut self) {
        self.state = STATE.clone();
        self.bag = Bag::default();
        self.blanks = vec![];
        self.cross_checks = [array_init(|_| Vec::new()), array_init(|_| Vec::new())];
        self.affected = vec![];
        for di in 0..2 {
            for p in positions().iter() {
                self.cross_checks[di][p.to_int()] = chars([true; 26]);
            }
        }
    }

    pub fn at_position(&self, p: Position) -> char {
        self.state[p.row][p.col]
    }

    pub fn is_letter(&self, p: Position) -> bool {
        return !"#^+-*.".contains(self.at_position(p));
    }

    fn set(&mut self, p: Position, c: char) {
        self.state[p.row][p.col] = c;
    }

    pub fn play_word(&mut self, p: Position, word: String, dir: Direction, force: bool) -> bool {
        // self.affected.clear();
        let mut current = p.clone();
        let mut aff = Vec::new();

        for c in word.chars() {
            let uc = c.to_uppercase().next().unwrap();
            if force {
                if uc != '.' {
                    self.set(current, uc);
                }
            } else {
                match self.at_position(current) {
                    '.' | '*' | '-' | '+' | '^' | '#' => self.set(current, uc),
                    _ => return false,
                }
            }

            if c.is_lowercase() {
                self.blanks.push(current);
            }

            for p in current.neighbors() {
                aff.push(p);
            }

            if !(current.tick(dir)) && !force {
                return false;
            }
        }

        self.affected.clear();

        for p in aff {
            for d in Direction::iter() {
                let mut np = p.clone();
                self.affected.push(np);
                while np.tick(*d) && self.is_letter(np) {
                    self.affected.push(np);
                }

                for c in np.neighbors() {
                    self.affected.push(c);
                }

                np = p.clone();
                while np.tick_opp(*d) && self.is_letter(np) {
                    self.affected.push(np);
                }

                for c in np.neighbors() {
                    self.affected.push(c);
                }
            }
        }

        self.affected = self
            .affected
            .iter()
            .filter(|x| !self.is_letter(**x))
            .cloned()
            .collect();

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

        for (i, l) in ALPH.chars().enumerate() {
            if l == '?' {
                continue;
            }
            let old = self.at_position(p);
            self.set(p, l);
            cross[i] = self.valid(&dir);
            self.set(p, old);
        }

        cross
    }

    pub fn valid(&self, d: &Direction) -> bool {
        // TODO check connectedness
        // self.get_words(*dir).iter().all(|x| self.dict.check_word(&x.word))
        // println!("chekin {}", self);
        let mut marked: [bool; 225] = [false; 225];
        for row in 0..15 {
            for col in 0..15 {
                let p = Position { row, col };
                if !marked[p.to_int()] && self.is_letter(p) {
                    let mut curr = p.clone();
                    // let mut node = self.trie.hashroot();
                    // let mut len = 0;
                    // let mut c = self.at_position(curr);
                    // while !"#^+-*.".contains(c) {
                    //     // println!("Following {}", c);
                    //     if let Some(nnode) = self.trie.follow(node, c) {
                    //         // println!("\tfound");
                    //         node = nnode;
                    //         len += 1;
                    //     } else {`
                    //         // println!("\tnot found");
                    //         return false
                    //     }
                    let mut word = String::with_capacity(10);
                    while self.is_letter(curr) {
                        word.push(self.at_position(curr));
                        marked[curr.to_int()] = true;
                        if !curr.tick(*d) {
                            break;
                        }
                        // c = self.at_position(curr);
                    }

                    if word.len() > 1 {
                        if !self.dict.check_word(&word) {
                            return false;
                        }
                    }

                    // if len > 1 {
                    //     if let Some(_terminal) = self.trie.follow(node, '@') {
                    //         // println!("\t terminal");
                    //     } else {
                    //         // println!("\t not terminal");
                    //         return false
                    //     }
                    // }
                }
            }
        }

        true
    }

    pub fn anchors(&self) -> Vec<Position> {
        let mut result = Vec::new();

        for p in positions().iter() {
            if !self.is_letter(*p) {
                continue;
            }
            for n in p.neighbors() {
                if !self.is_letter(n) {
                    result.push(n);
                }
            }
        }

        result
    }

    pub fn is_anchor(&self, p: Position) -> bool {
        if self.is_letter(p) {
            return false;
        }

        for n in p.neighbors() {
            if self.is_letter(n) {
                return true;
            }
        }

        false
    }

    pub fn save_state(&self) -> S {
        (
            self.state.clone(),
            self.blanks.clone(),
            self.cross_checks.clone(),
        )
    }

    pub fn set_state(&mut self, state: &S) {
        let (s, b, c) = state;
        self.state = (*s).clone();
        self.blanks = (*b).clone();
        self.cross_checks = (*c).clone();
    }
}

impl Board {
    pub fn gen_all_moves(&mut self, rack: &Vec<char>) -> Vec<Move> {
        // println!("{:?}", self.affected);
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
        let mut cross_sums: [[i32; 225]; 2] = [array_init(|_| 0), array_init(|_| 0)];
        for p in positions().iter() {
            for (di, d) in Direction::iter().enumerate() {
                if self.affected.contains(p) {
                    // only reevaluate for newly affected squares
                    self.cross_checks[di][p.to_int()] = chars(self.valid_at(*p, *d));
                    // note: requires mutability. also expensive method.
                }

                let mut p_sums = p.clone(); // start at position
                let mut score = 0;
                let mut found = false;
                while p_sums.tick(*d) && self.is_letter(p_sums) {
                    // go forward until find non letter
                    found = true; // found a letter so there are cross-sums. this is to distinguish finding a blank from finding nothing.

                    if !self.blanks.contains(&p_sums) {
                        // blanks are worth 0
                        score += self.bag.score(self.at_position(p_sums));
                    }
                }
                p_sums = p.clone();
                while p_sums.tick_opp(*d) && self.is_letter(p_sums) {
                    // go backwards until find non letter
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

        // println!("{}", self);

        // Initialize some necessary variables.
        let root = self.trie.root();

        let rword = to_word(&rack); // convert it to a vector-word (see utils.rs) for ease of insertion and deletion.

        let n_center = !self.is_letter(Position { row: 7, col: 7 }); // if we need to play at the center or not

        /*
        The following algorithm starts off the left-part algorithms at each anchor square.
        Note that it is repeated twice; the second loop is simply the same operation but on the
        transpose of the board.
        */
        let mut d = Direction::Across;
        let mut di_opp = 1; // opposite direction, for indexing cross_checks

        let mut last_anchor_col; // last column of the anchor to calculate the distance between current and last anchor square

        for row in 0..15 {
            // iterate over positions
            last_anchor_col = 0; // reset last column
            for col in 0..15 {
                let p = Position { row, col };
                if self.is_anchor(p) || (n_center && (p.col == 7 && p.row == 7)) {
                    // operate on either anchor, or middle piece *if* center is not *
                    let mut np = p.clone();
                    if np.tick_opp(d) && self.is_letter(np) {
                        // if left is a letter, use left part already on board
                        self.left_on_board(
                            np,
                            &rword,
                            &self.cross_checks[di_opp],
                            d,
                            &mut result,
                            &cross_sums[di_opp],
                        );
                    } else {
                        // make left part from rack. note that these arguments are really ugly but all fairly necessary (some for debugging)
                        self.left_part(
                            p,
                            Vec::new(),
                            root,
                            &rword,
                            &self.cross_checks[di_opp],
                            d,
                            &mut result,
                            (p.col - last_anchor_col + 1).try_into().unwrap(),
                            String::new(),
                            p,
                            p,
                            &cross_sums[di_opp],
                        );
                    }
                    last_anchor_col = p.col;
                }
            }
        }

        // Repetition with the transpose.
        d = Direction::Down;
        di_opp = 0;
        for col in 0..15 {
            // todo rayon
            last_anchor_col = 0;
            for row in 0..15 {
                let p = Position { row, col };
                if self.is_anchor(p) || (n_center && (p.col == 7 && p.row == 7)) {
                    let mut np = p.clone();
                    if np.tick_opp(d) && self.is_letter(np) {
                        self.left_on_board(
                            np,
                            &rword,
                            &self.cross_checks[di_opp],
                            d,
                            &mut result,
                            &cross_sums[di_opp],
                        );
                    } else {
                        self.left_part(
                            p,
                            Vec::new(),
                            root,
                            &rword,
                            &self.cross_checks[di_opp],
                            d,
                            &mut result,
                            (p.row - last_anchor_col + 1).try_into().unwrap(),
                            String::new(),
                            p,
                            p,
                            &cross_sums[di_opp],
                        );
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
                    result.push(Move {
                        word: j.iter().collect(),
                        position: Position { row: 0, col: 0 },
                        direction: Direction::Down,
                        score: 0,
                        evaluation: *self.dict.evaluate(&jw).expect(&format!("{:?}", &jw)),
                        typ: Type::Exch,
                    });
                }
            }
        }

        result
    }

    // todo: fix ugly arguments
    fn left_on_board(
        &self,
        position: Position,
        rack: &Vec<usize>,
        cross_checks: &[Vec<char>; 225],
        direction: Direction,
        moves: &mut Vec<Move>,
        cross_sums: &[i32; 225],
    ) {
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
            if !(np.tick_opp(direction) && self.is_letter(np)) {
                // end method
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
                if !self.is_letter(np)
                    || !((np.row == 0 && direction == Direction::Down)
                        || (np.col == 0 && direction == Direction::Across))
                {
                    nnnp.tick(direction);
                }
                // pass to extend-right
                self.extend_right(
                    &Vec::new(),
                    self.trie.seed(&word),
                    nnp,
                    cross_checks,
                    direction,
                    rack.to_vec(),
                    moves,
                    &word.iter().collect(),
                    nnnp,
                    nnp,
                    cross_sums,
                );
                return;
            }
        }
    }

    fn left_part(
        &self,
        position: Position,
        part: Vec<char>,
        node: NodeIndex,
        rack: &Vec<usize>,
        cross_checks: &[Vec<char>; 225],
        direction: Direction,
        moves: &mut Vec<Move>,
        limit: u32,
        word: String,
        curr_pos: Position,
        real_pos: Position,
        cross_sums: &[i32; 225],
    ) {
        /*
        This method extends the current move as far left as possible using only tiles on the rack.
        Therefore, if it would hit a tile on the board, it returns instead.
        Since the tile-on-board calculation is mildly expensive, we pass this method how far it can go
        (its limit). Once this limit is 0, it will return. Otherwise, it will recurse - for every tile,
        go left from that tile with limit - 1.

        The speed of this method is vastly improved with the use of a GADDAG, as it will therefore only
        check valid left parts.
        */

        // Check if this is a valid left part; if it is, extend right.
        if let Some(seed) = self.trie.follow(node, '#') {
            self.extend_right(
                &part,
                seed,
                real_pos,
                cross_checks,
                direction,
                rack.to_vec(),
                moves,
                &word,
                curr_pos,
                real_pos,
                cross_sums,
            );
        }

        if limit > 0 {
            // can still travel in the given direction
            let mut cp = position.clone(); // current position
            if cp.tick_opp(direction) {
                // try to move left
                // todo rayon
                //                let ms = self.trie.nexts(node).par_iter().map(|(next, nnode)| {
                //                    let mut mymoves = vec![];
                for (next, _) in self.trie.nexts(node) {
                    // iterate over nexts
                    if let Some(i) = ALPH.find(next) {
                        // get index of character (needed because rack is stored as bitword, see utils::to_word
                        // Valid letters must be both on the rack and in the cross checks.
                        if rack[i] > 0 && cross_checks[cp.to_int()].contains(&next) {
                            let mut new_rack = rack.clone();
                            new_rack[i] -= 1; // remove the letter from the rack

                            // add the letter to the left part and the word
                            let mut new_part = part.clone();
                            new_part.push(next);
                            let new_word = next.to_string() + &word;

                            let mut ccp = cp.clone(); // new starting position
                            ccp.tick_opp(direction);

                            if !self.is_letter(ccp) {
                                // final check to confirm we won't hit a letter
                                if let Some(nnode) = self.trie.follow(node, next) {
                                    // see if it can form a valid left part
                                    // recurse
                                    self.left_part(
                                        cp,
                                        new_part,
                                        nnode,
                                        &new_rack,
                                        cross_checks,
                                        direction,
                                        moves,
                                        limit - 1,
                                        new_word,
                                        cp,
                                        real_pos,
                                        cross_sums,
                                    );
                                }
                            }
                        }
                    }
                    //                    mymoves
                    //                }).collect::<Vec<Vec<Move>>>();
                    //
                    //                for i in ms { // todo do better
                    //                    moves.extend(i);
                    //                }
                }
            }

            if rack[26] > 0 {
                // have a blank
                // If we have a blank, we apply the algorithm above, but for each letter the blank could be
                let mut new_rack = rack.clone();
                new_rack[26] -= 1; // remove the blank

                let mut cp = position.clone();
                if cp.tick_opp(direction) {
                    let mut ccp = cp.clone();
                    ccp.tick_opp(direction);
                    if !self.is_letter(ccp) {
                        for (c, nnode) in self.trie.nexts(node) {
                            if cross_checks[cp.to_int()].contains(&c) {
                                // todo make bools?
                                let mut new_part = part.clone();
                                new_part.push(c);

                                let new_word = c.to_lowercase().to_string() + &word;

                                self.left_part(
                                    cp,
                                    new_part,
                                    nnode,
                                    &new_rack,
                                    cross_checks,
                                    direction,
                                    moves,
                                    limit - 1,
                                    new_word,
                                    cp,
                                    real_pos,
                                    cross_sums,
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    fn extend_right(
        &self,
        part: &Vec<char>,
        node: NodeIndex,
        position: Position,
        cross_checks: &[Vec<char>; 225],
        direction: Direction,
        rack: Vec<usize>,
        moves: &mut Vec<Move>,
        word: &String,
        start_pos: Position,
        anchor: Position,
        cross_sums: &[i32; 225],
    ) {
        /*
        The heart of the algorithm, extend-right attempts to place all moves from a given left part.
        This method also recurses.

        When it hits a letter, it simply puts it in the right part and moves on. Therefore, a right part
        can mix tiles on the rack and on the board, unlike a left part.

        todo: code duplication
        */
        if !self.is_letter(position) {
            // found an empty tile
            if position != anchor {
                // not the anchor so we can check if it's a move
                if let Some(_terminal) = self.trie.can_next(node, '@') {
                    // move forms a valid word
                    // return move
                    let mut m = Move {
                        word: word.to_string(),
                        position: start_pos,
                        direction,
                        score: 0,
                        evaluation: *self.dict.evaluate(&rack).expect(&format!("{:?}", &rack)),
                        typ: Type::Play,
                    };
                    m.score = self.score(&m, cross_sums); // score move
                    moves.push(m); // add move to list
                }
            }

            for (next, nnode) in self.trie.nexts(node) {
                // iterate over all possible nexts from the word
                if let Some(unext) = ALPH.find(next) {
                    if cross_checks[position.to_int()].contains(&next) {
                        // confirm that next is valid in the position todo: blanks here?
                        if rack[unext] > 0 || rack[26] > 0 {
                            // confirm that next is on rack, or rack has a blank. todo: reduce left-part code to look like this
                            let mut np = part.clone(); // add to part
                            np.push(next);
                            let mut nr = rack.clone(); // remove from rack
                            let mut snext = next.to_string();
                            if rack[unext] > 0 {
                                nr[unext] -= 1;
                            } else {
                                nr[26] -= 1;
                                snext = next.to_lowercase().to_string();
                            }
                            let mut npp = position.clone();

                            let nword = &(word.to_owned() + &snext); // add to word

                            if npp.tick(direction) {
                                // try to extend right
                                self.extend_right(
                                    &np,
                                    nnode,
                                    npp,
                                    cross_checks,
                                    direction,
                                    nr,
                                    moves,
                                    nword,
                                    start_pos,
                                    anchor,
                                    cross_sums,
                                );
                            } else if let Some(_terminal) = self.trie.can_next(nnode, '@') {
                                // try to place move
                                let mut m = Move {
                                    word: nword.to_string(),
                                    position: start_pos,
                                    direction,
                                    score: 0,
                                    evaluation: *self
                                        .dict
                                        .evaluate(&nr)
                                        .expect(&format!("{:?}", &nr)),
                                    typ: Type::Play,
                                };
                                m.score = self.score(&m, cross_sums);
                                moves.push(m);
                            }
                        }
                    }
                }
            }
        } else {
            // found a letter on the board
            let next = self.at_position(position);
            let mut np = part.clone();
            np.push(next);
            let mut npp = position.clone();

            let nword = &(word.to_owned() + &next.to_string());

            if let Some(next_node) = self.trie.follow(node, next) {
                if npp.tick(direction) {
                    // try to extend right
                    self.extend_right(
                        &np,
                        next_node,
                        npp,
                        cross_checks,
                        direction,
                        rack,
                        moves,
                        nword,
                        start_pos,
                        anchor,
                        cross_sums,
                    );
                } else if let Some(_terminal) = self.trie.can_next(next_node, '@') {
                    // try to place move
                    let mut m = Move {
                        word: nword.to_string(),
                        position: start_pos,
                        direction,
                        score: 0,
                        evaluation: *self.dict.evaluate(&rack).expect(&format!("{:?}", &rack)),
                        typ: Type::Play,
                    };
                    m.score = self.score(&m, cross_sums);
                    moves.push(m);
                }
            }
        }
    }

    pub fn reals(&self, m: &Move) -> Vec<char> {
        let mut result = Vec::new();
        for (curr_pos, i) in m.iter() {
            if !self.is_letter(curr_pos) {
                result.push(i);
            }
        }

        result
    }

    pub fn format(&self, m: &Move, human: bool) -> String {
        let mut res = String::new();
        for (curr_pos, i) in m.iter() {
            if !self.is_letter(curr_pos) {
                res.push(i);
            } else {
                if human {
                    res.push('(');
                    if self.blanks.contains(&curr_pos) {
                        res.push(i.to_lowercase().nth(0).unwrap());
                    } else {
                        res.push(i);
                    }
                    res.push(')');
                } else {
                    res.push('.');
                }
            }
        }

        res.replace(")(", "")
    }

    pub fn score(&self, m: &Move, cross_sums: &[i32; 225]) -> i32 {
        let mut true_score = 0;
        let mut total_cross_score = 0;
        let mut true_mult = 1;
        let mut n_played = 0;
        for (curr_pos, i) in m.iter() {
            let mut cross_mult = 1;
            let mut tile_mult = 1;
            match self.at_position(curr_pos) {
                '*' | '^' => {
                    true_mult *= 2;
                    cross_mult *= 2
                }
                '#' => {
                    true_mult *= 3;
                    cross_mult *= 3
                }
                '+' => {
                    tile_mult *= 3;
                }
                '-' => {
                    tile_mult *= 2;
                }
                '.' => {}
                _ => {
                    cross_mult = 0;
                    n_played += 1;
                } // char was already there, so don't score old words
            }

            let mut curr_score = 0;
            if !(i.is_lowercase() || self.blanks.contains(&curr_pos)) {
                curr_score = self.bag.score(i) * tile_mult;
            }

            let cross_sum = cross_sums[curr_pos.to_int()];

            if cross_sum >= 0 {
                let cross_score = curr_score + cross_sum;
                total_cross_score += cross_mult * cross_score;
            }

            true_score += curr_score;
        }

        let mut score = true_mult * true_score + total_cross_score;

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
        for row in ALPH.chars().take(15) {
            write!(f, "{}", format!(" {} |", row)).expect("fail");
        }
        write!(f, "\n{}\n", sep).expect("fail");

        // let a = &self.affected;

        for (num, row) in self.state.iter().enumerate() {
            write!(f, "| {} |", format!("{:0>2}", num + 1)).expect("fail");
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
                    _ => {
                        if self.blanks.contains(&Position { row: num, col }) {
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
