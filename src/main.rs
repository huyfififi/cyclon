use std::collections::HashMap;
use std::fs::read_to_string;

use clap::Parser;

mod cyclomatic_complexity;
use cyclomatic_complexity::count;

fn read_lines(filename: String) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}

#[derive(Parser)]
#[command(name = "Cyclon")]
#[command(version = "0.0.1")]
#[command(author = "Kazuki Kijima <kazukiyoshida96@gmail.com>")]
#[command(about = "Check cyclomatic complexity")]
struct Args {
    #[arg(index(1))]
    path: String,
}

fn main() {
    let path: String = Args::parse().path;
    let contents: Vec<String> = read_lines(path);
    let results: HashMap<String, u8> = count(&contents);
    for (function, complexity) in results.into_iter() {
        println!("{}, {}", function, complexity)
    }
}
