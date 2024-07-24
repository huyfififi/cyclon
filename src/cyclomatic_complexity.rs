use std::collections::HashMap;
use std::path::PathBuf;

// ref: https://www.ibm.com/docs/en/raa/6.1?topic=metrics-cyclomatic-complexity
// TODO: Correct this list of keywords used to count cyclomatic complexity
static PYTHON_KEYWORDS: &[(&str, usize)] = &[("if ", 4), ("elif ", 4), ("for ", 4), ("case ", 8)];

fn extract_function_name(func: &String) -> &str {
    let v: Vec<&str> = func.split("(").collect();
    return v[0].trim();
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
    let mut curr_func: &str = ""; // without spaces
    let mut curr_comp: u8 = 0;
    let mut curr_indent: usize = 0;

    for line in contents {
        // TODO: skip multiline comments
        if line.clone().trim().starts_with("#") {
            continue; // skip comments
        }

        let def_index = line.find("def ");

        // if the line does not contain "def ",
        // then it might not be a function definition.
        if def_index.is_none() {
            if curr_func == "" {
                continue;
            }
            for (keyword, indent) in PYTHON_KEYWORDS {
                let index = line.find(keyword);
                if index.is_none() {
                    continue;
                }
                // LIMITATION: this logic assumes Python indent is 4 spaces
                // TODO: make this logic more flexible
                // skip if the indent is not 4 spaces <=> might be a nested function
                if index.unwrap() != curr_indent + indent {
                    continue;
                }
                curr_comp += 1;
                break; // check only one keyword per line
            }
            continue;
        }

        if curr_func != "" {
            if def_index.unwrap() != curr_indent {
                continue;
            }
            result.insert(curr_func, curr_comp);
        }
        let func_name: &str = extract_function_name(&line);
        if func_name != "" {
            curr_func = func_name;
            curr_comp = 0;
            curr_indent = def_index.unwrap();
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
