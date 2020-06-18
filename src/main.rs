extern crate handlebars;
extern crate serde;
extern crate serde_yaml;

use std::{env, fs};
use std::collections::BTreeMap;

use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

use crate::rustgen_error::RustgenResult;

mod rustgen_error;
mod template;

#[derive(Serialize, Deserialize)]
struct TemplateHeader {
    path: String,
}

const MARK_SYMBOL_LENGTH: usize = 3;

fn main() -> RustgenResult<()> {
    let bars = Handlebars::new();
    let cwd = env::current_dir()?;
    let template = fs::read_to_string(cwd.join("_generator/example/new/new_example.lua.hbs"))?;
    let mut data = BTreeMap::new();
    data.insert("name", "Test");

    let mut rendered = bars.render_template(&template, &data)?;
    let yaml_start = rendered.find("---").unwrap_or(0) + MARK_SYMBOL_LENGTH;
    let mut yaml: String = rendered.chars().skip(yaml_start).collect();
    let yaml_end = yaml.find("---").unwrap_or(0);
    yaml = yaml.chars().take(yaml_end).collect();
    rendered = rendered.chars().skip(yaml_end + MARK_SYMBOL_LENGTH + yaml_start).collect();
    rendered = String::from(rendered.trim_start());
    rendered = String::from(rendered.trim_end());

    let header: TemplateHeader = serde_yaml::from_str(yaml.as_str())?;

    let mut path = header.path.clone();
    let last_slash = path.rfind("/").unwrap_or(0);
    let app_dir: String = path.chars().take(last_slash).collect();

    fs::create_dir_all(cwd.join(&app_dir))?;
    fs::write(cwd.join(&header.path), &rendered)?;
    println!("{}", rendered);

    Ok(())
}
