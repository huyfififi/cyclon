use std::collections::HashMap;
use std::path::PathBuf;

// ref: https://www.ibm.com/docs/en/raa/6.1?topic=metrics-cyclomatic-complexity
// TODO: Correct this list of keywords used to count cyclomatic complexity
// "if " matches both "if" and "elif"
static PYTHON_KEYWORDS: &[&str] = &["if ", "for ", "case "];

fn extract_function_name(func: &String) -> &str {
    let v: Vec<&str> = func.split("(").collect();
    return v[0];
}

/*
my first attempt to count cyclomatic complexity involves creating a struct
to hold the current function name and its complexity.
but I could not figure out how to implement the logic as I'm new to Rust.
TODO: Implement the logic using a struct
*/
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

pub fn print_result(path: &PathBuf, result: &HashMap<&str, u8>) {
    println!("{}", path.display());
    println!("Found {} functions.", result.len());
    for (func_name, complexity) in result.into_iter() {
        println!("{}\t{}", complexity, func_name);
    }
}
