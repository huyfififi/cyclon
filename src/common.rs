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

pub fn read_lines(filename: String) -> Vec<String> {
    let mut result = Vec::new();

    for line in fs::read_to_string(filename).unwrap().lines() {
        result.push(line.trim().to_string());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_python_files() {
        let paths: Vec<String> = vec![
            "script.py".to_string(),
            "README.md".to_string(),
            "module.rs".to_string(),
            "test.py".to_string(),
        ];
        let expected: Vec<String> = vec![
            "script.py".to_string(),
            "test.py".to_string(),
        ];
        let result: Vec<String> = filter_python_files(&paths);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_filter_python_files_no_python_files() {
        let paths: Vec<String> = vec![
            "README.md".to_string(),
            "module.rs".to_string(),
            "Cargo.toml".to_string(),
        ];
        let expected: Vec<String> = vec![];
        let result: Vec<String> = filter_python_files(&paths);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_filter_python_files_all_python_files() {
        let paths: Vec<String> = vec![
            "script1.py".to_string(),
            "script2.py".to_string(),
            "script3.py".to_string(),
        ];
        let expected: Vec<String> = paths.clone();
        let result: Vec<String> = filter_python_files(&paths);
        assert_eq!(result, expected);
    }
}
