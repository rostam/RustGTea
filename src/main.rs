extern crate petgraph;

use petgraph::Graph;
use petgraph::visit::IntoNeighbors;
use std::collections::HashSet;

fn greedy_vertex_coloring<N, E>(graph: &Graph<N, E>) -> Vec<usize> {
    let mut colors = vec![0; graph.node_count()];
    for node in graph.node_indices() {
        // Get the colors of the neighboring nodes
        let neighbor_colors: HashSet<_> = graph
            .neighbors(node)
            .filter_map(|n| {
                if colors[n.index()] == 0 {
                    None
                } else {
                    Some(colors[n.index()])
                }
            })
            .collect();

        // Find the smallest color that's not used by neighbors
        let mut color = 1;
        while neighbor_colors.contains(&color) {
            color += 1;
        }
        
        colors[node.index()] = color;
    }
    colors
}

fn main() {
    // Create a sample graph: a triangle
    let mut graph = Graph::<&str, ()>::new();
    let a = graph.add_node("A");
    let b = graph.add_node("B");
    let c = graph.add_node("C");

    graph.add_edge(a, b, ());
    graph.add_edge(b, c, ());
    graph.add_edge(c, a, ());

    let colors = greedy_vertex_coloring(&graph);
    for (node, color) in graph.node_indices().zip(colors.iter()) {
        println!("Node {}: Color {}", graph[node], color);
    }
}

