use std::collections::HashMap;
use std::vec::Vec;
use rand::seq::SliceRandom; 
use std::cmp::PartialEq;

trait ItemRemovable<T> {
    fn remove_item(&mut self, some_x: T) -> T;
}

impl<T: PartialEq> ItemRemovable<T> for Vec<T> { // implementation of unstable feature
    fn remove_item(&mut self, some_x: T) -> T {
        self.remove(self.iter().position(|x| *x == some_x).unwrap())
    }
}


#[derive(Debug)]
pub struct Bag {
    pub alph: [char; 27],
    pub amts: [i32; 27],
    pub values: [i32; 27],
    pub scores: HashMap<char, i32>,
    pub distribution: Vec<char>
}

impl Bag {
    pub fn default() -> Bag {
        Bag {
            alph: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 
                'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '?'],
            amts: [9, 2, 2, 4, 12, 2, 3, 2, 9, 1, 1, 4, 2, 6, 8, 2, 1, 6, 4, 6, 4, 2, 2, 1, 2, 1, 2],
            values: [1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10, 0],
            scores: HashMap::new(), 
            distribution: Vec::new()
        }
    }

    pub fn init(&mut self) {
        for (i, &c) in self.alph.iter().enumerate() {
            self.scores.insert(c, self.values[i]);
        }

        for (i, &c) in self.alph.iter().enumerate() {
            for _ in 0..self.amts[i] {
                self.distribution.push(c);
            }
        }
    }

    pub fn score(&self, c: char) -> i32 {
        match self.scores.get(&c) {
            Some(i) => *i,
            None => -1
        }
    }
    
    pub fn draw_tiles(&mut self, n: usize) -> Vec<char> {
        let tiles: Vec<char> = self.distribution
                                .choose_multiple(&mut rand::thread_rng(), n)
                                .cloned().collect();
        for i in tiles.iter() {
            self.distribution.remove_item(*i);
        }
        tiles
    }
}