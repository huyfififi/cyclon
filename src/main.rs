use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::read_to_string;
use std::io;
use std::path::Path;

use clap::Parser;

mod cyclomatic_complexity;
use cyclomatic_complexity::count;
use cyclomatic_complexity::print_result;
mod common;
use common::filter_python_files;
use common::get_all_paths_in_directory;

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
    if base_path.is_file() {
        let contents: Vec<String> = read_lines(path.clone());
        let result: HashMap<&str, u8> = count(&contents);
        print_result(
            base_path
                .file_name()
                .unwrap_or(OsStr::new("invalid path"))
                .to_str()
                .unwrap_or("invalid str"),
            &result,
        );
    } else {
        let paths: io::Result<Vec<String>> = get_all_paths_in_directory(&base_path);
        let python_files = filter_python_files(&paths.unwrap());
        for path in python_files {
            println!("{}", path);
        }
    }
}
