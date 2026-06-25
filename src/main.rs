use std::fs::File;
use std::io::{BufRead, BufReader};

async fn get(url: &str) -> std::result::Result<String, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client.get(url).send().await?;

    response.text().await
}

fn main() {
    let seed_file = File::open("seeds.txt").expect("no seeds.txt found");
    let reader = BufReader::new(seed_file);

    let seeds: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .expect("failed to parse seeds.txt");

    for line in &seeds {
        println!("{}", line);
    }
}
