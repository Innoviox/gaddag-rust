use std::collections::HashMap;
use std::vec::Vec;

#[derive(Debug)]
struct Bag {
    alph: [char; 27],
    amts: [i32; 27],
    values: [i32; 27],
    scores: HashMap<char, i32>,
    distribution: Vec<char>
}

impl Bag {
    fn default() -> Bag {
        Bag {
            alph: ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 
                'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '?'],
            amts: [9, 2, 2, 4, 12, 2, 3, 2, 9, 1, 1, 4, 2, 6, 8, 2, 1, 6, 4, 6, 4, 2, 2, 1, 2, 1, 2],
            values: [1, 3, 3, 2, 1, 4, 2, 4, 1, 8, 5, 1, 3, 1, 1, 3, 10, 1, 1, 1, 1, 4, 4, 8, 4, 10, 0],
            scores: HashMap::new(), 
            distribution: Vec::new()
        }
    };

    pub fn init(&mut self) {
        for (i, &c) in self.alph.iter().enumerate() {
            self.scores.insert(c, self.values[i]);
        }
        // self.distribution = 
    }

    fn score(&self, c: char) -> i32 {
        match self.scores.get(&c) {
            Some(i) => *i,
            None => -1
        }
    }

    // fn draw_tiles(&self, n: i32) -> i32 {
    //     if let d = self.distribution {

    //     }
    //     self.init().draw_tiles(n)
    // }
}

fn main() {
    let mut BAG = Bag::default();
    BAG.init();
    println!("Bag is: {:?}", BAG);
    println!("Score for a is: {}", BAG.score('a'));
}