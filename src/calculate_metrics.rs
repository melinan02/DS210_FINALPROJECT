use petgraph::graph::UnGraph;
// use petgraph::algo::dijkstra;
use petgraph::stable_graph::NodeIndex;

// pub fn calculate_average_distance(graph: &UnGraph<usize, ()>) -> Option<f64> {
//     let n = graph.node_count();
//     let mut total_distance = 0.0;
//     let mut total_pairs = 0;

//     // perform breadth-first search (BFS) from each node to compute shortest paths
//     for node in graph.node_indices() {
//         let distances = dijkstra(&graph, node, None, |_| 1);

//         for (_, distance) in distances {
//             match distance {
//                 Some(d) => {
//                     total_distance += d;
//                     total_pairs += 1;
//                 }
//                 None => {
//                     // handle the case where distance is None
//                     return None;
//                 }
//             }
//         }
//     }

//     if total_pairs > 0 {
//         Some(total_distance / total_pairs as f64)
//     } else {
//         None
//     }
// }

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
                    queue.push(new_path.clone());
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
    let normalized_betweenness: Vec<f64> = betweenness.iter().map(|x| *x / normalization_factor as f64).collect();

    // Print the betweenness centrality scores
    println!("Betweenness Centrality Scores:");
    for (index, score) in normalized_betweenness.iter().enumerate() {
        println!("Node {}: {}", index, score);
    }

    normalized_betweenness
}

#[cfg(test)]
mod tests {
    use petgraph::Graph;
    use crate::load_embeddings::Embedding;

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

        // calculate betweenness centrality
        let betweenness = calculate_betweenness_centrality(&graph);

        // assert the result
        assert_eq!(betweenness.len(), graph.node_count());

        // expected values for the given graph
        let expected_betweenness = vec![0.0, 0.0, 3.0, 0.0, 0.0];
        for node in graph.node_indices() {
            assert_eq!(betweenness[node.index()], expected_betweenness[node.index()]);
        }
    }
}