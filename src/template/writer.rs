use std::{env, fs};

use crate::rustgen_error::RustgenResult;
use crate::template::{ExtendLocation, TemplateHeader, WriteAction, Writer};

impl Writer {
    pub fn new(header: TemplateHeader, rendered_template: String) -> Self {
        Self { header, rendered_template }
    }

    pub fn run_action(&self) -> RustgenResult<()> {
        match &self.header.action {
            WriteAction::CreateFile => self.action_create_file()?,
            WriteAction::Append(location) => self.action_append(location.clone())?,
        }

        Ok(())
    }

    fn action_create_file(&self) -> RustgenResult<()> {
        let cwd = env::current_dir()?;
        let path = self.header.path.clone();
        let last_part = path.rfind("/").unwrap_or_default();
        let dir_path = path.chars().take(last_part).collect::<String>();

        fs::create_dir_all(cwd.join(dir_path))?;
        fs::write(cwd.join(path), &self.rendered_template);

        Ok(())
    }

    fn action_append(&self, location: ExtendLocation) -> RustgenResult<()> {
        match location {
            ExtendLocation::BeginOfFile => self.append_begin()?,
            ExtendLocation::EndOfFile => self.append_end()?,
            ExtendLocation::After(identifier) => self.append_after(identifier)?,
            ExtendLocation::Before(identifier) => self.append_before(identifier)?
        }

        Ok(())
    }

    fn append_begin(&self) -> RustgenResult<()> {
        let mut content = self.rendered_template.clone();
        content.push_str("\n");
        content.push_str(self.get_content()?.as_str());

        self.write_content(content)?;

        Ok(())
    }

    fn append_end(&self) -> RustgenResult<()> {
        let mut content = self.get_content()?;
        content.push_str(self.rendered_template.as_str());
        content.push_str("\n");

        self.write_content(content)?;

        Ok(())
    }

    fn append_before(&self, identifier: String) -> RustgenResult<()> {
        let mut content = self.get_content()?;

        content = content.replace(
            identifier.as_str(),
            format!("{}\n{}", self.rendered_template, identifier).as_str(),
        );

        self.write_content(content)?;

        Ok(())
    }

    fn append_after(&self, identifier: String) -> RustgenResult<()> {
        let mut content = self.get_content()?;

        content = content.replace(
            identifier.as_str(),
            format!("{}\n{}", identifier, self.rendered_template).as_str(),
        );

        self.write_content(content)?;

        Ok(())
    }

    fn get_content(&self) -> RustgenResult<String> {
        let cwd = env::current_dir()?;

        Ok(fs::read_to_string(cwd.join(self.header.path.clone()))?)
    }

    fn write_content(&self, content: String) -> RustgenResult<()> {
        let cwd = env::current_dir()?;
        fs::write(cwd.join(self.header.path.clone()), &content)?;

        Ok(())
    }
}