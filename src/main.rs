use std::collections::HashMap;
use std::vec::Vec;
use rand::seq::SliceRandom; 

#[derive(Debug)]
struct Bag {
    alph: [char; 27],
    amts: [i32; 27],
    values: [i32; 27],
    scores: HashMap<char, i32>,
    distribution: Vec<char>
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

    fn score(&self, c: char) -> i32 {
        match self.scores.get(&c) {
            Some(i) => *i,
            None => -1
        }
    }

    fn draw_tiles(&self, n: usize) -> Vec<&char> {
        let tiles: Vec<&char> = self.distribution
                                   .choose_multiple(&mut rand::thread_rng(), n)
                                   .collect();
        for i in tiles.iter() {
            self.distribution.remove_item(i);
        }
        tiles
    }
}

fn main() {
    let mut BAG = Bag::default();
    BAG.init();
    println!("Bag is: {:?}", BAG);
    println!("Score for z is: {}", BAG.score('z'));

    let rack = BAG.draw_tiles(7);
    println!("Rack is: {:?}", rack);
    println!("Bag is: {:?}", BAG.distribution);
}