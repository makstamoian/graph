use std::collections::HashMap;
use std::collections::HashSet;
use rand::Rng;

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
        if source != target {
            self.nodes.entry(source)
                .and_modify(|adjacency_set: &mut HashSet<u32>|{adjacency_set.insert(target);});
            self.nodes.entry(target)
                .and_modify(|adjacency_set: &mut HashSet<u32>|{adjacency_set.insert(source);});
        }
    }

    fn dfs (&self, node: u32, visited_nodes: &mut HashSet<u32>) {
        visited_nodes.insert(node);

        for new_node in &self.nodes[&node]{
            if visited_nodes.contains(new_node) {
                continue;
            }

            self.dfs(*new_node, visited_nodes)
        }
    }
}

fn generate_graph(nodes_count: u32, edges_count: u32) -> Graph {
    let mut graph = Graph { nodes: HashMap::new() };
    for node in 0.. nodes_count + 1 {
        graph.add_node(node)
    }
    for _edge in 0.. edges_count + 1 {
        let source_node = rand::thread_rng().gen_range(0, nodes_count);
        let mut target_node: u32;

        loop {
            target_node = rand::thread_rng().gen_range(0, nodes_count);
            if target_node != source_node {
                break;
            }
        }

        graph.add_edge(source_node, target_node)
    }
    return graph;
}

fn main() {
    let my_graph = generate_graph(10, 11);

    let mut visited_nodes = HashSet::new();

    my_graph.dfs(0, &mut visited_nodes);

    println!("Visited total of {} nodes:\n{:#?}", visited_nodes.len(), visited_nodes);
}

