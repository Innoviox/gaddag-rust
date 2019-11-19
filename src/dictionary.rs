use crate::utils::alph;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
// use daggy::*;
use petgraph::Graph;


pub struct Dictionary {
    words: HashMap<char, HashMap<char, HashSet<String>>>
}

impl Dictionary {
    pub fn default() -> Dictionary {
        let mut dict = Dictionary { words: HashMap::new() };

        for i in alph.chars() {
            let mut sub: HashMap<char, HashSet<String>> = HashMap::new();

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

    pub fn check_word(&self, word: &String) -> bool {
        let mut chars = word.chars();
        if let Some(c1) = chars.next() {
            if let Some(c2) = chars.next() {
                return self.words[&c1][&c2].contains(word)
            }
        }
        false
    }
}

pub struct Trie {
    pub graph: Graph<char, u32>
}

impl Trie {
    pub fn default() -> Trie {
        let mut trie = Trie { graph: Graph::new() };

        for i in alph.chars() {
            let mut sub: HashMap<char, HashSet<String>> = HashMap::new();

            let i_node = trie.graph.add_node(i);

            for j in alph.chars() {
                let j_node = trie.graph.add_node(j);

                trie.graph.add_edge(i_node, j_node, 0);

                let dipth: String = i.to_string() + &j.to_string();
                let filepath = format!("resources/{}.txt", dipth);

                let words: Vec<String> = fs::read_to_string(filepath)
                                .expect(&dipth)
                                .lines().map(String::from).collect();
                

                for word in words {
                    let mut last_node = j_node;
                    for c in word.chars() {
                        let next_node = trie.graph.add_node(c);
                        trie.graph.add_edge(last_node, next_node, 0);
                        last_node = next_node;
                    }
                }

                return trie;
            }
        }

        trie
    }

    // pub fn seed(initial: Vec<char>) {

    // }

    // pub fn next() -> Vec<char> {

    // }

    // pub fn advance(next: char) {
        
    // }
}