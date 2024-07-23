use std::collections::HashMap;
use std::path::Path;

static PYTHON_KEYWORDS: &[&str] = &["if ", "for "];

fn extract_function_name(func: &String) -> &str {
    let v: Vec<&str> = func.split("(").collect();
    return v[0];
}

// nested functions will break this logic
// only top-level functions are supported
pub fn count(contents: &Vec<String>) -> HashMap<&str, u8> {
    let mut result = HashMap::new();
    let mut curr_func: &str = "";
    let mut curr_comp: u8 = 0;

    for line in contents {
        // TODO: Skip comments

        if line.contains(&"def ") {
            if curr_func != "" {
                result.insert(curr_func, curr_comp);
            }
            let extracted = extract_function_name(&line);
            if extracted != "" {
                curr_func = extracted;
                curr_comp = 0;
            }
            continue;
        }

        for keyword in PYTHON_KEYWORDS {
            if line.contains(keyword) {
                curr_comp += 1;
                continue;
            }
        }
    }

    // TODO: Extract the common operation
    if curr_func != "" {
        result.insert(curr_func, curr_comp);
    }

    result
}

pub fn print_result(path: &Path, result: &HashMap<&str, u8>) {
    println!("{}", path.display());
    for (func_name, complexity) in result.into_iter() {
        println!("{}\t{}", complexity, func_name);
    }
}
