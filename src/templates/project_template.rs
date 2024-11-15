#[derive(Debug, Clone)]
pub enum ProjectTemplate {
    TypeScript,
}

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

    pub fn available_templates() -> Vec<&'static str> {
        vec!["TypeScript"]
    }
}
