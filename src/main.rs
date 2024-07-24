use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::process::exit;

use clap::Parser;

mod cyclomatic_complexity;
use cyclomatic_complexity::count;
use cyclomatic_complexity::print_result;
mod common;
use common::filter_python_files;
use common::get_all_paths_in_directory;
// use common::get_common_directory;
use common::read_lines;

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
    let path: PathBuf = PathBuf::from(&Args::parse().path);
    let _curr_path: PathBuf = env::current_dir().unwrap();

    if !path.exists() {
        println!("There is no such file or directory: {}", path.display());
        exit(1);
    }

    if !path.is_dir() {
        let contents: Vec<String> = read_lines(&path);
        let result: HashMap<&str, u8> = count(&contents);
        print_result(&path, &result);
    } else {
        let paths: Vec<PathBuf> = get_all_paths_in_directory(&path);
        let python_files: Vec<PathBuf> = filter_python_files(&paths);
        for path in python_files {
            let contents: Vec<String> = read_lines(&path);
            let result: HashMap<&str, u8> = count(&contents);
            print_result(&path, &result);
        }
    }
}
