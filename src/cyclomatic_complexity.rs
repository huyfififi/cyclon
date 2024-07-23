use std::collections::HashMap;

pub fn count(contents: &Vec<String>) -> HashMap<String, u8> {
    let mut result = HashMap::new();
    for line in contents {
        result.insert(line.clone(), 1);
    }

    result
}
