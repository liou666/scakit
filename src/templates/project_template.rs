use include_dir::{include_dir, Dir};
use std::path::Path;
use std::{fs, io};
#[derive(Debug, Clone)]
pub enum ProjectTemplate {
    TypeScript,
}
static TEMPLATES_DIR: Dir = include_dir!("templates");
impl ProjectTemplate {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::TypeScript => "TypeScript",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "TypeScript" => Some(Self::TypeScript),
            _ => None,
        }
    }

    pub fn get_template_dir(&self) -> &'static str {
        match self {
            Self::TypeScript => "templates/ts",
        }
    }

    pub fn create_project_files(&self, target_dir: &Path) -> io::Result<()> {
        let template_name = match self {
            ProjectTemplate::TypeScript => "ts",
        };

        if let Some(template_dir) = TEMPLATES_DIR.get_dir(template_name) {
            for file in template_dir.files() {
                let relative_path = file.path();
                let target_file_path = target_dir.join(relative_path);

                if let Some(parent) = target_file_path.parent() {
                    fs::create_dir_all(parent)?;
                }

                fs::write(&target_file_path, file.contents())?;
            }
        }

        Ok(())
    }

    pub fn available_templates() -> Vec<&'static str> {
        vec!["TypeScript"]
    }
}
