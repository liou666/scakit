use inquire::{Select, Text};

use crate::error::template_error::{Result, TemplateError};
use crate::templates::project_template::ProjectTemplate;

pub fn select_template() -> Result<ProjectTemplate> {
    let template_str = Select::new("Choose a template:", ProjectTemplate::available_templates())
        .prompt()
        .map_err(|e| TemplateError::UserInput(e.to_string()))?;

    ProjectTemplate::from_str(template_str)
        .ok_or_else(|| TemplateError::InvalidTemplate(template_str.to_string()))
}

pub fn input_project_name() -> Result<String> {
    Text::new("Enter project name:")
        .with_initial_value("my_project")
        .with_formatter(&|input| input.to_lowercase().replace(" ", "_"))
        // .with_help_message("Project name should be in lowercase with underscores")
        .prompt()
        .map_err(|e| TemplateError::UserInput(e.to_string()))
}
