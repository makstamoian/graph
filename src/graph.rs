
use serde_json;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::collections::BinaryHeap;

// Nodes: 1, 2, 3, 4, 5
// Edge: 1 -> 2, 1 -> 3, 2 -> 4, 3 -> 4, 4 -> 5
//
// Adjacency Sets
//
// Map:
// 1 => Set[2, 3]
// 2 => Set[4]
// 3 => Set[4]
// 4 => Set[5]

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashMap<u32, HashSet<(u32, u32)>>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct DijkstraState {
    node: u32,
    cost: i32,
}

#[derive(PartialEq, Debug)]
pub struct DijkstraResult {
    pub cost: Option<u32>,
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
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: u32) {
        self.nodes.insert(node, HashSet::new());
    }

    pub fn add_edge_directed(&mut self, source: u32, target: u32, weight: u32) {
        if source != target {
            self.nodes
                .entry(source)
                .and_modify(|adjacency_set: &mut HashSet<(u32, u32)>| {
                    adjacency_set.insert((target, weight));
                });
        }
    }

    pub fn add_edge(&mut self, source: u32, target: u32, weight: u32) {
        self.add_edge_directed(source, target, weight);
        self.add_edge_directed(target, source, weight)
    }

    pub fn drop_edge_directed(&mut self, node_a: u32, node_b: u32) {
        self.nodes
            .entry(node_a)
            .and_modify(|adjacency_set: &mut HashSet<(u32, u32)>| {
                adjacency_set.retain(|&(first, _)| first != node_b);

            });
    }

    pub fn drop_edge(&mut self, node_a: u32, node_b: u32) {
        self.drop_edge_directed(node_a, node_b);
        self.drop_edge_directed(node_b, node_a);
    }

    pub fn drop_node(&mut self, node: u32) {
        self.nodes.remove(&node);
        for (_, adjacents) in self.nodes.iter_mut() {
            adjacents.retain(|&adjacent| {
                adjacent.1 != node
            })
        }
    }

    pub fn has_node(&self, node: u32) -> bool {
        return self.nodes.contains_key(&node);
    }

    pub fn has_edge_directed(&self, node_a: u32, node_b: u32) -> bool {
        if self.has_node(node_a) && self.has_node(node_b) {
            return self.nodes[&node_a].iter().any(|&(first, _)| first == node_b);
        }

        return false;
    }

    pub fn has_edge(&self, node_a: u32, node_b: u32) -> bool {
        return self.has_edge_directed(node_a, node_b) && self.has_edge_directed(node_b, node_a);
    }

    pub fn get_node_adjacents(&self, node: u32) -> &HashSet<(u32, u32)> {
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
        } // add all nodes to queue and distances HashMap with distance to them as i32::MAX

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

    pub fn is_connected(&self) -> bool {
        let visited_nodes = self.depth_first_search(0);
        if visited_nodes.len() < self.nodes.len() {
            return false;
        }
        return true;
    }

    pub fn serialize(&self) -> String {
        return serde_json::to_string(&self.nodes).unwrap();
    }

    pub fn clear(&mut self) -> &HashMap<u32, HashSet<(u32, u32)>> {
        let _ = &self.nodes.clear();

        return &self.nodes;
    }
}
