//! ## Mgraph
//! mgraph is a simple, fast, performace-oriented graph library for Rust. It's being contributed to on daily basis and grows fast!
//! 
//! ## Features
//! 
//! Features that are implemented already:
//! 
//! -  DFS algorithm
//! -  BFS algorithm
//! -  Dijkstra shortest path algorithm
//! -  Serialization and deserialization of graphs
//! 
//! Features that are to be implemented in future:
//! 
//! -  A* algorithm
//! -  Other intresting things of graph theory, such as different search algorithms, sorting algorithms etc.
//!
//! ## Example usage
//! 
//! ```
//! let mut graph = mgraph::Graph::new();
//! 
//! graph.add_node(0);
//! graph.add_node(1);
//! graph.add_node(2);
//! graph.add_node(3);
//! 
//! graph.add_edge(0, 1, 6);
//! graph.add_edge(0, 2, 16);
//! graph.add_edge(1, 2, 7);
//! graph.add_edge(2, 3, 8);
//! 
//! let result = graph.shortest_path(0, 2);
//! 
//! let parents = result.parents.unwrap();
//! let cost = result.cost.unwrap();
//! 
//! println!("{:#?}\n\n{:?}", cost, parents);
//! 
//! let shortest_path = graph.restore_path(0, 2, parents);
//! ``` 


use serde_json;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::collections::BinaryHeap;


