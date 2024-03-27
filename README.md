# **mgraph**
This is a simple graph library packed up into a crate (https://crates.io/crates/mgraph). You can add it to your project by running:
```cargo add mgraph```

# **Example usage**
Let's assume you want to find the shortest path in a graph using Dijkstra algorithm. 
1. First, we need to create the graph itself:

```let mut graph = mgraph::Graph::new()```

2. After that, we need to add nodes to our graph. Nodes are represented by integers.

```
graph.add_node(0);
graph.add_node(1);
graph.add_node(2);
graph.add_node(3);
```

3. Now we can add connections between our nodes - edges:

```
graph.add_edge(0, 1, 6);
graph.add_edge(0, 2, 16);
graph.add_edge(1, 2, 7);
graph.add_edge(2, 3, 8);
```
Arguments of the `add_edge()` function are source node, target node and edge weight. If you want to create an edge going only from source to target, but not vice-versa, you can use `add_edge_directed()` instead.

4. Now we can finally run Dijkstra algorithm on our graph and see the result:

```
let result = graph.shortest_path(0, 2);
println!("{:#?}\n\n{:?}", result.cost.unwrap(), result.parents.unwrap());
```

The ```shortest_path()``` function returns two values: ```cost``` and ```parents```. ```cost``` is the cost of the shortest path from node A to node B (if one exists), while ```parents``` is a HashMap which represents a node and its predecessor (parent). We need ```parents``` to be able to restore the full shortest path from node A to node B, once again, if one exists. 

5. In order to restore the full path, we can use the ```resore_path()``` function:

```
let shortest_path = graph.restore_path(0, 2, parents);
```

```shortest_path()``` function receives ```source```, ```target``` and ```parents``` as arguments. 


This was only one of many use cases of this library, however, feel free to contribute to README.md and improve the library and the docs, I will highly appreciate it
