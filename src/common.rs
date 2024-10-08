use std::error::Error;
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

pub fn filter_python_files(paths: &Vec<PathBuf>) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    if paths.is_empty() {
        return Err("No paths provided".into());
    }

    let mut python_paths = Vec::new();
    for path in paths {
        if path.extension().and_then(|s| s.to_str()) != Some("py") {
            continue;
        }
        python_paths.push(path.clone());
    }
    Ok(python_paths)
}

pub fn read_lines(path: &PathBuf) -> Vec<String> {
    let mut result = Vec::new();

    for line in fs::read_to_string(path).unwrap().lines() {
        result.push(line.to_string());
    }

    result
}

pub fn get_common_directory(path1: &Path, path2: &Path) -> PathBuf {
    let mut common_dir = PathBuf::new();
    let mut components1 = path1.components();
    let mut components2 = path2.components();

    loop {
        match (components1.next(), components2.next()) {
            (Some(comp1), Some(comp2)) if comp1 == comp2 => common_dir.push(comp1),
            _ => break,
        }
    }

    common_dir
}

pub fn get_relative_path(full_path: &Path, base_path: &Path) -> Result<PathBuf, String> {
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

    fn assert_result_eq<T, E>(result: Result<T, E>, expected: Result<T, E>)
    where
        T: PartialEq + std::fmt::Debug,
        E: std::fmt::Debug + std::fmt::Display,
    {
        match (result, expected) {
            (Ok(result_vec), Ok(expected_vec)) => assert_eq!(result_vec, expected_vec),
            (Err(result_err), Err(expected_err)) => {
                assert_eq!(result_err.to_string(), expected_err.to_string())
            }
            _ => panic!("Result and expected do not match"),
        }
    }

    #[test]
    fn test_filter_python_files_empty_paths() {
        let paths: Vec<PathBuf> = vec![];
        let expected: Result<Vec<PathBuf>, Box<dyn Error>> = Err("No paths provided".into());
        let result = filter_python_files(&paths);
        assert_result_eq(result, expected);
    }

    #[test]
    fn test_filter_python_files() {
        let paths = vec![
            PathBuf::from("script.py"),
            PathBuf::from("README.md"),
            PathBuf::from("module.rs"),
            PathBuf::from("test.py"),
        ];
        let expected: Result<Vec<PathBuf>, Box<dyn Error>> =
            Ok(vec![PathBuf::from("script.py"), PathBuf::from("test.py")]);
        let result = filter_python_files(&paths);
        assert_result_eq(result, expected);
    }

    #[test]
    fn test_filter_python_files_no_python_files() {
        let paths = vec![
            PathBuf::from("README.md"),
            PathBuf::from("module.rs"),
            PathBuf::from("Cargo.toml"),
        ];
        let expected: Result<Vec<PathBuf>, Box<dyn Error>> = Ok(vec![]);
        let result = filter_python_files(&paths);
        assert_result_eq(result, expected);
    }

    #[test]
    fn test_filter_python_files_all_python_files() {
        let paths = vec![
            PathBuf::from("script1.py"),
            PathBuf::from("script2.py"),
            PathBuf::from("script3.py"),
        ];
        let expected: Result<Vec<PathBuf>, Box<dyn Error>> = Ok(paths.clone());
        let result = filter_python_files(&paths);
        assert_result_eq(result, expected);
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

    #[test]
    fn test_relative_path_success() {
        let base_path = PathBuf::from("/home/user/documents");
        let full_path = PathBuf::from("/home/user/documents/project/file.txt");
        let expected = PathBuf::from("project/file.txt");
        let result = get_relative_path(&full_path, &base_path).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_relative_path_failure() {
        let base_path = PathBuf::from("/home/user/docs");
        let full_path = PathBuf::from("/home/user/documents/project/file.txt");
        let result = get_relative_path(&full_path, &base_path);
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
        let result = get_relative_path(&full_path, &base_path).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_relative_path_same_path() {
        let base_path = PathBuf::from("/home/user/documents/project/file.txt");
        let full_path = PathBuf::from("/home/user/documents/project/file.txt");
        let expected = PathBuf::from("");
        let result = get_relative_path(&full_path, &base_path).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_relative_path_no_common_prefix() {
        let base_path = PathBuf::from("/var/log");
        let full_path = PathBuf::from("/home/user/documents/project/file.txt");
        let result = get_relative_path(&full_path, &base_path);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "'/var/log' is not a prefix of '/home/user/documents/project/file.txt'"
        );
    }
}