/// Graph data structure
#[derive(Debug)]
pub struct Graph {
    /// Nodes data structure, a HashMap of a node and a tuple of adjacent node and edge weight. 
    /// Nodes and weights are represented as integers of type `u32`.
    /// This implementation of graph data structure uses adjacentcy list architecture rather than adjacency matrix because of second's bad performance
    pub nodes: HashMap<u32, HashSet<(u32, u32)>>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct DijkstraState {
    node: u32,
    cost: i32,
}

/// DijkstraResult structure contains shortest path algorithm return values
#[derive(PartialEq, Debug)]
pub struct DijkstraResult {
    /// Cost of shortest path. `Option<u32>` if exists, `None` if no path was found
    pub cost: Option<u32>, 
    /// Map of a node and its predecessor, used in `resore_path()`. `Option<HashMap<u32, i32>>` if exists, `None` otherwise.
    pub parents: Option<HashMap<u32, i32>>
}

impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost).then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Graph {

    /// Creates new graph of the `Graph` type.
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    /// Adds a node to the graph.

    pub fn add_node(&mut self, node: u32) {
        self.nodes.insert(node, HashSet::new());
    }

    /// Adds a directed edge between `source` and `target` with the weight `weight`. After using this function, an edge will appear for `source`, but not for `target`.

    pub fn add_edge_directed(&mut self, source: u32, target: u32, weight: u32) {
        if source != target {
            self.nodes
                .entry(source)
                .and_modify(|adjacency_set: &mut HashSet<(u32, u32)>| {
                    adjacency_set.insert((target, weight));
                });
        }
    }

    /// Adds an edge between `source` and `target` with the weight `weight`. After using this function, and edge will appear for both `source` and `target`.

    pub fn add_edge(&mut self, source: u32, target: u32, weight: u32) {
        self.add_edge_directed(source, target, weight);
        self.add_edge_directed(target, source, weight)
    }

    /// Drops edge from `node_a` to `node_b`. After using this function, only edge from `node_a` to `node_b` will be terminated, however connection between `node_b` and `node_a` will still remain.

    pub fn drop_edge_directed(&mut self, node_a: u32, node_b: u32) {
        self.nodes
            .entry(node_a)
            .and_modify(|adjacency_set: &mut HashSet<(u32, u32)>| {
                adjacency_set.retain(|&(first, _)| first != node_b);

            });
    }

    /// Drops edge from `node_a` to `node_b` and vise-versa. After using this function, no connection between nodes `node_a` and `node_b` will remain.

    pub fn drop_edge(&mut self, node_a: u32, node_b: u32) {
        self.drop_edge_directed(node_a, node_b);
        self.drop_edge_directed(node_b, node_a);
    }

    /// Drops a node and all edges going to that node from adjacent nodes. 

    pub fn drop_node(&mut self, node: u32) {
        self.nodes.remove(&node);
        for (_, adjacents) in self.nodes.iter_mut() {
            adjacents.retain(|&adjacent| {
                adjacent.1 != node
            })
        }
    }

    /// Checks if graph has a certain node. 

    pub fn has_node(&self, node: u32) -> bool {
        return self.nodes.contains_key(&node);
    }

    /// Checks if there is an edge from `node_a` to `node_b`.

    pub fn has_edge_directed(&self, node_a: u32, node_b: u32) -> bool {
        if self.has_node(node_a) && self.has_node(node_b) {
            return self.nodes[&node_a].iter().any(|&(first, _)| first == node_b);
        }

        return false;
    }

    /// Checks if there is and edge from `node_a` to `node_b` and an edge from `node_b` to `node_a`.

    pub fn has_edge(&self, node_a: u32, node_b: u32) -> bool {
        return self.has_edge_directed(node_a, node_b) && self.has_edge_directed(node_b, node_a);
    }

    /// Returns all adjacent nodes of `node`

    pub fn get_node_adjacents(&self, node: u32) -> &HashSet<(u32, u32)> {
        return &self.nodes[&node];
    }

    /// Returns all leaf nodes in graph.
    /// Leaf nodes are nodes which have only one adjacent node.

    pub fn get_leaf_nodes(&self) -> HashSet<u32> {
        let mut leaf_nodes: HashSet<u32> = HashSet::new();

        for node in self.nodes.keys() {
            if self.get_node_adjacents(*node).len() == 1 {
                leaf_nodes.insert(*node);
            }
        }

        return leaf_nodes;
    }

    pub fn depth_first_search(&self, node: u32) -> HashSet<u32> {
        let mut stack: VecDeque<u32> = VecDeque::new();
        let mut visited_nodes: HashSet<u32> = HashSet::new();

        stack.push_back(node);

        while let Some(node_pop) = stack.pop_back() {
            if visited_nodes.contains(&node_pop) == false {
                visited_nodes.insert(node_pop);
                for adjacent in self.get_node_adjacents(node_pop).iter() {
                    stack.push_back((*adjacent).0);
                }
            }
        }

        return visited_nodes;
    }

    pub fn breadth_first_search(&self, node: u32, target: u32) -> i32 {
        let mut queue: VecDeque<u32> = VecDeque::new();
        let mut visited_nodes: HashSet<u32> = HashSet::new();

        visited_nodes.insert(node);
        queue.push_back(node);

        while let Some(node_pop) = queue.pop_front() {
            if node_pop == target {
                return node_pop as i32;
            }

            for adjacent in self.get_node_adjacents(node_pop).iter() {
                queue.push_back((*adjacent).0);
                visited_nodes.insert((*adjacent).0);
            }
        }

        return -1;
    }

    /// Restores path from `source` to `target`. `parents` is a `HashMap<u32, i32>`, which stores a node and its predecessor.

    pub fn restore_path(&self, source: u32, target: u32, parents: HashMap<u32, i32>) -> VecDeque<u32> {
        let mut path: VecDeque<u32> = VecDeque::from([]);
        let mut current_node = target;
        while current_node != source {
            path.push_front(current_node);
            current_node = *parents.get(&current_node).unwrap() as u32;
        }
        path.push_front(source);
        return path;
    }

    /// Finds the length of the shortest path from `source` to `target` using Dijkstra algorithm.
    /// Returns `DijkstraResult`.

    pub fn shortest_path(&self, source: u32, target: u32) -> DijkstraResult {
        if source == target {
            return DijkstraResult { cost: Some(0), parents: Some(HashMap::from([(0, 0)])) }
        }

        let mut queue: BinaryHeap<DijkstraState> = BinaryHeap::new();
        let mut tentative_distances: HashMap<u32, i32> = HashMap::new();
        let mut parents: HashMap<u32, i32> = HashMap::new();

        for node in self.nodes.keys() {
            tentative_distances.insert(*node, i32::MAX);
            parents.insert(*node, -1);
        } // add all nodes to queue and distances HashMap with distance to them set to i32::MAX by default. 

        queue.push(DijkstraState {node: source, cost: 0});

        tentative_distances.entry(source).and_modify(|entry| {
            *entry = 0;
        }); // set distance to source to 0
        parents.entry(source).and_modify(|entry| {
            *entry = 0;
        });

        while let Some(DijkstraState {node, cost}) = queue.pop() {
            if node == target {
                return DijkstraResult { cost: Some(cost as u32), parents: Some(parents) };
            }
            if cost > *tentative_distances.get(&node).unwrap() {
                continue;
            }

            for node_tuple in &self.nodes[&node] {
                let next: DijkstraState = DijkstraState { node: node_tuple.0, cost: node_tuple.1 as i32 + cost };

                if next.cost < *tentative_distances.get(&next.node).unwrap() {
                    queue.push(next);
                    tentative_distances.entry(next.node).and_modify(|entry| {
                        *entry = next.cost;
                    });
                    parents.entry(next.node).and_modify(|entry| {
                        *entry = node as i32;
                    });
                }
            }
        }

        return DijkstraResult {cost: None, parents: None} ;
    }

    pub fn astar(&self, source: u32, target: u32, heuristic: impl Fn(u32, u32) -> u32) -> u32 {
        if source == target {
            return 0;
        }

        let mut queue: BinaryHeap<DijkstraState> = BinaryHeap::new();
        let mut tentative_distances: HashMap<u32, i32> = HashMap::new();
        let mut parents: HashMap<u32, i32> = HashMap::new();

        for node in self.nodes.keys() {
            tentative_distances.insert(*node, i32::MAX);
            parents.insert(*node, -1);
        } // add all nodes to queue and distances HashMap with distance to them set to i32::MAX by default. 

        // reweighting the graph...

        queue.push(DijkstraState {node: source, cost: 0});

        tentative_distances.entry(source).and_modify(|entry| {
            *entry = 0;
        }); // set distance to source to 0
        parents.entry(source).and_modify(|entry| {
            *entry = 0;
        });

        while let Some(DijkstraState {node, cost}) = queue.pop() {
            if node == target {
                return *tentative_distances.get(&node).unwrap() as u32;
            }
            
            if cost > *tentative_distances.get(&node).unwrap() {
                continue;
            }

            for node_tuple in &self.nodes[&node] { // for adjacent node to our node that we are observing
                let next_cost = node_tuple.1 as i32 + cost;
                let next_node = node_tuple.0;

                if next_cost < *tentative_distances.get(&next_node).unwrap() {
                    queue.push(DijkstraState { node: next_node, cost: next_cost + heuristic(next_node, target) as i32 });
                    tentative_distances.entry(next_node).and_modify(|entry| {
                        *entry = next_cost;
                    });
                    parents.entry(next_node).and_modify(|entry| {
                        *entry = node as i32;
                    });
                }
            }
        }
    
        return 0;
    }


    /// Checks if a graph is conected. 
    /// Conected graph is a graph where there are no nodes with less than one adjacent node.

    pub fn is_connected(&self) -> bool {
        let visited_nodes = self.depth_first_search(0);
        if visited_nodes.len() < self.nodes.len() {
            return false;
        }
        return true;
    }

    /// Serializes a graph into JSON format.

    pub fn serialize(&self) -> String {
        return serde_json::to_string(&self.nodes).unwrap();
    }

    /// Clears graph from edges.

    pub fn clear(&mut self) -> &HashMap<u32, HashSet<(u32, u32)>> {
        let _ = &self.nodes.clear();

        return &self.nodes;
    }
}
