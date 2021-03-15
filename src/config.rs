use std::collections::HashMap;
use std::env::current_dir;
use std::fs::{File, OpenOptions};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::rustgen_error::RustgenError;

/// Available application config
///
/// # Options
///
/// | Name | Description | Default |
/// | --- | --- | --- |
/// | `template_path` | Path to the templates root directory. Containing structure has to match "_generator/TYPE/ACTION/template.hbs" | `_generator` |
/// | `defaults` | Default variables set in templates (e.g. the name of your *main* plugin). | *None* |
///
/// # Example
///
/// ```yaml
/// template_path: ".generator"
/// defaults:
///     basepath: "./plugin/MyPlugin"
/// ```
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ApplicationConfig {
    #[serde(default = "default_template_path")]
    pub template_path: String,
    #[serde(default)]
    pub default: HashMap<String, String>,
}

/// Reads the Application config from the paths (in the following order)
/// - ./.rustgenrc.yaml
/// - ./.rustgenrc.yml
///
pub fn read() -> ApplicationConfig {
    let file = open_rc_file(".rustgenrc.yaml").or(open_rc_file(".rustgenrc.yml"));

    if let Ok(file) = file {
        return serde_yaml::from_reader(file).unwrap_or_else(|error| {
            eprintln!("rustgenrc.yaml syntax is invalid: {:?}", error);

            ApplicationConfig::default()
        });
    }

    ApplicationConfig::default()
}

fn open_rc_file<P: AsRef<Path>>(path: P) -> Result<File, RustgenError> {
    let current_dir = current_dir().unwrap_or(PathBuf::new());

    OpenOptions::new()
        .read(true)
        .write(false)
        .create(false)
        .open(current_dir.join(path))
        .or(Err(RustgenError::new("Could not find .rustgenrc.yaml")))
}

fn default_template_path() -> String {
    String::from("_generator")
}

impl Default for ApplicationConfig {
    fn default() -> Self {
        ApplicationConfig {
            template_path: String::from("_generator"),
            default: Default::default(),
        }
    }
}
