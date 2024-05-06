use petgraph::{algo::*, prelude::*};
use petgraph::graph::NodeIndex;

pub fn calculate_average_distance(graph: &UnGraph<usize, ()>) -> Option<f64> {
    let n = graph.node_count();
    let mut total_distance = 0;
    let mut total_pairs = 0;

    for node in graph.node_indices() {
        let distances = dijkstra(&graph, node, None, |_| 1);
        for (_, distance) in distances {
            if let Some(d) = distance {
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
        let mut paths: Vec<Vec<NodeIndex>> = Vec::new();

        // Perform breadth-first search (BFS) from the current node
        let mut queue = vec![vec![node]];
        while let Some(path) = queue.pop() {
            let current_node = *path.last().unwrap();
            let neighbors: Vec<NodeIndex> = graph.neighbors(current_node).collect();

            for &neighbor in &neighbors {
                if !path.contains(&neighbor) {
                    let mut new_path = path.clone();
                    new_path.push(neighbor);
                    queue.push(new_path);
                    if neighbor != node {
                        paths.push(new_path);
                    }
                }
            }
        }

        // Count the number of shortest paths passing through each edge
        for path in paths {
            for &node in &path {
                betweenness[node.index()] += 1.0;
            }
        }
    }

    // Normalize betweenness centrality scores
    let normalization_factor = (n - 1) * (n - 2) / 2;
    betweenness.iter_mut().map(|x| *x / normalization_factor as f64).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::{UnGraph, NodeIndex};
    
    #[test]
    fn test_calculate_average_distance() {
        // Create a test graph with a large number of nodes
        let mut graph = UnGraph::<usize, ()>::new_undirected();
        let nodes: Vec<_> = (0..1000).map(|_| graph.add_node(0)).collect();
        // Add edges to the graph (you might need to design a specific structure of edges based on your requirements)
        for i in 0..nodes.len() - 1 {
            graph.add_edge(nodes[i], nodes[i + 1], ());
        }

        // Calculate the average distance
        let average_distance = calculate_average_distance(&graph);
        // Assert the result based on your expectations
        // assert_eq!(average_distance, Some(expected_value));
    }

    #[test]
    fn test_calculate_betweenness_centrality() {
        // Create a test graph with a large number of nodes
        let mut graph = UnGraph::<usize, ()>::new_undirected();
        let nodes: Vec<_> = (0..1000).map(|_| graph.add_node(0)).collect();
        // Add edges to the graph (you might need to design a specific structure of edges based on your requirements)
        for i in 0..nodes.len() - 1 {
            graph.add_edge(nodes[i], nodes[i + 1], ());
        }

        // Calculate the betweenness centrality
        let betweenness_centrality = calculate_betweenness_centrality(&graph);
        // Assert the result based on your expectations
        // assert_eq!(betweenness_centrality, expected_values);
    }
}