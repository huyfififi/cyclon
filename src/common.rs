use std::fs;
use std::io;
use std::path::Path;

pub fn get_all_paths_in_directory(dir: &Path) -> io::Result<Vec<String>> {
    let mut paths = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            // Recursively add paths from subdirectories
            let sub_paths = get_all_paths_in_directory(&path)?;
            paths.extend(sub_paths);
        } else {
            // Convert the path to a String and add it to the list
            if let Some(path_str) = path.to_str() {
                paths.push(path_str.to_string());
            }
        }
    }
    Ok(paths)
}

pub fn filter_python_files(paths: &Vec<String>) -> Vec<String> {
    let mut python_paths = Vec::new();
    for path in paths {
        if !path.ends_with(&(".py")) {
            continue;
        }
        python_paths.push(path.clone())
    }
    python_paths
}
