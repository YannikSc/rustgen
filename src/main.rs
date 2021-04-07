#[macro_use]
extern crate handlebars;
#[macro_use]
extern crate serde;
extern crate serde_yaml;

use std::{env, fs};
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;

use crate::console::get_arguments;
use crate::rustgen_error::{RustgenError, RustgenResult};
use crate::template::{PreProcessor, Writer};

mod config;
mod console;
mod rustgen_error;
mod template;

fn insert_default_data(defaults: &HashMap<String, String>, data: &mut BTreeMap<String, String>) {
    for (key, value) in defaults {
        data.insert(key.clone(), value.clone());
    }
}

fn generate(named: HashMap<String, String>, mapped: HashMap<String, String>) -> RustgenResult<()> {
    let t_type = String::from(mapped.get("type").ok_or(RustgenError::new("Missing parameter 'type'"))?);
    let action = String::from(mapped.get("action").ok_or(RustgenError::new("Missing parameter 'action'"))?);
    let name = String::from(mapped.get("name").ok_or(RustgenError::new("Missing parameter 'name'"))?);
    let mut data = BTreeMap::<String, String>::new();
    let config = config::read();
    insert_default_data(&config.default, &mut data);

    for (key, value) in &named {
        data.insert(key.clone(), value.clone());
    }

    let cwd = env::current_dir()?;
    let templates = cwd.join(format!("{}/{}/{}", &config.template_path, &t_type, &action));

    data.insert(String::from("type"), t_type);
    data.insert(String::from("action"), action);
    data.insert(String::from("name"), name);

    for directory in fs::read_dir(templates.clone()).or(Err(RustgenError::new(format!(
        "Templates not found in {}",
        templates.display()
    ))))? {
        if let Ok(entry) = directory {
            if entry.path().is_file() {
                generate_file(entry.path(), data.clone())?;
            }
        }
    }

    Ok(())
}

fn generate_file(path: PathBuf, data: BTreeMap<String, String>) -> RustgenResult<()> {
    let template = fs::read_to_string(path)?;
    let processor = PreProcessor::new(template).unwrap();
    let (header, template) = processor.extract_config_template(data)?;

    Writer::new(header, template).run_action()?;

    Ok(())
}

fn main() {
    let (named, mapped, _) = get_arguments(vec!["type", "action", "name"]);

    match generate(named, mapped) {
        Ok(_) => {}
        Err(error) => {
            eprintln!("{}", &error);
        }
    }
}
