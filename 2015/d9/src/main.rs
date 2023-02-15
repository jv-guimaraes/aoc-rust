#![allow(unused)]

use std::collections::{HashMap};

use petgraph::{graph::Graph, Undirected, dot::{Dot}, algo::{k_shortest_path, self}, visit::{Bfs, Dfs, EdgeRef}};

fn load_graph<'a>() -> Graph<&'a str, i32, Undirected> {
    let input = include_str!("..\\s_input.txt");
    // let input = include_str!("..\\input.txt");
    let mut graph: Graph<&str, i32, Undirected> = Graph::default();
    let mut cities = HashMap::new();
    
    for line in input.lines() {
        let tokens = line.split(' ').collect::<Vec<_>>();
        let start = tokens[0];
        let destination = tokens[2];
        
        if !cities.contains_key(start) {
            cities.insert(start, graph.add_node(start));
        }
        if !cities.contains_key(destination) {
            cities.insert(destination, graph.add_node(destination));
        }
        
        let start = *cities.get(start).unwrap();
        let destination = *cities.get(destination).unwrap();
        let weight = tokens[4].parse::<i32>().unwrap();
        graph.add_edge(start, destination, weight);
    }
    graph
}

fn main() {
    let mut graph = load_graph();

    println!("{:?}", Dot::with_config(&graph, &[]));

    println!("{:?}", graph);

}