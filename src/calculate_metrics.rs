use petgraph::graph::UnGraph;
use petgraph::algo::dijkstra;
use petgraph::stable_graph::NodeIndex;

// define the Embedding struct
#[derive(Debug)]
struct Embedding {
    vector: Vec<f64>,
}

impl Embedding {
    // constructor method to create Embedding from a vector
    fn new(vector: Vec<f64>) -> Self {
        Embedding { vector }
    }
}

pub fn calculate_average_distance(graph: &UnGraph<usize, ()>) -> Option<f64> {
    let n = graph.node_count();
    let mut total_distance = 0.0;
    let mut total_pairs = 0;

    // perform breadth-first search (BFS) from each node to compute shortest paths
    for node in graph.node_indices() {
        let distances = dijkstra(&graph, node, None, |_| 1);
        for (_, distance) in distances {
            if let Some(d) = distance {
                total_distance += d as f64;
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

        // Perform BFS from the current node
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

        // count the number of shortest paths passing through each edge
        for path in paths {
            for &node in &path {
                betweenness[node.index()] += 1.0;
            }
        }
    }

    // normalize betweenness centrality scores
    let normalization_factor = (n - 1) * (n - 2) / 2;
    betweenness.iter_mut().map(|x| *x / normalization_factor as f64).collect()
}

fn calculate_similarity(embedding1: &[f64], embedding2: &[f64]) -> f64 {
    // implement cosine similarity
    let dot_product = embedding1.iter().zip(embedding2.iter()).map(|(a, b)| a * b).sum::<f64>();
    let norm1 = embedding1.iter().map(|x| x * x).sum::<f64>().sqrt();
    let norm2 = embedding2.iter().map(|x| x * x).sum::<f64>().sqrt();
    dot_product / (norm1 * norm2)
}

#[cfg(test)]
mod tests {
    use petgraph::Graph;

    use super::*;

    #[test]
    fn test_calculate_betweenness_centrality() {
        // Create a test graph with a small number of nodes and random embeddings
        let mut graph = Graph::<Embedding, f64>::new_undirected();
        let nodes: Vec<_> = (0..5)
            .map(|_| {
                graph.add_node(Embedding::new(vec![0.0, 0.0]))
            })
            .collect();
        graph.add_edge(nodes[0], nodes[1], 0.0);
        graph.add_edge(nodes[1], nodes[2], 0.0);
        graph.add_edge(nodes[2], nodes[3], 0.0);
        graph.add_edge(nodes[3], nodes[4], 0.0);
        graph.add_edge(nodes[0], nodes[4], 0.0);

        // Calculate the betweenness centrality
        let betweenness_centrality = calculate_betweenness_centrality(&graph);
        // Assert the result based on your expectations
        // assert_eq!(betweenness_centrality, expected_values);
    }
}