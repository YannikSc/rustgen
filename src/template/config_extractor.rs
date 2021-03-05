use std::collections::BTreeMap;

use convert_case::{Case, Casing};
use handlebars::{Context, Handlebars, Helper, HelperDef, JsonValue, RenderContext, RenderError, ScopedJson};
use regex::Regex;

use crate::rustgen_error::RustgenResult;
use crate::template::{Processor, TemplateHeader, WriteAction};

struct RegexReplace;

impl Default for WriteAction {
    fn default() -> Self {
        Self::CreateFile
    }
}

const MARK_SYMBOL: &str = "---";

macro_rules! add_case_helper {
    ($bars: expr, $name: ident, $case: expr) => {
        handlebars_helper!($name: |string: str| {
            string.to_case($case)
        });

        $bars.register_helper(stringify!($name), Box::new($name));
    };
}

impl HelperDef for RegexReplace {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<Option<ScopedJson<'reg, 'rc>>, RenderError> {
        let params = h.params();

        if params.len() != 3 {
            return Err(RenderError::new("Invalid replace arguments. Usage: {{replace <input> <from> <to>}}"));
        }

        if let [input, from, to] = &params[..] {
            let input = input.value().as_str().ok_or(RenderError::new("Argument input has to be a string"))?;
            let from = from.value().as_str().ok_or(RenderError::new("Argument from has to be a string"))?;
            let to = to.value().as_str().ok_or(RenderError::new("Argument to has to be a string"))?;

            let result = Regex::new(from).unwrap().replace_all(input, to).to_string();

            return Ok(Some(ScopedJson::Derived(JsonValue::String(result))));
        }


        Err(RenderError::new("Could not replace. Unknown error."))
    }
}

handlebars_helper!(replace: |input: str, from: str, to: str| {
    input.replace(from, to)
});

impl Processor {
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
        add_case_helper!(bars, upper_case, Case::Upper);
        add_case_helper!(bars, lower_case, Case::Lower);
        add_case_helper!(bars, title_case, Case::Title);
        add_case_helper!(bars, toggle_case, Case::Toggle);
        add_case_helper!(bars, camel_case, Case::Camel);
        add_case_helper!(bars, pascal_case, Case::Pascal);
        add_case_helper!(bars, snake_case, Case::Snake);
        add_case_helper!(bars, screaming_snake_case, Case::ScreamingSnake);
        add_case_helper!(bars, kebab_case, Case::Kebab);
        add_case_helper!(bars, cobol_case, Case::Cobol);
        add_case_helper!(bars, train_case, Case::Train);
        add_case_helper!(bars, flat_case, Case::Flat);
        add_case_helper!(bars, upper_flat_case, Case::UpperFlat);
        add_case_helper!(bars, alternating_case, Case::Alternating);

        bars.register_helper("regex_replace", Box::new(RegexReplace));
        bars.register_helper("replace", Box::new(replace));

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
