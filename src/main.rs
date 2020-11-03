extern crate handlebars;
#[macro_use]
extern crate serde;
extern crate serde_yaml;

use std::{env, fs};
use std::collections::{BTreeMap, HashMap};

use clap::{App, Arg, ArgMatches};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde::private::de::IdentifierDeserializer;

use crate::console::get_arguments;
use crate::rustgen_error::RustgenResult;
use crate::template::{Processor, TemplateHeader, Writer};

mod rustgen_error;
mod template;
mod console;

fn generate(named: HashMap<String, String>, mapped: HashMap<String, String>) -> RustgenResult<()> {
    let t_type = String::from(mapped.get("type").unwrap());
    let action = String::from(mapped.get("action").unwrap());
    let name = String::from(mapped.get("name").unwrap());

    let cwd = env::current_dir()?;

    let template = fs::read_to_string(cwd.join(format!(
        "_generator/{}/{}/index.hbs",
        &t_type,
        &action
    )))?;

    let processor = Processor::new(template).unwrap();
    let mut data = BTreeMap::<String, String>::new();
    data.insert(String::from("type"), t_type);
    data.insert(String::from("action"), action);
    data.insert(String::from("name"), name);

    for (key, value) in &named {
        data.insert(key.clone(), value.clone());
    }

    let (header, template) = processor.extract_config_template(data)?;
    println!("{:?}", template);
    println!("{:?}", header);

    Writer::new(header, template).run_action();

    Ok(())
}

fn main() -> RustgenResult<()> {
    let (named, mapped, _) = get_arguments(vec!["type", "action", "name"]);

    generate(named, mapped)?;

    Ok(())
}
