use std::fs;
use std::path::{Path, PathBuf};

pub fn get_all_paths_in_directory(dir: &Path) -> Vec<PathBuf> {
    println!("get_all_paths_in_directory: {}", dir.display());
    let mut paths = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let path: PathBuf = entry.unwrap().path();
        if path.is_dir() {
            // Recursively add paths from subdirectories
            let sub_paths = get_all_paths_in_directory(&path);
            paths.extend(sub_paths);
        } else {
            paths.push(path.clone())
        }
    }
    paths
}

pub fn filter_python_files(paths: &Vec<PathBuf>) -> Vec<PathBuf> {
    let mut python_paths = Vec::new();
    for path in paths {
        if path.extension().and_then(|s| s.to_str()) != Some("py") {
            continue;
        }
        python_paths.push(path.clone());
    }
    python_paths
}

pub fn read_lines(path: &PathBuf) -> Vec<String> {
    let mut result = Vec::new();

    for line in fs::read_to_string(path).unwrap().lines() {
        result.push(line.trim().to_string());
    }

    result
}

pub fn get_common_directory(path1: &Path, path2: &Path) -> PathBuf {
    let mut common_path = PathBuf::new();
    let mut components1 = path1.components();
    let mut components2 = path2.components();

    loop {
        match (components1.next(), components2.next()) {
            (Some(comp1), Some(comp2)) if comp1 == comp2 => common_path.push(comp1),
            _ => break,
        }
    }

    common_path
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_filter_python_files() {
        let paths = vec![
            PathBuf::from("script.py"),
            PathBuf::from("README.md"),
            PathBuf::from("module.rs"),
            PathBuf::from("test.py"),
        ];
        let expected = vec![PathBuf::from("script.py"), PathBuf::from("test.py")];
        let result = filter_python_files(&paths);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_filter_python_files_no_python_files() {
        let paths = vec![
            PathBuf::from("README.md"),
            PathBuf::from("module.rs"),
            PathBuf::from("Cargo.toml"),
        ];
        let expected: Vec<PathBuf> = vec![];
        let result = filter_python_files(&paths);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_filter_python_files_all_python_files() {
        let paths = vec![
            PathBuf::from("script1.py"),
            PathBuf::from("script2.py"),
            PathBuf::from("script3.py"),
        ];
        let expected = paths.clone();
        let result = filter_python_files(&paths);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_common_directory() {
        let path1 = PathBuf::from("/home/user/documents/project/file.txt");
        let path2 = PathBuf::from("/home/user/documents/notes/notes.txt");
        let expected = PathBuf::from("/home/user/documents");
        let result = get_common_directory(&path1, &path2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_no_common_directory() {
        let path1 = PathBuf::from("/home/user/documents/project/file.txt");
        let path2 = PathBuf::from("/var/log/syslog");
        let expected = PathBuf::from("/");
        let result = get_common_directory(&path1, &path2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_root_common_directory() {
        let path1 = PathBuf::from("/home/user/documents/project/file.txt");
        let path2 = PathBuf::from("/home/user/documents/project/file2.txt");
        let expected = PathBuf::from("/home/user/documents/project");
        let result = get_common_directory(&path1, &path2);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_same_paths() {
        let path1 = PathBuf::from("/home/user/documents/project/file.txt");
        let path2 = PathBuf::from("/home/user/documents/project/file.txt");
        let expected = PathBuf::from("/home/user/documents/project/file.txt");
        let result = get_common_directory(&path1, &path2);
        assert_eq!(result, expected);
    }
}
