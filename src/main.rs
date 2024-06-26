mod load_embeddings;
mod construct_graph;
mod calculate_metrics;

use load_embeddings::load_embeddings_from_csv;
use construct_graph::construct_graph;
use calculate_metrics::calculate_betweenness_centrality;
// use calculate_metrics::calculate_average_distance;

fn main() {
    // load embeddings from CSV
    let user_embeddings = load_embeddings_from_csv("web-redditEmbeddings-users.csv");
    let subreddit_embeddings = load_embeddings_from_csv("web-redditEmbeddings-subreddits.csv");

    // construct graph
    let graph = construct_graph(&user_embeddings, &subreddit_embeddings);

    // // calculate average distance
    // if let Some(average_distance) = calculate_average_distance(&graph) {
    //     println!("Average Distance: {}", average_distance);
    // } else {
    //     println!("Unable to calculate average distance.");
    // }

    // calculate betweenness centrality
    let betweenness = calculate_betweenness_centrality(&graph);
    println!("Betweenness Centrality: {:?}", betweenness);
}