use thiserror::Error;

pub type Result<T> = std::result::Result<T, TemplateError>;

#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Template '{0}' not found")]
    TemplateNotFound(String),

    #[error("Directory '{0}' already exists")]
    DirectoryExists(String),

    #[error("Invalid template: {0}")]
    InvalidTemplate(String),

    #[error("User input error: {0}")]
    UserInput(String),

    #[error("Dependency installation failed: {0}")]
    DependencyInstallation(String),
}
