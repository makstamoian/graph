fn main() {}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::collections::HashSet;

    // This function creates universe-usable graph in the field of tests, which is suitable for almost every test.
    // For tests, there the tested graph has to have specific properties (be disconnected)

    fn generate_test_graph() -> mgraph::Graph {
        let mut graph = mgraph::Graph::new();

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
        let mut graph = mgraph::Graph::new();

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
        let mut graph = mgraph::Graph::new();

        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);

        graph.add_edge(0, 1, 3);

        assert_eq!(graph.is_connected(), false);
    }

    #[test]
    fn test_graph_get_leaf_nodes() {
        let mut graph = mgraph::Graph::new();

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
        let mut graph = mgraph::Graph::new();

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
        let mut graph = mgraph::Graph::new();

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
        let mut graph = mgraph::Graph::new();

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
        let mut graph = mgraph::Graph::new();

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

    #[test]
    fn test_graph_shortest_path_1_astar() {
        let mut graph = mgraph::Graph::new();

        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);

        graph.add_edge(0, 1, 6);
        graph.add_edge(0, 2, 11);
        graph.add_edge(1, 2, 7);
        graph.add_edge(2, 3, 8);

        fn heuristic(_node: u32, _target: u32) -> u32 {
            return 1;
        }

        let result = graph.astar(0, 2, heuristic);

        assert_eq!(result, 11);
    }
    #[test]
    fn test_euclidian_distance() {
        let graph = mgraph::Graph::new();
        assert_eq!(graph.euclidian_distance((0, 4), (3, 0)), 5 as f32);
    }

    #[test]
    fn test_manhattan_distance() {
        let graph = mgraph::Graph::new();
        assert_eq!(graph.manhattan_distance((0, 4), (3, 0)), 7);
    }

    #[test]
    fn test_graph_bellman_ford_1() {
        let mut graph = mgraph::Graph::new();

        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);

        graph.add_edge(0, 1, 6);
        graph.add_edge(0, 2, 11);
        graph.add_edge(1, 2, 7);
        graph.add_edge(2, 3, 8);

        let distances = graph.bellman_ford(0);
        let result = distances.get(&2).unwrap();

        assert_eq!(*result, 11);
    }

    #[test]
    fn test_graph_has_negative_cycle_1() {
        let mut graph = mgraph::Graph::new();

        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);

        graph.add_edge(0, 1, 1); 
        graph.add_edge(1, 2, 1); 
        graph.add_edge(2, 3, 1); 
        graph.add_edge(3, 0, -4);

        let distances = graph.bellman_ford(0);
        let result = graph.has_negative_cycle(&distances);

        assert_eq!(result, true);
    }

    #[test]
    fn test_graph_has_negative_cycle_2() {
        let mut graph = mgraph::Graph::new();

        graph.add_node(0);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);

        graph.add_edge(0, 1, 2); 
        graph.add_edge(1, 2, 3); 
        graph.add_edge(2, 3, 4); 
        graph.add_edge(3, 0, 1);

        let distances = graph.bellman_ford(0);
        let result = graph.has_negative_cycle(&distances);

        assert_eq!(result, false);
    }
}
