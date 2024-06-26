use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let mut graph = mgraph::Graph::new();
    
    graph.add_node(0);
    graph.add_node(1);
    graph.add_node(2);
    graph.add_node(3);

    graph.add_edge(0, 1, 6);
    graph.add_edge(0, 2, 16);
    graph.add_edge(1, 2, 7);
    graph.add_edge(2, 3, 8);

    fn heuristic(_node: u32, _target: u32) -> u32 {
        return 1;
    }

    c.bench_function("BFS", |b| b.iter(|| graph.breadth_first_search(black_box(0), black_box(3))));
    c.bench_function("DFS", |b| b.iter(|| graph.depth_first_search(black_box(0))));
    c.bench_function("Dijkstra", |b| b.iter(|| graph.shortest_path(black_box(0), black_box(3))));
    c.bench_function("A*", |b| b.iter(|| graph.astar(black_box(0), black_box(3), black_box(heuristic))));
    c.bench_function("Bellman-Ford", |b| b.iter(|| graph.bellman_ford(black_box(0))));


}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);