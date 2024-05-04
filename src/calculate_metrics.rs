use petgraph::graph::{Graph, UnGraph};
use petgraph::prelude::*;
use petgraph::algo::{dijkstra, eccentricity, unweighted_shortest_path};
use petgraph::visit::NodeCount;

pub fn calculate_average_distance(graph: &UnGraph<usize, ()>) -> Option<f64> {
    let n = graph.node_count();
    let mut total_distance = 0;
    let mut total_pairs = 0;

    for node in graph.node_indices() {
        let distances = dijkstra(&graph, node, None, |_| 1);
        for (_, distance) in distances {
            if let Option(d) = distance {
                total_distance += d;
                total_pairs += 1;
            }
        }
    }

    if total_pairs > 0 {
        Some(total_distance as f64 / total_pairs as f64)
    } else {
        None
    }
}

pub fn calculate_betweenness_centrality(graph: &UnGraph<usize, ()>) -> Vec<f64> {
    let n = graph.node_count();
    let mut betweenness = vec![0.0; n];

    for node in graph.node_indices() {
        let shortest_paths = unweighted_shortest_path(&graph, node);
        for target in graph.node_indices() {
            if node != target {
                for path in shortest_paths.paths_to(target) {
                    for edge in path {
                        betweenness[edge] += 1.0;
                    }
                }
            }
        }
    }

    let normalization_factor = (n - 1) * (n - 2) / 2;
    betweenness.iter_mut().map(|x| *x / normalization_factor as f64).collect()
}