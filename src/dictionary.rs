use crate::utils::alph;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
// use daggy::*;
use petgraph::Graph;
use petgraph::Direction;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;

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
            let mut sub: HashMap<char, HashSet<String>> = HashMap::new();

            let i_node = trie.graph.add_node(i.clone());

            trie.graph.add_edge(trie.current, i_node, i.clone());

            for j in alph.chars() {
                let j_node = trie.graph.add_node(j.clone());

                trie.graph.add_edge(i_node, j_node, j.clone());

                let dipth: String = i.to_string() + &j.to_string();
                let filepath = format!("resources/{}.txt", dipth);

                let words: Vec<String> = fs::read_to_string(filepath)
                                .expect(&dipth)
                                .lines().map(String::from).collect();
                

                for word in words {
                    let mut last_node = j_node;
                    for c in word.chars() {
                        let next_node = trie.graph.add_node(c.clone());
                        trie.graph.add_edge(last_node, next_node, c.clone());
                        last_node = next_node;
                    }
                }

                return trie;
            }
        }

        trie
    }

    pub fn seed(&mut self, initial: Vec<char>) {
        println!("seeding");

        let nodes = self.graph.raw_nodes();
        let edges = self.graph.raw_edges();
        self.current = self.graph.node_indices().next().unwrap();
        for c in initial {
            for a in self.graph.edges_directed(self.current, Direction::Outgoing) {
                println!("{:?}", a); 
                for b in self.graph.edges_directed(edges[a.id().index()].target(), Direction::Outgoing) {
                    println!("\t{:?}", b); 
                    for d in self.graph.edges_directed(edges[b.id().index()].target(), Direction::Outgoing) {
                        println!("\t\t{:?}", d); 
                        // for e in self.graph.edges_directed(edges[d.id().index()].target(), Direction::Outgoing) {
                        //     println!("\t\t\t{:?}", e); 
                        // }
                    }
                }
            }
        }
    }

    pub fn next(&self) -> Vec<char> {
        Vec::new()
    }

    pub fn advance(&self, next: char) {
        
    }
}