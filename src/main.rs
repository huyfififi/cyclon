use std::cmp;
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
use common::get_common_directory;
use common::get_relative_path;
use common::read_lines;

#[derive(Parser)]
#[command(name = "Cyclon")]
#[command(version = "0.0.1")]
#[command(author = "Kazuki Kijima <kazukiyoshida96@gmail.com>")]
#[command(about = "Check cyclomatic complexity")]
struct Args {
    #[arg(index(1))]
    path: String,
    // TODO: Add optiosn to show only functions with 7> complexity
}

// TODO: Handle errors correctly, maybe with std::io::Result
fn main() {
    let curr_dir: PathBuf = env::current_dir().unwrap();
    let mut path: PathBuf = PathBuf::from(&Args::parse().path);
    if !path.is_absolute() {
        path = curr_dir.join(&path).to_path_buf();
    }

    if !path.exists() {
        let common_dir: PathBuf = get_common_directory(&path, &curr_dir);
        let rel_path: PathBuf = get_relative_path(&path, &common_dir.parent().unwrap()).unwrap();
        println!("There is no such file or directory: {}", rel_path.display());
        exit(1);
    }

    if !path.is_dir() {
        let contents: Vec<String> = read_lines(&path);
        let result: HashMap<&str, u8> = count(&contents);
        // TODO: Extract and clean the common operation
        let common_dir: PathBuf = get_common_directory(&path, &curr_dir);
        let rel_path: PathBuf = get_relative_path(&path, &common_dir.parent().unwrap()).unwrap();
        print_result(&rel_path, &result);
        if max_complexity(&result) > 7 {
            println!("The complexity of some functions is greater than 7.");
            exit(1);
        }
    } else {
        let paths: Vec<PathBuf> = get_all_paths_in_directory(&path);
        let python_files: Vec<PathBuf> = filter_python_files(&paths);
        let mut max: u8 = 0;
        for path in python_files {
            let contents: Vec<String> = read_lines(&path);
            let result: HashMap<&str, u8> = count(&contents);
            let common_dir: PathBuf = get_common_directory(&path, &curr_dir);
            let rel_path: PathBuf =
                get_relative_path(&path, &common_dir.parent().unwrap()).unwrap();
            print_result(&rel_path, &result);
            max = cmp::max(max_complexity(&result), max);
        }
        if max > 7 {
            println!("The complexity of some functions is greater than 7.");
            exit(1);
        }
    }
}

fn max_complexity(result: &HashMap<&str, u8>) -> u8 {
    let mut max: u8 = 0;
    for complexity in result.values() {
        if *complexity > max {
            max = *complexity;
        }
    }
    max
}
