mod load_embeddings;
mod construct_graph;
mod calculate_metrics;

use load_embeddings::load_embeddings_from_csv;
use construct_graph::construct_graph;
use calculate_metrics::{calculate_average_distance, calculate_betweenness_centrality};

fn main() {
    // Load embeddings from CSV
    let user_embeddings = load_embeddings_from_csv("web-redditEmbeddings-users.csv");
    let subreddit_embeddings = load_embeddings_from_csv("web-redditEmbeddings-subreddits.csv");

    // Construct graph
    let graph = construct_graph(&user_embeddings, &subreddit_embeddings);

    // Calculate average distance
    let avg_distance = calculate_average_distance(&graph);
    println!("Average Distance: {:?}", avg_distance);

    // Calculate betweenness centrality
    let betweenness = calculate_betweenness_centrality(&graph);
    println!("Betweenness Centrality: {:?}", betweenness);
}git lfs install