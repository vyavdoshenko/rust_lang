use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = minigrep::parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}