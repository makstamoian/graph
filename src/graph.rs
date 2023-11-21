use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::time::Instant;
use serde_json;


// Nodes: 1, 2, 3, 4, 5
// Edge: 1 -> 2, 1 -> 3, 2 -> 4, 3 -> 4, 4 -> 5
//
// Adjacency Sets
//
// Map:
// 1 => Set[2, 3]
// 2 => Set[4]
// 3 => Set[4]
// 4 => Set[5]

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<u32, HashSet<u32>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: u32) {
        self.nodes.insert(node, HashSet::new());
    }

    pub fn add_edge(&mut self, source: u32, target: u32) {
        if source != target {
            self.nodes
                .entry(source)
                .and_modify(|adjacency_set: &mut HashSet<u32>| {
                    adjacency_set.insert(target);
                });
            self.nodes
                .entry(target)
                .and_modify(|adjacency_set: &mut HashSet<u32>| {
                    adjacency_set.insert(source);
                });
        }
    }

    pub fn drop_edge(&mut self, node_a: u32, node_b: u32) {
        self.nodes
            .entry(node_a)
            .and_modify(|adjacency_set: &mut HashSet<u32>| {
                adjacency_set.remove(&node_b);
            });
        self.nodes
            .entry(node_b)
            .and_modify(|adjacency_set: &mut HashSet<u32>| {
                adjacency_set.remove(&node_a);
            });
    }

    pub fn drop_node(&mut self, node: u32) {
        let adjacent_nodes = self.nodes[&node].clone();
        for adjacent in &adjacent_nodes {
            self.drop_edge(node, *adjacent);
        }
        self.nodes.remove(&node);
    }

    pub fn has_node(&self, node: u32) -> bool {
        return self.nodes.contains_key(&node);
    }

    pub fn has_edge(&self, node_a: u32, node_b: u32) -> bool {
        if self.has_node(node_a) && self.has_node(node_b) {
            return self.nodes[&node_a].contains(&node_b) && self.nodes[&node_b].contains(&node_a);
        }

        return false;
    }

    pub fn get_node_adjacents(&self, node: u32) -> &HashSet<u32> {
        return &self.nodes[&node];
    }

    pub fn get_leaf_nodes(&self) -> HashSet<u32> {
        let mut leaf_nodes: HashSet<u32> = HashSet::new();

        for node in self.nodes.keys() {
            if self.get_node_adjacents(*node).len() == 1 {
                leaf_nodes.insert(*node);
            }
        }

        return leaf_nodes;
    }

    pub fn depth_first_search (&self, node: u32) -> HashSet<u32> {

        let mut stack: VecDeque<u32> = VecDeque::new();
        let mut visited_nodes: HashSet<u32> = HashSet::new();
        
        let start = Instant::now();

        stack.push_back(node);

        while let Some(node_pop) = stack.pop_back() {
            if visited_nodes.contains(&node_pop) == false {
                visited_nodes.insert(node_pop);
                for adjacent in self.get_node_adjacents(node_pop).iter() {
                    stack.push_back(*adjacent);
                }
            }
        }
        
        let end = Instant::now();
        let duration = end.duration_since(start);
        println!("DFS elapsed time: {:#?}", duration);

        return visited_nodes;
    }

    pub fn breadth_first_search (&self, node: u32, target: u32) -> i32 {
        let mut queue: VecDeque<u32> = VecDeque::new();
        let mut visited_nodes: HashSet<u32> = HashSet::new();

        let start = Instant::now();

        visited_nodes.insert(node);
        queue.push_back(node);

        while let Some(node_pop) = queue.pop_front() {
            if node_pop == target {
                let end = Instant::now();
                let duration = end.duration_since(start);
                println!("BFS lapsed time: {:#?}", duration);

                return node_pop as i32;
            }

            for adjacent in self.get_node_adjacents(node_pop).iter() {
                queue.push_back(*adjacent);
                visited_nodes.insert(*adjacent);
            }

        }

        let end = Instant::now();
        let duration = end.duration_since(start);

        println!("BFS lapsed time: {:#?}", duration);

        return -1;

    }

    pub fn is_connected(&self) -> bool {
        let visited_nodes = self.depth_first_search(0);
        if visited_nodes.len() < self.nodes.len() {
            return false;
        }
        return true;
    }

    pub fn serialize(&self) -> String {        
        return serde_json::to_string(&self.nodes).unwrap();
    }

    pub fn clear(&mut self) -> &HashMap<u32, HashSet<u32>> {
        let _ = &self.nodes.clear();

        return &self.nodes;
    }
}
