use std::fs::File;
use std::io::{BufRead, BufReader};
use tokio::task::JoinSet;

mod help;

#[tokio::main]
async fn main() {
    let seed_file = File::open("seeds.txt").expect("no seeds.txt found");
    let reader = BufReader::new(seed_file);
    let mut tasks = JoinSet::new();

    let seeds: Vec<String> = reader
        .lines()
        .collect::<Result<_, _>>()
        .expect("failed to parse seeds.txt");

    for line in seeds {
        tasks.spawn(async move {
            help::crawl_url(&line).await;
        });
    }

    while tasks.join_next().await.is_some() {}
}
