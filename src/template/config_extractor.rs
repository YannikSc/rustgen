use std::collections::BTreeMap;

use handlebars::{Context, Handlebars, Helper, HelperDef, JsonValue, RenderContext, RenderError, ScopedJson};
use regex::Regex;

use crate::rustgen_error::RustgenResult;
use crate::template::{PreProcessor, TemplateHeader, WriteAction};
use crate::template::helpers::add_helpers;

impl Default for WriteAction {
    fn default() -> Self {
        Self::CreateFile
    }
}

const MARK_SYMBOL: &str = "---";

impl PreProcessor {
    pub fn new(template: String) -> RustgenResult<Self> {
        Ok(Self {
            template,
        })
    }

    ///
    /// # Return
    ///
    /// - .0 - The extracted header struct
    /// - .1 - The remaining template String
    ///
    pub fn extract_config_template(self, data: BTreeMap<String, String>) -> RustgenResult<(TemplateHeader, String)> {
        let mut bars = Handlebars::new();

        add_helpers(&mut bars);

        let (yaml, template) = self.extract_parts();
        let yaml_rendered = bars.render_template(&yaml, &data)?;
        let template = bars.render_template(&template, &data)?;
        let header: TemplateHeader = serde_yaml::from_str(yaml_rendered.as_str())?;

        Ok((header, template))
    }

    ///
    /// Extracts the template and header part from the template property
    ///
    /// # Return
    ///
    /// - .0 - The extracted header String
    /// - .1 - The remaining String (template)
    ///
    fn extract_parts(&self) -> (String, String) {
        let mut template = self.template.clone();
        let header_yaml_start = template.find(MARK_SYMBOL).unwrap_or(0) + MARK_SYMBOL.len();
        let mut header_yaml: String = template.chars().skip(header_yaml_start).collect();
        let header_yaml_end = header_yaml.find(MARK_SYMBOL).unwrap_or(0);
        header_yaml = header_yaml.chars().take(header_yaml_end).collect();

        template = template.chars().skip(header_yaml_end + MARK_SYMBOL.len() + header_yaml_start).collect();
        template = String::from(template.trim_start());
        template = String::from(template.trim_end());

        (header_yaml, template)
    }
}
