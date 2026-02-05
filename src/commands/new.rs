//! New command implementation

use anyhow::Result;
use crate::{Template, RenderContext, Renderer};
use colored::*;
use std::path::PathBuf;
use serde_json::json;

pub fn execute(name: &str, template: Option<&str>) -> Result<()> {
    println!("{}", "Rawk - Creating Project".green().bold());
    println!();
    
    if let Some(tmpl_name) = template {
        // Load template
        let template = match Template::load(tmpl_name) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{} {}", "✗ Failed to load template:".red().bold(), e);
                eprintln!();
                eprintln!("{}", "Try:".yellow());
                eprintln!("  rawk list              # See all templates");
                eprintln!("  rawk search <query>    # Search templates");
                return Err(e);
            }
        };
        
        println!("{}: {}", "Project Name".bold(), name.cyan());
        println!("{}: {}/{}", "Template".bold(), template.category.cyan(), template.name.cyan());
        println!("{}: {}", "Description".bold(), template.config.template.description);
        println!();
        
        // Validate template
        if let Err(e) = template.validate() {
            eprintln!("{} {}", "Template validation warning:".yellow(), e);
            println!();
        }
        
        // Create render context
        let mut context = RenderContext::new(
            name.to_string(),
            name.to_lowercase().replace(' ', "-"),
            format!("A {} project", template.config.template.description),
            "Your Name".to_string(),
            "your.email@example.com".to_string(),
        );
        
        // Add template-specific variables with defaults
        let python_version = "3.12";
        context.add_variable("python_version".to_string(), json!(python_version));
        
        // Pre-compute python_target for Ruff (3.12 -> py312)
        let python_target = format!("py{}", python_version.replace('.', ""));
        context.add_variable("python_target".to_string(), json!(python_target));
        
        context.add_variable("package_manager".to_string(), json!("uv"));
        context.add_variable("model_type".to_string(), json!("classification"));
        context.add_variable("dataset_source".to_string(), json!("local"));
        context.add_variable("use_wandb".to_string(), json!(true));
        context.add_variable("use_hydra".to_string(), json!(true));
        context.add_variable("use_marimo".to_string(), json!(true));
        context.add_variable("use_docker".to_string(), json!(true));
        context.add_variable("use_precommit".to_string(), json!(true));
        context.add_variable("ci_cd".to_string(), json!("github"));
        context.add_variable("license".to_string(), json!("MIT"));
        context.add_variable("init_git".to_string(), json!(true));
        
        // Create output directory
        let output_dir = PathBuf::from(name);
        
        // Create renderer
        let renderer = Renderer::new(template, output_dir.clone(), context);
        
        // Render project
        println!("{}", "📦 Generating project files...".bold());
        match renderer.render() {
            Ok(_) => {
                println!();
                println!("{}", "✓ Project created successfully!".green().bold());
                println!();
                println!("{}", "Next steps:".bold());
                println!("  cd {}", name.cyan());
                println!("  ./setup.sh              # Run setup script");
                println!("  just --list             # See available commands");
                println!();
                println!("{}", "🎸 Rawk your project! ⚡".green());
                Ok(())
            }
            Err(e) => {
                eprintln!();
                eprintln!("{} {}", "✗ Failed to generate project:".red().bold(), e);
                eprintln!();
                
                // Better error message
                let error_msg = e.to_string();
                if error_msg.contains("undefined") || error_msg.contains("unknown method") {
                    eprintln!("{}", "Template variable error:".yellow());
                    eprintln!("  The template uses Jinja2 features not supported by minijinja");
                    eprintln!("  (method calls like .replace() are not available)");
                    eprintln!();
                    eprintln!("{}", "Debug info:".dimmed());
                    eprintln!("  {}", error_msg.dimmed());
                } else {
                    eprintln!("{}", "Common issues:".yellow());
                    eprintln!("  - Directory already exists (use different name)");
                    eprintln!("  - Permission denied (check directory permissions)");
                    eprintln!("  - Invalid template structure (run: rawk info {})", tmpl_name);
                }
                
                Err(e)
            }
        }
    } else {
        println!("{}: {}", "Project Name".bold(), name.cyan());
        println!("{}: {}", "Template".bold(), "Not specified".red());
        println!();
        eprintln!("{}", " No template specified!".yellow().bold());
        eprintln!();
        eprintln!("{}", "Usage:".bold());
        eprintln!("  rawk new {} --template <category>/<name>", name.cyan());
        eprintln!();
        eprintln!("{}", "Example:".bold());
        eprintln!("  rawk new {} --template ml/simple-ml", name.cyan());
        eprintln!();
        eprintln!("{}", "See available templates:".bold());
        eprintln!("  rawk list");
        
        anyhow::bail!("Template not specified");
    }
}
