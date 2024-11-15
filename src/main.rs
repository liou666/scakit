mod cli;
mod error;
mod generator;
mod templates;
mod utils;

use cli::prompts;
use error::template_error::Result;
use generator::project_generator::ProjectGenerator;

fn main() -> Result<()> {
    let template = prompts::select_template()?;
    let project_name = prompts::input_project_name()?;

    println!(
        "🚀 Creating {} project '{}'...",
        template.as_str(),
        project_name
    );

    let generator = ProjectGenerator::new(template, project_name);
    generator.create_project()?;

    println!("🎉 Project setup completed!");
    Ok(())
}
