use crate::utils::ALPH;
use crate::utils::to_word;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use petgraph::{Graph, Directed}; // todo use daggy?
use petgraph::Direction;
use petgraph::graph::{NodeIndex, Edges};
use petgraph::visit::EdgeRef;
use indicatif::ProgressBar;
use indicatif::ProgressIterator;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

fn load_from_file<T: DeserializeOwned + Serialize>(file: &str, callback: fn () -> T) -> T {
    match fs::read(file) {
        Ok(b) => {
            println!("Loaded from file {}", file);
            bincode::deserialize(&b).unwrap()
        },
        Err(_) => {
            let t = callback();
            let serialized = bincode::serialize(&t).unwrap();
            match fs::write(file, &serialized) {
                Ok(_) => { println!("Saving successful"); }
                Err(e) => { println!("error {}", e); }
            };
            t
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Dictionary {
    words: HashMap<char, HashMap<char, HashSet<String>>>,
    leaves: HashMap<Vec<usize>, f32>
}

impl Dictionary {
    pub fn default() -> Dictionary {
        load_from_file("dict.ser", || {
            let mut dict = Dictionary { words: HashMap::new(), leaves: HashMap::new() };
            for i in ALPH.chars().progress() {
                if i == '?' { continue } 
                let mut sub: HashMap<char, HashSet<String>> = HashMap::new();
    
                for j in ALPH.chars() {
                    if j == '?' { continue } 
                    let dipth: String = i.to_string() + &j.to_string();
                    let filepath = format!("resources/{}.txt", dipth);
    
                    let words = fs::read_to_string(filepath)
                                    .expect(&dipth)
                                    .lines().map(String::from).collect();
                    
                    sub.insert(j, words);
                }
                dict.words.insert(i, sub);
            }
    
            let bar = ProgressBar::new(40);
    
            dict.leaves = fs::read_to_string("resources/leaves.txt").expect("No leaves file")
                                .lines().map(String::from).collect::<Vec<String>>()
                                .par_iter().map(|line| {
                let s: Vec<&str> = line.split(" ").collect();
                let word = to_word(&s[0].chars().collect());
                let eval = s[1].parse::<f32>().unwrap();  
                (word, eval)
            }).collect();
    
            dict.leaves.insert(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0.0);
            bar.finish();

            dict
        })
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

    pub fn evaluate(&self, rack: &Vec<usize>) -> Option<&f32> {
        self.leaves.get(rack)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Trie {
    pub graph: Graph<char, char>,
    pub current: NodeIndex<u32>,
}

impl Trie {
    pub fn default() -> Trie {
        load_from_file("trie.ser", || {
            let mut graph = Graph::new();
            let current = graph.add_node(' ');
            let mut trie = Trie { graph, current };

            let mut last_node;
            
            let extend = |t: &mut Trie, ln, c| {
                if let Some(new) = t.follow(ln, c) {
                    return new;
                } else {
                    let next_node = t.graph.add_node(c);
                    t.graph.add_edge(ln, next_node, c.clone());
                    return next_node;
                }
            };

            let dummy = extend(&mut trie, current, '#');

            for i in ALPH.chars().progress() {
                if i == '?' { continue }
                let i_node = extend(&mut trie, dummy, i);

                for j in ALPH.chars() {
                    if j == '?' { continue } 
                    let j_node = extend(&mut trie, i_node, j);

                    let dipth: String = i.to_string() + &j.to_string();
                    let filepath = format!("resources/{}.txt", dipth);

                    let words: Vec<String> = fs::read_to_string(filepath)
                                    .expect(&dipth)
                                    .lines().map(String::from).collect();
                    
                    for word in words {
                        last_node = j_node;

                        for c in word.chars().skip(2) {
                            last_node = extend(&mut trie, last_node, c);
                        }

                        extend(&mut trie, last_node, '@'); // EOW

                        for l in 1..word.len() {
                            last_node = current;
                            let v: Vec<char> = word.chars().take(l).collect();
                            for c in v.iter().rev() {
                                last_node = extend(&mut trie, last_node, *c);
                            }

                            last_node = extend(&mut trie, last_node, '#');

                            for c in word.chars().skip(l) {
                                last_node = extend(&mut trie, last_node, c);
                            }

                            extend(&mut trie, last_node, '@');
                        }
                    }
                }
            }

            trie
        })
    }
    
    pub fn root(&self) -> NodeIndex {
        self.graph.node_indices().next().unwrap()
    }

    pub fn hashroot(&self) -> NodeIndex {
        self.follow(self.root(), '#').unwrap()
    }

    pub fn seed(&self, initial: &Vec<char>) -> NodeIndex {        
        let edges = self.graph.raw_edges(); // todo: optimize away
        let mut current = self.hashroot();
        
        for c in initial {
            for a in self.graph.edges_directed(current, Direction::Outgoing) {
                let e = &edges[a.id().index()];
                if e.weight == *c {
                    current = e.target();
                    break;
                }
            }
        }

        current
    }

    pub fn can_next(&self, current: NodeIndex, next: char) -> Option<NodeIndex> {
        let edges = self.graph.raw_edges();
        for a in self.graph.edges_directed(current, Direction::Outgoing) {
            let e = &edges[a.id().index()];
            if e.weight == next {
                return Some(e.target())
            }
        }
        
        None
    }

    // for readability
    pub fn follow(&self, current: NodeIndex, next: char) -> Option<NodeIndex> {
        self.can_next(current, next)
    }

    // -> [Option<(char, NodeIndex)>; 26]
    pub fn _nexts(&self, current: NodeIndex) -> Edges<char, Directed, u32> {
        self.graph.edges_directed(current, Direction::Outgoing)
    }

    pub fn nexts(&self, current: NodeIndex) -> Vec<(char, NodeIndex)> {
        let edges = self.graph.raw_edges();
        self._nexts(current).map(|a| {
            let e = &edges[a.id().index()];
            (e.weight, e.target())
        }).collect()
    }
}