use std::collections::HashMap;
use std::fs::read_to_string;
use std::path::Path;

use clap::Parser;

mod cyclomatic_complexity;
use cyclomatic_complexity::count;
use cyclomatic_complexity::print_result;

fn read_lines(filename: String) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.trim().to_string());
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
    // TODO: Add optiosn to show only problematic functions
}

fn main() {
    let path: String = Args::parse().path;
    let base_path = Path::new(&path);
    let contents: Vec<String> = read_lines(path.clone());
    let result: HashMap<&str, u8> = count(&contents);
    print_result(base_path, &result);
}
