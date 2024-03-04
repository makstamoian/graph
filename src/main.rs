use rand::Rng;
mod graph;
use std::time::Instant;


fn generate_graph(nodes_count: u32, edges_count: u32) -> graph::Graph {
    let mut graph = graph::Graph::new();
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

        graph.add_edge(source_node, target_node, rand::thread_rng().gen_range(1,100))
    }

    return graph;
}

fn main() {
    let mut my_graph = generate_graph(100000, 200000);

    let start = Instant::now();

    my_graph.serialize();

    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("Serialisation elapsed time: {:#?}", duration);

    my_graph.depth_first_search(0);

    my_graph.has_node(1);

    my_graph.has_edge(1, 3);

    my_graph.is_connected();

    my_graph.has_edge(4, 3);

    my_graph.drop_edge(4, 3);

    my_graph.has_edge(4, 3);

    my_graph.get_node_adjacents(1);

    my_graph.get_leaf_nodes();

    my_graph.drop_node(1);

    my_graph.has_node(1);

    println!("{:#?}", my_graph.breadth_first_search(2, 9));

    my_graph.clear();
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::collections::HashSet;

    // This function creates universe-usable graph in the field of tests, which is suitable for almost every test.
    // For tests, there the tested graph has to have specific properties (be disconnected)

    fn generate_test_graph() -> graph::Graph {
        let mut graph = graph::Graph::new();

        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);

        graph.add_edge(0, 1, 1);
        graph.add_edge(1, 2, 2);

        return graph;
    }

    use super::*;

    #[test]
    fn test_graph_add_edge() {
        let graph = generate_test_graph();

        assert_eq!(graph.has_edge(1, 2), true);
    }

    #[test]
    fn test_graph_clear() {
        let mut graph = generate_test_graph();

        graph.clear();

        assert_eq!(graph.nodes, HashMap::from([]));
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

        graph.add_edge(0, 1, 3);

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

        graph.add_edge(0, 1, 4);
        graph.add_edge(1, 2, 5);
        graph.add_edge(2, 3, 6);

        assert_eq!(graph.get_leaf_nodes(), HashSet::from([0, 3]));
    }

    #[test]
    fn test_graph_drops_edge() {
        let mut graph = graph::Graph {
            nodes: HashMap::new(),
        };

        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);

        graph.add_edge(0, 1, 6);
        graph.add_edge(1, 2, 7);
        graph.add_edge(2, 3, 8);

        graph.drop_edge(2, 3);

        assert_eq!(graph.has_edge(2, 3), false);
    }

    #[test]
    fn test_graph_drops_node() {
        let mut graph = generate_test_graph();

        graph.drop_node(2);

        assert_eq!(graph.has_node(2), false);
    }

    #[test]
    fn test_breadth_first_search() {
        let graph = generate_test_graph();

        assert_eq!(graph.breadth_first_search(0, 2), 2)
    }

    #[test]
    fn test_graph_shortest_path_1() {
        let mut graph = graph::Graph {
            nodes: HashMap::new(),
        };

        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);

        graph.add_edge(0, 1, 6);
        graph.add_edge(0, 2, 11);
        graph.add_edge(1, 2, 7);
        graph.add_edge(2, 3, 8);

        assert_eq!(graph.shortest_path(0, 2), 11)
    }

    #[test]
    fn test_graph_shortest_path_2() {
        let mut graph = graph::Graph {
            nodes: HashMap::new(),
        };

        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);

        graph.add_edge(0, 1, 6);
        graph.add_edge(0, 2, 16);
        graph.add_edge(1, 2, 7);
        graph.add_edge(2, 3, 8);

        assert_eq!(graph.shortest_path(0, 2), 13)
    }
}
