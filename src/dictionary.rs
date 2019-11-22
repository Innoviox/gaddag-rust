use crate::utils::alph;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
// use daggy::*;
use petgraph::Graph;
use petgraph::Direction;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use petgraph::graph::Edge;

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
    pub graph: Graph<char, char>,
    pub current: NodeIndex<u32>,
}



impl Trie {
    pub fn default() -> Trie {
        let mut graph = Graph::new();
        let current = graph.add_node(' ');
        let mut trie = Trie { graph, current };

        for i in alph.chars() {
            let i_node = trie.graph.add_node(i);

            trie.graph.add_edge(trie.current, i_node, i.clone());

            for j in alph.chars() {
                let j_node = trie.graph.add_node(j);

                trie.graph.add_edge(i_node, j_node, j.clone());

                let dipth: String = i.to_string() + &j.to_string();
                let filepath = format!("resources/{}.txt", dipth);

                let words: Vec<String> = fs::read_to_string(filepath)
                                .expect(&dipth)
                                .lines().map(String::from).collect();
                

                for word in words {
                    let mut last_node = j_node;

                    for c in word.chars().skip(2) {
                        if let Some(new) = trie.follow(last_node, c) {
                            last_node = new
                        } else {
                            let next_node = trie.graph.add_node(c);
                            trie.graph.add_edge(last_node, next_node, c.clone());
                            last_node = next_node;
                        }
                    }

                    let end_node = trie.graph.add_node('@'); // EOW
                    trie.graph.add_edge(last_node, end_node, '@');
                }

                // return trie;
            }
        }

        trie
    }
    
    pub fn root(&self) -> NodeIndex {
        self.graph.node_indices().next().unwrap()
    }

    pub fn seed(&self, initial: &Vec<char>) -> NodeIndex {        
        let edges = self.graph.raw_edges(); // todo: optimize away
        let mut current = self.root();
        
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
        let edges = self.graph.raw_edges(); // todo: optimize away
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