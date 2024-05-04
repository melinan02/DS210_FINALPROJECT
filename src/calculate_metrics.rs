use petgraph::prelude::*;
use petgraph::algo::{average_distance, betweenness_centrality};

pub fn calculate_average_distance(graph: &UnGraph<usize, ()>) -> Option<f64> {
    average_distance(&graph)
}

pub fn calculate_betweenness_centrality(graph: &UnGraph<usize, ()>) -> Vec<f64> {
    betweenness_centrality(&graph, None)
}