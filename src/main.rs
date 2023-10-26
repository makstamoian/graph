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

    fn get_nodes(&self) -> HashSet<u32> {
        let mut nodes: HashSet<u32> = HashSet::new();
        for key in self.nodes.keys() {
            nodes.insert(*key);
        }

        return nodes;
    }

    fn has_node(&self, node: u32) -> bool {
        return self.nodes.contains_key(&node)
    }   

    fn has_edge(&self, node_a: u32, node_b: u32) -> bool {
        if self.has_node(node_a) && self.has_node(node_b) {
            return self.nodes[&node_a].contains(&node_b) && self.nodes[&node_b].contains(&node_a)
        }

        return false;
    }

    fn get_node_adjacents(&self, node: u32) -> &HashSet<u32>{
        return &self.nodes[&node];
    }

    fn depth_first_search (&self, node: u32, visited_nodes: &mut HashSet<u32>) {
        visited_nodes.insert(node);

        for new_node in &self.nodes[&node]{
            if visited_nodes.contains(new_node) {
                continue;
            }

            self.depth_first_search(*new_node, visited_nodes)
        }
    }

    fn clear (&mut self) -> &HashMap<u32, HashSet<u32>>{
        let _ = &self.nodes.clear();

        return &self.nodes;
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
    let mut my_graph = generate_graph(10, 11);

    let mut visited_nodes = HashSet::new();

    my_graph.depth_first_search(0, &mut visited_nodes);

    println!("Visited total of {} nodes:\n{:#?}", visited_nodes.len(), visited_nodes);

    println!("List of all node ids in generated graph:\n{:#?}", my_graph.get_nodes());    

    println!("Check if generated graph contains node with id 1: {}", my_graph.has_node(1));

    println!("{:#?}", my_graph.get_node_adjacents(2));

    my_graph.clear();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_graph_add_node() {
        let mut graph = Graph { nodes: HashMap::new() };

        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);

        assert_eq!(graph.get_nodes(), HashSet::from([1, 2, 3]));
    }

    #[test]
    fn test_graph_add_edge() {
        let mut graph = Graph { nodes: HashMap::new() };

        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);

        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 2);

        assert_eq!(graph.has_edge(1, 2), true);
    }

    #[test]
    fn test_graph_clear() {
        let mut graph = Graph { nodes: HashMap::new() };

        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);

        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 2);
        
        graph.clear();

        assert_eq!(graph.get_nodes(), HashSet::from([]));
    }

    #[test]
    fn test_graph_has_node() {
        let mut graph = Graph { nodes: HashMap::new() };

        graph.add_node(1);

        assert_eq!(graph.has_node(1), true);
    }

    #[test]
    fn test_graph_has_edge() {
        let mut graph = Graph { nodes: HashMap::new() };

        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);

        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 2);
        
        assert_eq!(graph.has_edge(1, 2), true);
    }
}
