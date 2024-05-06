# DS210_FINALPROJECT
For this project on Reddit data, I used a dataset from Stanford Large Network Dataset Collection which
contains embeddings of Reddit users and subreddits. Initially, I was interested in this dataset because
the social media platform is one I frequently use, so I had a personal investment in finding
connections within the app. I also thought finding the distances between different nodes in this
dataset would be insightful in determining what interests can be grouped together, which could
be unpredictable.

The data is linked here: https://snap.stanford.edu/data/web-RedditEmbeddings.html and is also uploaded to this repository.

Sources used:

https://snap.stanford.edu/data/web-RedditEmbeddings.html

https://users.rust-lang.org/t/exploring-rust-developing-a-custom-graph/57785

https://depth-first.com/articles/2020/02/03/graphs-in-rust-an-introduction-to-petgraph/

https://docs.rs/petgraph/latest/petgraph/

https://levelup.gitconnected.com/working-with-csv-data-in-rust-7258163252f8

### Overview:
This project attempts to analyze the interactions between Reddit users and subreddits using embeddings. It loads embeddings from CSV files containing user and subreddit representations, constructs a graph based on distances and centrality between embeddings, and performs graph analysis to gain insights into user-subreddit relationships.

### Features:
Loading Embeddings: The project loads embeddings from .csv files containing vectors of floating-point numbers representing user and subreddit embeddings.

Constructing Graph: It constructs a graph where nodes represent Reddit users and subreddits, and edges represent interactions between them. Interactions are determined based on the similarity between user and subreddit embeddings.

Graph Analysis: The project performs graph analysis tasks including:
- Average Distance: Calculates the average distance between user and subreddit nodes in the graph. This metric provides insights into the overall proximity of users to subreddits (Ultimately, commented out due to a bug -- will try to fix this in future).
- Betweenness Centrality: Computes betweenness centrality scores for each node in the graph, indicating the influence of users and subreddits in facilitating interactions.

### Output
Betweenness Centrality: The program prints betweenness centrality scores for each user and subreddit node in the graph, indicating their influence in facilitating interactions.

Average Distance: It displays the average distance between user and subreddit nodes in the graph, providing insights into the overall structure of user-subreddit relationships (will work on in future application).

Overall, this project is an exploration into social media connections, and I hope to continue with similar projects in the future!
