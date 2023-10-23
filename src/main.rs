use std::collections::HashMap;
use std::collections::HashSet;

// Nodes: 1, 2, 3, 4, 5
// Edge: 1 -> 2, 1 -> 3, 2 -> 4, 3 -> 4, 4 -> 5

// Adjacency Sets

// Map:
// 1 => Set[2, 3]
// 2 => Set[4]
// 3 => Set[4]
// 4 => Set[5]

#[derive(Debug)]
struct Graph {
    nodes: HashMap<u32, HashSet<u32>>,
}

impl Graph {
    fn add_node(&mut self, node: u32) {
        self.nodes.insert(node, HashSet::new());
    }

    fn add_edge(&mut self, source: u32, target: u32) {
        self.nodes.entry(source)
            .and_modify(|adjacency_set: &mut HashSet<u32>|{adjacency_set.insert(target);});
        // self.nodes.get_mut(&source).insert(target);
        // self.nodes[&target].insert(source);
    }
}

fn main() {
    let mut graph = Graph { nodes: HashMap::new() };

    graph.add_node(1);
    graph.add_node(2);
    graph.add_node(4);
    graph.add_node(7);
    graph.add_node(9);
    graph.add_edge(1, 4);
    graph.add_edge(2, 4);
    graph.add_edge(4, 7);

    println!("\n{:#?}", graph)
}