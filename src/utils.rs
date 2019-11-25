use std::collections::HashMap;
use std::collections::HashSet;
use std::slice::Iter;
use std::cmp::PartialEq;
use std::ops::Range;
use itertools::Itertools;
use std::cmp::Ordering;
use std::fmt;

pub trait ItemRemovable<T> {
    fn _remove_item(&mut self, some_x: T) -> T;
}

impl<T: PartialEq> ItemRemovable<T> for Vec<T> { // implementation of unstable feature
    fn _remove_item(&mut self, some_x: T) -> T {
        self.remove(self.iter().position(|x| *x == some_x).unwrap())
    }
}

pub static alph: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ?";

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
        match self {
            Direction::Down => return String::from("Down"),
            Direction::Across => return String::from("Across")
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

    pub fn add(&self, n: i32, d: Direction) -> Option<Position> {
        let mut p = self.clone();
        if n < 0 {
            for i in 0..(-n) {
                if !p.tick_opp(d) { return None }
            }
        } else {
            for i in 0..n {
                if !p.tick(d) { return None }
            }
        }

        Some(p)
    }

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
        
    pub fn to_str(&self) -> String {
        let mut s: String = alph.chars().nth(self.col).unwrap().to_string();
        s += &(self.row + 1).to_string();
        s
    }
}

pub fn chars(arr: [bool; 26]) -> Vec<char> {
    alph.chars()
        .zip(arr.iter())
        .filter(|&(a, b)| *b)
        .map(|(a, b)| a)
        .collect()
}

pub fn to_word(arr: &Vec<char>) -> Vec<usize> {
    alph.chars()
        .map(|x| arr.iter().filter(|&y| *y == x).count())
        .collect()
}

static POS: Range<usize> = 0..15;

pub fn positions() -> Vec<Position> {
    iproduct!(POS.clone(), POS.clone()).map(|(row, col)| Position { row, col }).collect::<Vec<Position>>()
}

#[derive(Debug)]
pub struct Move {
    pub word: String,
    pub position: Position,
    pub direction: Direction,
    pub score: i32,
    pub evaluation: f32
}

impl Move {
    pub fn cmp(x: &&Move, y: &&Move) -> Ordering {
        let v1 = (x.score as f32) + x.evaluation;
        let v2 = (y.score as f32) + y.evaluation;

        if v1 > v2 {
            return Ordering::Greater
        } else if v1 < v2 {
            return Ordering::Less
        } else {
            return Ordering::Equal
        }
    }
}

impl Move {
    pub fn of(m: &Move) -> Move {
        Move {
            word: m.word.clone(),
            position: m.position.clone(),
            direction: m.direction.clone(),
            score: m.score.clone(),
            evaluation: m.evaluation.clone()
        }
    }
}