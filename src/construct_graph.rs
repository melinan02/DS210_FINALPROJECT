use petgraph::graph::UnGraph;
use crate::load_embeddings::Embedding;

pub fn construct_graph(user_embeddings: &[Embedding], subreddit_embeddings: &[Embedding]) -> UnGraph<usize, ()> {
    let mut graph: UnGraph<usize, ()> = UnGraph::new_undirected();

    // add nodes to the graph for users
    let user_nodes: Vec<_> = user_embeddings.iter().map(|_| graph.add_node(0)).collect();

    // add nodes to the graph for subreddits
    let subreddit_nodes: Vec<_> = subreddit_embeddings.iter().map(|_| graph.add_node(0)).collect();

    // add edges between user and subreddit nodes based on a criterion
    for (user_idx, _) in user_embeddings.iter().enumerate() {
        for (subreddit_idx, _) in subreddit_embeddings.iter().enumerate() {
            // add edge between user and subreddit if their indices match
            if user_idx == subreddit_idx {
                graph.add_edge(user_nodes[user_idx], subreddit_nodes[subreddit_idx], ());
            }
        }
    }

    graph
}