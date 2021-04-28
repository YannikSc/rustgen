//! # Rustgen
//!
//! Inspired by [Hygen](https://www.hygen.io/) is this a code generator written in Rust!
//! While its pretty different from Hygen itself, it tries to archive the same goal: generating code.
//!
//! It follows the same principle of writing generators (templates), filling them out and placing them, where they belong.
//! And as well as creating new files it also generates code into existing files.
//!
//! This application is not bound to any language, so it's not just for Rust but written in Rust.
//!
//! ## Why?
//!
//! Because I wanted to do something in Rust... and I don't like the idea of having JavaScript (Node.JS) installed just for generating code. Rust can do this aswell but without a bigger runtime around it.
//!
//! ## (Some) Documentation
//!
//! ### Usage
//!
//! Rustgen gets invoked with at least 3 positional arguments and depending on the template some
//! named parameters as well. The parameters will entirely be forwarded into the template so you can
//! use them to modify, for example, if your generated code should end up in some subdirectory.
//!
//! The command will look like this:
//!
//! ```bash
//! # The raw command
//! rustgen {type} {action} {name}
//!
//! # An example
//! rustgen entity generate my-entity --subdirectory=entities
//! ```
//!
//! The arguments `type`, `action` and `name` will be available in the template as well as the (in
//! this case) parameter `subdirectory`.
//!
//! ### Getting started
//!
//! To get started with rustgen you have to create a folder called `_generator` in your project.
//! This folder will contain all of your templates. The folder structure inside has to match the
//! following pattern: `{type}/{action}/*.hbs`.
//!
//! The `type` and `action` comes from the command mentioned in the [Usage](#usage). The `*.hbs`
//! stands for all possible templates that should be rendered. So you can in the output generate
//! multiple files and/or append to multiple files. In short: its very flexible. Each template can
//! be seen as a small script that gets executed after each other so combining various options is no
//! problem.
//!
//! ### The template
//!
//! The template are written in the handlebars template language. Which features variables, basic
//! control structures (like loops and conditions) and helpers. For some more information on how
//! handlebars works and what it can you can take a look into the official [handlebars docs](https://handlebarsjs.com/guide/#language-features).
//! (Which are not fully applicable to the here used engine, as this one is a Rust port and **not**
//! the original JavaScript version of it).
//!
//! As already mentioned, you have a couple variables inside the template available like the `type`,
//! `action` and `name`, as well as all of your given parameters and default values from your config
//! (explained later on).
//!
//! For helpers there are (besides of the, in the [library included](https://docs.rs/handlebars/3.5.4/handlebars/#built-in-helpers)
//! ones) the following which are documented in their links:
//!
//! - [RegexReplaceHelper](crate::template::RegexReplaceHelper)
//! - [DefaultHelper](crate::template::DefaultHelper)
//! - [SetHelper](crate::template::SetHelper)
//! - [ConcatHelper](crate::template::ConcatHelper)
//! - [TimeHelper](crate::template::TimeHelper)
//!
//! And helpers for changing the case of a text, for the following formats:
//!
//! - upper_case
//! - lower_case
//! - title_case
//! - toggle_case
//! - camel_case
//! - pascal_case
//! - snake_case
//! - screaming_snake_case
//! - kebab_case
//! - cobol_case
//! - train_case
//! - flat_case
//! - upper_flat_case
//! - alternating_case
//!
//! They don't have a specific documentation as they all work in the same way: `{{upper_case variable_name}}`.
//! For an overview of what they produce you can take a look at the [crate's documentation](https://docs.rs/convert_case/0.4.0/convert_case/enum.Case.html#variants).
//!
//! ## Configure rustgen
//!
//! For configuring rustgen you have to create a .rustgenrc.yml or .yaml file in your project folder.
//! Rustgen will then detect, that there is a config file and will start using it. For a brief
//! overview of what the config is capable you can follow [this link](crate::config::ApplicationConfig)
//! (the config cannot do *that* much for now).
//!

#[macro_use]
extern crate handlebars;
#[macro_use]
extern crate serde;
extern crate serde_yaml;

use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use std::{env, fs};

use crate::console::get_arguments;
use crate::rustgen_error::{RustgenError, RustgenResult};
use crate::template::{PreProcessor, Writer};

pub mod config;
pub mod console;
pub mod rustgen_error;
pub mod template;

fn insert_default_data(defaults: &HashMap<String, String>, data: &mut BTreeMap<String, String>) {
    for (key, value) in defaults {
        data.insert(key.clone(), value.clone());
    }
}

fn generate(named: HashMap<String, String>, mapped: HashMap<String, String>) -> RustgenResult<()> {
    let t_type = String::from(
        mapped
            .get("type")
            .ok_or(RustgenError::new("Missing parameter 'type'"))?,
    );
    let action = String::from(
        mapped
            .get("action")
            .ok_or(RustgenError::new("Missing parameter 'action'"))?,
    );
    let name = String::from(
        mapped
            .get("name")
            .ok_or(RustgenError::new("Missing parameter 'name'"))?,
    );
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
