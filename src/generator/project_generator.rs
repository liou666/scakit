use crate::{
    error::{template_error::Result, template_error::TemplateError},
    templates::project_template::ProjectTemplate,
};
use std::{fmt::Display, path::Path, process::Command};

pub struct ProjectGenerator {
    template: ProjectTemplate,
    project_name: String,
}

impl ProjectGenerator {
    pub fn new(template: ProjectTemplate, project_name: String) -> Self {
        Self {
            template,
            project_name,
        }
    }

    pub fn create_project(&self) -> Result<()> {
        let target_path = Path::new(&self.project_name);

        if target_path.exists() {
            return Err(TemplateError::DirectoryExists(self.project_name.clone()));
        }

        self.template
            .create_project_files(&target_path)
            .map_err(|e| TemplateError::Io(e))?;

        println!("✨ Project '{}' created successfully!", self.project_name);

        Ok(())
    }

    fn validate_paths(&self, template_dir: &Path, target_path: &Path) -> Result<()> {
        if !template_dir.exists() {
            return Err(TemplateError::TemplateNotFound(
                self.template.as_str().to_string(),
            ));
        }

        if target_path.exists() {
            return Err(TemplateError::DirectoryExists(self.project_name.clone()));
        }

        Ok(())
    }

    fn initialize_project(&self, project_path: &Path) -> Result<()> {
        println!("📦 Installing dependencies...");
        let output = Command::new("pnpm")
            .arg("install")
            .current_dir(project_path)
            .output()?;

        if !output.status.success() {
            return Err(TemplateError::DependencyInstallation(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }

        Ok(())
    }
}
