use std::fs;
use std::path::{Path, PathBuf};

pub fn get_all_paths_in_directory(dir: &Path) -> Vec<PathBuf> {
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

pub fn read_lines(file: &PathBuf) -> Vec<String> {
    let mut result = Vec::new();

    for line in fs::read_to_string(file).unwrap().lines() {
        result.push(line.trim().to_string());
    }

    result
}

pub fn _get_relative_path(full_path: &Path, base_path: &Path) -> Result<PathBuf, String> {
    match full_path.strip_prefix(base_path) {
        Ok(relative_path) => Ok(relative_path.to_path_buf()),
        Err(_) => Err(format!(
            "'{}' is not a prefix of '{}'",
            base_path.display(),
            full_path.display()
        )),
    }
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
    fn test_relative_path_success() {
        let base_path = PathBuf::from("/home/user/documents");
        let full_path = PathBuf::from("/home/user/documents/project/file.txt");
        let expected = PathBuf::from("project/file.txt");
        let result = _get_relative_path(&full_path, &base_path).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_relative_path_failure() {
        let base_path = PathBuf::from("/home/user/docs");
        let full_path = PathBuf::from("/home/user/documents/project/file.txt");
        let result = _get_relative_path(&full_path, &base_path);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "'/home/user/docs' is not a prefix of '/home/user/documents/project/file.txt'"
        );
    }

    #[test]
    fn test_relative_path_root() {
        let base_path = PathBuf::from("/");
        let full_path = PathBuf::from("/home/user/documents/project/file.txt");
        let expected = PathBuf::from("home/user/documents/project/file.txt");
        let result = _get_relative_path(&full_path, &base_path).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_relative_path_same_path() {
        let base_path = PathBuf::from("/home/user/documents/project/file.txt");
        let full_path = PathBuf::from("/home/user/documents/project/file.txt");
        let expected = PathBuf::from("");
        let result = _get_relative_path(&full_path, &base_path).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_relative_path_no_common_prefix() {
        let base_path = PathBuf::from("/var/log");
        let full_path = PathBuf::from("/home/user/documents/project/file.txt");
        let result = _get_relative_path(&full_path, &base_path);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "'/var/log' is not a prefix of '/home/user/documents/project/file.txt'"
        );
    }
}
