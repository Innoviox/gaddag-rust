use crate::utils::alph;
use crate::utils::to_word;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use petgraph::Graph;
use petgraph::Direction;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use indicatif::ProgressBar;
use indicatif::ProgressIterator;
use std::io;
use std::io::*;

pub struct Dictionary {
    words: HashMap<char, HashMap<char, HashSet<String>>>,
    leaves: HashMap<Vec<usize>, f32>
}

impl Dictionary {
    pub fn default() -> Dictionary {
        let mut dict = Dictionary { words: HashMap::new(), leaves: HashMap::new() };
        for i in alph.chars().progress() {
            if i == '?' { continue } 
            let mut sub: HashMap<char, HashSet<String>> = HashMap::new();

            for j in alph.chars() {
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
        let mut i = 0;
        for line in fs::read_to_string("resources/leaves.txt").expect("No leaves file").lines().map(String::from) {
            let s: Vec<&str> = line.split(" ").collect();
            let word = to_word(&s[0].chars().collect());
            let eval = s[1].parse::<f32>().unwrap();
            dict.leaves.insert(word, eval);
            i += 1;
            if i % 25000 == 0 { bar.inc(1); }
        }
        dict.leaves.insert(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 0.0);
        bar.finish();

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

    pub fn evaluate(&self, rack: &Vec<usize>) -> Option<&f32> {
        self.leaves.get(rack)
    }
}

pub struct Trie {
    pub graph: Graph<char, char>,
    pub current: NodeIndex<u32>,
}



impl Trie {
    pub fn default() -> Trie {
        let mut graph = Graph::new();
        let current = graph.add_node(' ');
        let mut trie = Trie { graph, current };

        let mut last_node = current;
        
        let mut extend = |t: &mut Trie, ln, c| {
            if let Some(new) = t.follow(ln, c) {
                return new;
            } else {
                let next_node = t.graph.add_node(c);
                t.graph.add_edge(ln, next_node, c.clone());
                return next_node;
            }
        };

        let dummy = extend(&mut trie, current, '#');

        for i in alph.chars().progress() {
            if i == '?' { continue }
            let i_node = extend(&mut trie, dummy, i);

            for j in alph.chars() {
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

                    // // println!("gaddagging {}", word);
                    for l in 1..word.len() {
                        last_node = current;
                        let v: Vec<char> = word.chars().take(l).collect();
                        // println!("\t recced {} found {:?}", l, v);
                        for c in v.iter().rev() {
                            // print!("{}", c);
                            last_node = extend(&mut trie, last_node, *c);
                        }
                        // println!("");

                        last_node = extend(&mut trie, last_node, '#');

                        for c in word.chars().skip(l) {
                            // print!("{}", c);
                            last_node = extend(&mut trie, last_node, c);
                        }

                        extend(&mut trie, last_node, '@');
                        // println!("");
                    }
                    // let mut guess = String::new();
                    // io::stdin().read_line(&mut guess).expect("Failed to read line");

                    // if word == "AARDVARKS".to_string() {
                    //     println!("{:?}", trie.nseed(&vec!['D', 'R', 'A', 'A', '#', 'V', 'A', 'R', 'K', 'S']));
                    // }
                }
                // return trie;
            }
        }

        // println!("{:?}", trie.nseed(&vec!['D', 'R', 'A', 'A', '#', 'V', 'A', 'R', 'K', 'S']));
        // println!("{:?}", trie.nseed(&vec!['R', 'E', 'T', 'A', 'E', '#', 'I', 'E', 'S']));

        trie
    }
    
    pub fn root(&self) -> NodeIndex {
        self.graph.node_indices().next().unwrap()
    }

    pub fn seed(&self, initial: &Vec<char>) -> NodeIndex {        
        let edges = self.graph.raw_edges(); // todo: optimize away
        let mut current = self.follow(self.root(), '#').unwrap();
        
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


    pub fn nseed(&self, initial: &Vec<char>) -> Option<NodeIndex> {        
        let mut current = self.root();
        
        for c in initial {
            if let Some(next) = self.follow(current, *c) {
                current = next;
            } else {
                return None
            }
        }

        Some(current)
    }

    pub fn nrseed(&self, initial: &Vec<char>) -> Option<NodeIndex> {
        let mut a = initial.clone();
        a.reverse();
        self.nseed(&a)
    }

    pub fn can_next(&self, current: NodeIndex, next: char) -> Option<NodeIndex> {
        let edges = self.graph.raw_edges();
        // println!("looking for {}", next);
        for a in self.graph.edges_directed(current, Direction::Outgoing) {
            let e = &edges[a.id().index()];
            if e.weight == next {
                // println!("found");
                return Some(e.target())
            }
        }
        
        None
    }

    pub fn can_back(&self, current: NodeIndex, back: char) -> Option<NodeIndex> {
        let edges = self.graph.raw_edges();
        for a in self.graph.edges_directed(current, Direction::Incoming) {
            let e = &edges[a.id().index()];
            if e.weight == back {
                return Some(e.target())
            }
        }
        
        None
    }

    // for readability
    pub fn follow(&self, current: NodeIndex, next: char) -> Option<NodeIndex> {
        self.can_next(current, next)
    }

    pub fn back_follow(&self, current: NodeIndex, back: char) -> Option<NodeIndex> {
        self.can_back(current, back)
    }


    pub fn nexts(&self, current: NodeIndex) -> Vec<char> { // debugging method
        let edges = self.graph.raw_edges();
        let mut res = Vec::new();
        for a in self.graph.edges_directed(current, Direction::Outgoing) {
            let e = &edges[a.id().index()];
            res.push(e.weight);
        }

        res
    }
}