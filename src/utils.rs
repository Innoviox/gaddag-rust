use std::collections::HashMap;
use std::fs;
use std::slice::Iter;
use std::cmp::PartialEq;
use std::ops::Range;

pub static alph: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub struct Dictionary {
    words: HashMap<char, HashMap<char, Vec<String>>>
}

impl Dictionary {
    pub fn default() -> Dictionary {
        let mut dict = Dictionary { words: HashMap::new() };

        for i in alph.chars() {
            let mut sub: HashMap<char, Vec<String>> = HashMap::new();

            for j in alph.chars() {
                let dipth: String = i.to_string() + &j.to_string();
                let filepath = format!("resources/{}.txt", dipth);

                let words = fs::read_to_string(filepath)
                                .expect(&dipth)
                                .lines().map(String::from).collect();
                
                sub.insert(j, words);
            }

            dict.words.insert(i, sub);
        }

        dict
    }

    pub fn check_word(&self, word: String) -> bool {
        let mut chars = word.chars();
        if let Some(c1) = chars.next() {
            if let Some(c2) = chars.next() {
                return self.words[&c1][&c2].contains(&word)
            }
        }
        false
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub row: usize,
    pub col: usize
}

#[derive(Copy, Clone)]
pub enum Direction {
    Across,
    Down
}

impl Direction {
    pub fn iter() -> Iter<'static, Direction> {
        static d: [Direction; 2] = [Direction::Across, Direction::Down];
        d.iter()
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

    pub fn neighbors(&self) -> Vec<Position> {
        let mut result = Vec::new();

        result.push(Position { row: self.row, col: self.col + 1 });
        result.push(Position { row: self.row, col: self.col - 1 });
        result.push(Position { row: self.row + 1, col: self.col });
        result.push(Position { row: self.row - 1, col: self.col });

        result
    }
}

pub fn chars(arr: [bool; 26]) -> Vec<char> {
    alph.chars()
        .zip(arr.iter())
        .filter(|&(a, b)| *b)
        .map(|(a, b)| a)
        .collect()
}

static pos: Range<usize> = 0..15;

pub fn positions() -> Vec<Position> {
    iproduct!(pos.clone(), pos.clone()).map(|(row, col)| Position { row, col }).collect::<Vec<Position>>()
}

