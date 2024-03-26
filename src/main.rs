fn main() {

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

        let result = graph.shortest_path(0, 2);

        assert_eq!(result.cost.unwrap(), 11);
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

        let result = graph.shortest_path(0, 2);

        assert_eq!(result.cost.unwrap(), 13);
    }

    #[test]
    fn test_graph_directed_edge() {
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
        graph.add_edge_directed(2, 3, 8);

        assert_eq!(graph.has_edge(2, 3), false);
        assert_eq!(graph.has_edge_directed(2, 3), true);
        assert_eq!(graph.has_edge_directed(3, 2), false);

        graph.drop_edge(2, 3);
        assert_eq!(graph.has_edge_directed(2, 3), false);

    }
}
