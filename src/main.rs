extern crate petgraph;

use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use petgraph::prelude::*;
use petgraph::Undirected;
use petgraph::algo::min_spanning_tree;
use petgraph::visit::Dfs;

#[derive(Default)]
struct MyEdge {
    weight: i32,
    name: String,
}

#[derive(Default)]
struct MyNode {
    color: i32,
    distance: i32, // distance can be another struct if needed
}

type MyGraph = Graph<MyNode, MyEdge, Undirected, u32>;

fn statistics(graph: &MyGraph) {
    let num_vertices = graph.node_count();
    let num_edges = graph.edge_count();

    println!("Number of vertices: {}", num_vertices);
    println!("Number of edges: {}", num_edges);

    // eigenvalues cannot be computed directly with petgraph, you might need to use other crates or write the logic manually
}

fn main() {
    let mut graph = MyGraph::new_undirected();

    // Add nodes and edges to the graph as needed
    /*max_flow.add_edge(0, 1, 16);
    max_flow.add_edge(0, 2, 13);
    max_flow.add_edge(1, 2, 10);
    max_flow.add_edge(1, 3, 12);
    max_flow.add_edge(2, 1, 4);
    max_flow.add_edge(2, 4, 14);
    max_flow.add_edge(3, 2, 9);
    max_flow.add_edge(3, 5, 20);
    max_flow.add_edge(4, 3, 7);
    max_flow.add_edge(4, 5, 4);*/
    
        // Add nodes to the graph
    let node1 = graph.add_node(MyNode::default());  // adding a default node
    let node2 = graph.add_node(MyNode { color: 1, distance: 5 });  // adding a node with specific properties

    // Add an edge to the graph with a weight
    let edge = MyEdge { weight: 10, name: String::from("edge1") };
    graph.add_edge(node1, node2, edge);

    // Print the edges along with their weights
    for edge in graph.edge_indices() {
        let edge_weight = &graph[edge];
        println!("Edge {:?} has weight {}", edge, edge_weight.weight);
    }

    statistics(&graph);

}

