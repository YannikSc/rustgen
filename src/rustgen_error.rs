use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::{fmt, io};

use handlebars::TemplateRenderError;

pub type RustgenResult<T> = Result<T, RustgenError>;

pub struct RustgenError {
    message: String,
    debug: String,
}

impl RustgenError {
    pub fn new(message: impl ToString) -> Self {
        Self {
            debug: message.to_string(),
            message: message.to_string(),
        }
    }
}

impl Error for RustgenError {}

impl Display for RustgenError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Debug for RustgenError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.debug)
    }
}

impl From<serde_yaml::Error> for RustgenError {
    fn from(parent: serde_yaml::Error) -> Self {
        Self {
            message: format!("Could not (de-)serialize: {}", parent),
            debug: format!("{:?}", parent),
        }
    }
}

impl From<io::Error> for RustgenError {
    fn from(parent: io::Error) -> Self {
        Self {
            message: format!(
                "Could not fulfill file operation: {}\n Error: {:?}",
                parent,
                parent.kind()
            ),
            debug: format!("{:?}", parent),
        }
    }
}

impl From<TemplateRenderError> for RustgenError {
    fn from(parent: TemplateRenderError) -> Self {
        Self {
            message: format!("Could not render template: {}", parent),
            debug: format!("{:?}", parent),
        }
    }
}
