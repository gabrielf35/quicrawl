use std::fs::File;
use std::io::{BufRead, BufReader};

mod help;

#[tokio::main]
async fn main() {
    let seed_file = File::open("seeds.txt").expect("no seeds.txt found");
    let reader = BufReader::new(seed_file);

    let seeds: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .expect("failed to parse seeds.txt");

    for line in &seeds {
        let response = help::get(line).await.unwrap();

        let links = help::parse_links(&response);

        println!("{:#?}", links)
    }
}
