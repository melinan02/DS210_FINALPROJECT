// Remove the unused import
// use petgraph::graph::{Graph, UnGraph};
use petgraph::graph::UnGraph;
use crate::load_embeddings::Embedding;

pub fn construct_graph(user_embeddings: &[Embedding], subreddit_embeddings: &[Embedding]) -> UnGraph<usize, ()> {
    let mut graph: UnGraph<usize, ()> = UnGraph::new_undirected();

    // Add nodes to the graph for users
    let user_nodes: Vec<_> = user_embeddings.iter().map(|_| graph.add_node(0)).collect();

    // Add nodes to the graph for subreddits
    let subreddit_nodes: Vec<_> = subreddit_embeddings.iter().map(|_| graph.add_node(0)).collect();

    // Add edges between user and subreddit nodes based on embeddings (e.g., using similarity)
    for (user_idx, user_embedding) in user_embeddings.iter().enumerate() {
        for (subreddit_idx, subreddit_embedding) in subreddit_embeddings.iter().enumerate() {
            // Calculate similarity between user and subreddit embeddings
            let similarity = calculate_similarity(&user_embedding.vector, &subreddit_embedding.vector);
            // Add edge if similarity meets certain criteria
            if similarity > 0.5 {
                graph.add_edge(user_nodes[user_idx], subreddit_nodes[subreddit_idx], ());
            }
        }
    }

    graph
}

fn calculate_similarity(embedding1: &[f64], embedding2: &[f64]) -> f64 {
    // For demonstration, let's calculate cosine similarity
    let dot_product = embedding1.iter().zip(embedding2.iter()).map(|(a, b)| a * b).sum::<f64>();
    let norm1 = embedding1.iter().map(|x| x * x).sum::<f64>().sqrt();
    let norm2 = embedding2.iter().map(|x| x * x).sum::<f64>().sqrt();
    dot_product / (norm1 * norm2)
}