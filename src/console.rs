use std::collections::HashMap;
use std::env;

pub fn get_arguments(positional_names: Vec<impl ToString>) -> (HashMap<String, String>, HashMap<String, String>, Vec<String>) {
    // Skipping the first as this is the name of the program
    let args: Vec<String> = env::args().skip(1).collect();
    let mut positional = Vec::new();
    let mut named = HashMap::new();

    for arg in &args {
        if arg.starts_with("--") {
            let name: String = arg.chars().skip(2).take_while(|c| c.ne(&'=')).collect();
            let value: String = arg.chars().skip_while(|c| c.ne(&'=')).skip(1).collect();

            named.insert(name, value);
        }

        if arg.starts_with("-") {
            continue; // I dont care (yet) about short arguments
        }

        positional.push(arg.clone());
    }

    let mut positional_mapped = HashMap::new();

    for (index, value) in positional.iter().enumerate() {
        if let Some(name) = positional_names.get(index) {
            positional_mapped.insert(name.to_string(), value.clone());
        }
    }

    (named, positional_mapped, positional)
}