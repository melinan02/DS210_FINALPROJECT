// module for loading in embeddings from the .csv files
use std::fs::File;
use git2::Repository;
use csv::ReaderBuilder;
use std::fmt;

pub struct Embedding {
    pub vector: Vec<f64>,
}

pub fn load_embeddings_from_csv(filename: &str) -> Vec<Embedding> {
    // open the .csv file using Git LFS
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

    // create a .csv reader
    let mut reader = ReaderBuilder::new().from_reader(file);

    // read the .csv records and parse embeddings
    let mut embeddings = Vec::new();
    for result in reader.records() {
        let record = result.expect("error reading CSV record");
        // assuming each record represents an embedding with space-separated values
        let embedding: Embedding = Embedding {
            vector: record.iter().map(|v| v.parse().expect("error parsing embedding")).collect(),
        };
        embeddings.push(embedding);
    }

    embeddings
}

impl fmt::Debug for Embedding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // implement the formatting logic here
        write!(f, "Embedding {{ vector: {:?} }}", self.vector)
    }
}

fn main() {
    // example usage of load_embeddings_from_csv function
    let embeddings = load_embeddings_from_csv("example.csv");
    println!("{:?}", embeddings);
}