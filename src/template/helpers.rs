use std::rc::Rc;

use convert_case::{Case, Casing};
use handlebars::{Context, Handlebars, Helper, HelperDef, JsonValue, PathAndJson, RenderContext, RenderError, ScopedJson};
use regex::Regex;

use crate::template::{ConcatHelper, DefaultHelper, RegexReplaceHelper, SetHelper};

macro_rules! add_case_helper {
    ($bars: expr, $name: ident, $case: expr) => {
        handlebars_helper!($name: |string: str| {
            string.to_case($case)
        });

        $bars.register_helper(stringify!($name), Box::new($name));
    };
}

handlebars_helper!(replace: |input: str, from: str, to: str| {
    input.replace(from, to)
});

/// Adds the template helpers to the given handlebars instance
pub fn add_helpers(bars: &mut Handlebars) {
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

    bars.register_helper("regex_replace", Box::new(RegexReplaceHelper));
    bars.register_helper("concat", Box::new(ConcatHelper));
    bars.register_helper("default", Box::new(DefaultHelper));
    bars.register_helper("set", Box::new(SetHelper));
    bars.register_helper("replace", Box::new(replace));
}

impl HelperDef for RegexReplaceHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        helper: &Helper<'reg, 'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<Option<ScopedJson<'reg, 'rc>>, RenderError> {
        let params = helper.params();

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

impl ConcatHelper {
    fn stringify_params(&self, params: &Vec<PathAndJson>) -> Vec<String> {
        let mut strings = vec![];

        for value in params {
            strings.push(value.render());
        }

        strings
    }
}

impl HelperDef for ConcatHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        helper: &Helper<'reg, 'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<Option<ScopedJson<'reg, 'rc>>, RenderError> {
        let params = helper.params();
        let strings = self.stringify_params(params);

        Ok(Some(ScopedJson::Derived(JsonValue::String(strings.join(&String::new())))))
    }
}

impl HelperDef for DefaultHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        helper: &Helper<'reg, 'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
    ) -> Result<Option<ScopedJson<'reg, 'rc>>, RenderError> {
        let arguments = helper.params();
        let value = arguments.get(0).ok_or(RenderError::new("Missing value argument"))?;
        let fallback = arguments.get(1).ok_or(RenderError::new("Missing fallback argument"))?;

        if value.is_value_missing() {
            return Ok(Some(ScopedJson::Derived(fallback.value().clone())));
        }

        let value = value.value().clone();

        if value == 0 || value == "" || value == String::new() || value == false || value.is_null() {
            return Ok(Some(ScopedJson::Derived(fallback.value().clone())));
        }

        Ok(Some(ScopedJson::Derived(value)))
    }
}

impl HelperDef for SetHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        helper: &Helper<'reg, 'rc>,
        _: &'reg Handlebars<'reg>,
        default_context: &'rc Context,
        render_context: &mut RenderContext<'reg, 'rc>,
    ) -> Result<Option<ScopedJson<'reg, 'rc>>, RenderError> {
        let arguments = helper.params();
        let variable_name = arguments.get(0).ok_or(RenderError::new("Missing variable_name argument"))?;
        let content = arguments.get(1).ok_or(RenderError::new("Missing content argument"))?;

        let variable_name = variable_name.render();
        let content = content.value();

        let mut ctx = render_context.context().unwrap_or(Rc::new(default_context.clone())).as_ref().clone();
        let mut data = ctx.data_mut();

        data[variable_name] = content.clone();

        render_context.set_context(ctx);

        Ok(None)
    }
}
