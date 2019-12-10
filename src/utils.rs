use std::slice::Iter;
use std::ops::Range;
use std::cmp::Ordering;

pub trait ItemRemovable<T> {
    fn _remove_item(&mut self, some_x: T) -> T;
}

impl<T: PartialEq> ItemRemovable<T> for Vec<T> { // implementation of unstable feature
    fn _remove_item(&mut self, some_x: T) -> T {
        self.remove(self.iter().position(|x| *x == some_x).unwrap())
    }
}

pub static ALPH: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ?";

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Across,
    Down
}

impl Direction {
    pub fn iter() -> Iter<'static, Direction> {
        static D: [Direction; 2] = [Direction::Across, Direction::Down];
        D.iter()
    }

    pub fn to_str(&self) -> String {
        match self { // todo cooler arrows http://xahlee.info/comp/unicode_arrows.html
            Direction::Down => return String::from("↓"),
            Direction::Across => return String::from("→")
        }
    }

    pub fn flip(&self) -> Direction {
        match self {
            Direction::Across => Direction::Down,
            Direction::Down => Direction::Across
        }
    }
}

impl Position {
    pub fn tick(&mut self, d: Direction) -> bool {
        match d {
            Direction::Across => {
                if self.col < 14 { // note: don't have to check for 0-bound because usizes are positive
                    self.col += 1;
                } else {
                    return false;
                }
            },
            Direction::Down => {
                if self.row < 14 {
                    self.row += 1;
                } else {
                    return false;
                }
            }
        }
        true
    }

    pub fn tick_opp(&mut self, d: Direction) -> bool {
        match d {
            Direction::Across => {
                if 0 < self.col { // note: don't have to check for 0-bound because usizes are positive
                    self.col -= 1;
                } else {
                    return false;
                }
            },
            Direction::Down => {
                if 0 < self.row {
                    self.row -= 1;
                } else {
                    return false;
                }
            }
        }
        true
    }

    // pub fn add(&self, n: i32, d: Direction) -> Option<Position> {
    //     let mut p = self.clone();
    //     if n < 0 {
    //         for _ in 0..(-n) {
    //             if !p.tick_opp(d) { return None }
    //         }
    //     } else {
    //         for _ in 0..n {
    //             if !p.tick(d) { return None }
    //         }
    //     }

    //     Some(p)
    // }

    pub fn neighbors(&self) -> Vec<Position> {
        let mut result = Vec::new();

        if self.col < 14 { result.push(Position { row: self.row, col: self.col + 1 }); }
        if self.row < 14 { result.push(Position { row: self.row + 1, col: self.col }); }

        if self.col > 0  { result.push(Position { row: self.row, col: self.col - 1 }); }
        if self.row > 0  { result.push(Position { row: self.row - 1, col: self.col }); }

        result
    }

    pub fn to_int(&self) -> usize {
        self.row * 15 + self.col
    }
        
    pub fn to_str(&self, dir: Direction) -> String {
        let a = ALPH.chars().nth(self.col).unwrap().to_string();
        let b = (self.row + 1).to_string();
        match dir {
            Direction::Across => return b + &a,
            Direction::Down   => return a + &b
        }
    }
}

pub fn chars(arr: [bool; 26]) -> Vec<char> {
    ALPH.chars()
        .zip(arr.iter())
        .filter(|&(_, b)| *b)
        .map(|(a, _)| a)
        .collect()
}

pub fn to_word(arr: &Vec<char>) -> Vec<usize> {
    ALPH.chars()
        .map(|x| arr.iter().filter(|&y| *y == x).count())
        .collect()
}

static POS: Range<usize> = 0..15;

pub fn positions() -> Vec<Position> {
    iproduct!(POS.clone(), POS.clone()).map(|(row, col)| Position { row, col }).collect::<Vec<Position>>()
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Play,
    Exch
}

#[derive(Debug)]
pub struct Move {
    pub word: String,
    pub position: Position,
    pub direction: Direction,
    pub score: i32,
    pub evaluation: f32,
    pub typ: Type
}

impl Move {
    fn _cmp(x: &&Move, y: &&Move, w1: &f32, w2: &f32) -> Ordering {
        let v1 = w1 * (x.score as f32) + w2 * x.evaluation;
        let v2 = w1 * (y.score as f32) + w2 * y.evaluation;

        if v1 > v2 {
            return Ordering::Greater
        } else if v1 < v2 {
            return Ordering::Less
        } else {
            return Ordering::Equal
        }
    }

    pub fn cmp(x: &&Move, y: &&Move) -> Ordering {
        Move::_cmp(x, y, &1.0, &1.0)
    }
    
    pub fn cmp_with(a: f32, b: f32) -> impl Fn(&&Move, &&Move) -> Ordering {
        move |x: &&Move, y: &&Move| Move::_cmp(x, y, &a, &b)
    }
}

impl Move {
    pub fn of(m: &Move) -> Move {
        Move {
            word: m.word.clone(),
            position: m.position.clone(),
            direction: m.direction.clone(),
            score: m.score.clone(),
            evaluation: m.evaluation.clone(),
            typ: m.typ.clone()
        }
    }

    pub fn none() -> Move {
        Move {
            word: String::new(),
            position: Position { row: 0, col: 0 },
            direction: Direction::Down,
            score: 0, evaluation: 0.0, typ: Type::Play
        }
    }

    pub fn complement(&self, rack: &Vec<char>) -> Vec<char> {
        let mut nr = rack.clone();

        for c in self.word.chars() {
            nr._remove_item(c);
        }

        nr
    }
    
    pub fn exch(&self) -> bool {
        self.typ == Type::Exch
    }
}