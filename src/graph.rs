use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]

pub struct Graph {
    pub nodes: HashMap<u32, HashSet<u32>>,
}

impl Graph {
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

    pub fn get_nodes(&self) -> HashSet<u32> {
        let mut nodes: HashSet<u32> = HashSet::new();
        for key in self.nodes.keys() {
            nodes.insert(*key);
        }

        return nodes;
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

    fn depth_first_search_child(&self, node: u32, visited_nodes: &mut HashSet<u32>) {
        visited_nodes.insert(node);

        for new_node in &self.nodes[&node] {
            if visited_nodes.contains(new_node) {
                continue;
            }

            self.depth_first_search_child(*new_node, visited_nodes)
        }
    }

    pub fn depth_first_search(&self, node: u32) -> HashSet<u32> {
        let mut visited_nodes = HashSet::new();

        self.depth_first_search_child(node, &mut visited_nodes);
        return visited_nodes;
    }

    pub fn is_connected(&self) -> bool {
        let visited_nodes = self.depth_first_search(0);
        if visited_nodes.len() < self.nodes.len() {
            return false;
        }
        return true;
    }

    pub fn clear(&mut self) -> &HashMap<u32, HashSet<u32>> {
        let _ = &self.nodes.clear();

        return &self.nodes;
    }
}
