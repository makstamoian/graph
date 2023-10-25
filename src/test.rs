#[cfg(test)]
mod tests {
    #[test]
    fn test_graph_adds_nodes() {
        let mut graph = Graph { nodes: HashMap::new() };

        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);

        nodes = graph.get_nodes();

        assert_eq!(nodes, HashSet::new(1, 2, 3));
    }
}