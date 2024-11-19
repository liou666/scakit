use include_dir::{include_dir, Dir, DirEntry};
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

    fn copy_dir_contents(template_dir: &Dir, target_dir: &Path) -> io::Result<()> {
        for entry in template_dir.entries() {
            match entry {
                DirEntry::Dir(subdir) => {
                    // remove the template directory from the path
                    let relative_path = subdir
                        .path()
                        .strip_prefix(template_dir.path())
                        .unwrap_or(subdir.path());
                    let target_path = target_dir.join(relative_path);
                    fs::create_dir_all(&target_path)?;

                    for entry in subdir.entries() {
                        match entry {
                            DirEntry::Dir(nested_dir) => {
                                let nested_target = target_dir.join(
                                    nested_dir
                                        .path()
                                        .strip_prefix(template_dir.path())
                                        .unwrap_or(nested_dir.path()),
                                );
                                fs::create_dir_all(&nested_target)?;
                                for file in nested_dir.files() {
                                    let file_target =
                                        nested_target.join(file.path().file_name().unwrap());
                                    fs::write(file_target, file.contents())?;
                                }
                            }
                            DirEntry::File(file) => {
                                let file_target =
                                    target_path.join(file.path().file_name().unwrap());
                                fs::write(file_target, file.contents())?;
                            }
                        }
                    }
                }
                DirEntry::File(file) => {
                    let target_path = target_dir.join(file.path().file_name().unwrap());
                    fs::write(target_path, file.contents())?;
                }
            }
        }
        Ok(())
    }

    pub fn create_project_files(&self, target_dir: &Path) -> io::Result<()> {
        let template_name = match self {
            ProjectTemplate::TypeScript => "ts",
        };

        if let Some(template_dir) = TEMPLATES_DIR.get_dir(template_name) {
            fs::create_dir_all(target_dir)?;
            Self::copy_dir_contents(template_dir, target_dir)?;
        }

        Ok(())
    }
    pub fn available_templates() -> Vec<&'static str> {
        vec!["TypeScript"]
    }
}
