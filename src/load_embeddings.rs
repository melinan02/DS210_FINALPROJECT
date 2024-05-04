// module for loading in embeddings from the .csv files
use std::fs::File;
use git2::Repository;
use csv::ReaderBuilder;

pub struct Embedding {
    pub vector: Vec<f64>,
}

pub fn load_embeddings_from_csv(filename: &str) -> Vec<Embedding> {
    // Open the CSV file using Git LFS
    let repo = match Repository::open(".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let file_path = match repo.workdir() {
        Some(path) => path.join(filename),
        None => panic!("unable to determine repository working directory"),
    };

    let file = match File::open(&file_path) {
        Ok(file) => file,
        Err(e) => panic!("failed to open file: {}", e),
    };

    // Create a CSV reader
    let mut reader = ReaderBuilder::new().from_reader(file);

    // Read the CSV records and parse embeddings
    let mut embeddings = Vec::new();
    for result in reader.records() {
        let record = result.expect("error reading CSV record");
        // Assuming each record represents an embedding with space-separated values
        let embedding: Embedding = Embedding {
            vector: record.iter().map(|v| v.parse().expect("error parsing embedding")).collect(),
        };
        embeddings.push(embedding);
    }

    embeddings
}