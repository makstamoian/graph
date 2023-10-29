use rand::Rng;
use std::collections::HashMap;

mod graph;

// Nodes: 1, 2, 3, 4, 5
// Edge: 1 -> 2, 1 -> 3, 2 -> 4, 3 -> 4, 4 -> 5

// Adjacency Sets

// Map:
// 1 => Set[2, 3]
// 2 => Set[4]
// 3 => Set[4]
// 4 => Set[5]

fn generate_graph(nodes_count: u32, edges_count: u32) -> graph::Graph {
    let mut graph = graph::Graph {
        nodes: HashMap::new(),
    };
    for node in 0..nodes_count + 1 {
        graph.add_node(node)
    }
    for _edge in 0..edges_count + 1 {
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
    let mut my_graph = generate_graph(10, 11);

    let visited_nodes = my_graph.depth_first_search(0);

    println!(
        "Visited total of {} nodes:\n{:#?}\n",
        visited_nodes.len(),
        visited_nodes
    );

    println!(
        "List of all node ids in generated graph:\n{:#?}\n",
        my_graph.get_nodes()
    );

    println!(
        "Check if generated graph contains node with id 1: {}\n",
        my_graph.has_node(1)
    );

    println!(
        "Check if generated graph contains edge from 1 to 3: {}\n",
        my_graph.has_edge(1, 3)
    );

    println!(
        "Adjacents nodes for node 2: {:#?}",
        my_graph.get_node_adjacents(2)
    );

    println!("Is graph connected: {:#?}", my_graph.is_connected());

    println!("Graph leaf nodes: {:#?}", my_graph.get_leaf_nodes());

    println!("Dropping node 1...");

    my_graph.drop_node(1);

    println!(
        "Check if graph has node 1 after its deletion: {:#?}\n",
        my_graph.has_node(1)
    );

    my_graph.clear();
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    // This function creates universe-usable graph in the field of tests, which is suitable for almost every test.
    // For tests, there the tested graph has to have specific properties (be disconnected)

    fn generate_test_graph() -> graph::Graph {
        let mut graph = graph::Graph {
            nodes: HashMap::new(),
        };

        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);

        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        return graph;
    }

    use super::*;
    #[test]
    fn test_graph_add_node() {
        let graph = generate_test_graph();

        assert_eq!(graph.get_nodes(), HashSet::from([0, 1, 2]));
    }

    #[test]
    fn test_graph_add_edge() {
        let graph = generate_test_graph();

        assert_eq!(graph.has_edge(1, 2), true);
    }

    #[test]
    fn test_graph_clear() {
        let mut graph = generate_test_graph();

        graph.clear();

        assert_eq!(graph.get_nodes(), HashSet::from([]));
    }

    #[test]
    fn test_graph_has_node() {
        let mut graph = graph::Graph {
            nodes: HashMap::new(),
        };

        graph.add_node(0);

        assert_eq!(graph.has_node(0), true);
    }

    #[test]
    fn test_graph_has_edge() {
        let graph = generate_test_graph();

        assert_eq!(graph.has_edge(0, 1), true);
    }

    #[test]
    fn test_graph_drops_node() {
        let mut graph = generate_test_graph();

        graph.drop_node(0);

        assert_eq!(graph.get_nodes(), HashSet::from([1, 2]));
    }

    #[test]
    fn test_graph_connected_true() {
        let graph = generate_test_graph();

        assert_eq!(graph.is_connected(), true);
    }

    #[test]
    fn test_graph_connected_false() {
        let mut graph = graph::Graph {
            nodes: HashMap::new(),
        };

        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);

        graph.add_edge(0, 1);

        assert_eq!(graph.is_connected(), false);
    }

    #[test]
    fn test_graph_get_leaf_nodes() {
        let mut graph = graph::Graph {
            nodes: HashMap::new(),
        };

        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);

        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);

        assert_eq!(graph.get_leaf_nodes(), HashSet::from([0, 3]));
    }
}
