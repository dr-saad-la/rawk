//! Info command implementation

use anyhow::Result;
use crate::Template;
use colored::*;

pub fn execute(template_name: &str) -> Result<()> {
    println!("{}\n", " Template Information".green().bold());
    
    match Template::load(template_name) {
        Ok(tmpl) => {
            // Header
            println!("{}: {}/{}", "Template".bold(), tmpl.category.cyan(), tmpl.name.cyan());
            println!("{}: {}", "Description".bold(), tmpl.config.template.description);
            println!("{}: {}", "Version".bold(), tmpl.config.template.version);
            
            // Tags
            if !tmpl.config.template.tags.is_empty() {
                println!("{}: {}", "Tags".bold(), tmpl.config.template.tags.join(", "));
            }
            
            // Requirements
            println!("\n{}", "Requirements:".bold());
            if let Some(python) = &tmpl.config.requirements.python {
                println!("  Python: {}", python);
            } else {
                println!("  Python: Any version");
            }
            
            if !tmpl.config.requirements.tools.is_empty() {
                println!("  Tools: {}", tmpl.config.requirements.tools.join(", "));
            }
            
            // Questions (if any)
            if !tmpl.config.questions.is_empty() {
                println!("\n{}", "Configuration Questions:".bold());
                for q in &tmpl.config.questions {
                    println!("  • {}", q.prompt);
                    if let Some(default) = &q.default {
                        println!("    Default: {}", default.dimmed());
                    }
                }
            }
            
            // Path
            println!("\n{}: {}", "Path".bold(), tmpl.path.display().to_string().dimmed());
            
            // Usage
            println!("\n{}", "Usage:".bold());
            println!("  rawk new my-project --template {}/{}", tmpl.category, tmpl.name);
            
            Ok(())
        }
        Err(e) => {
            eprintln!("{} {}", "✗ Failed to load template:".red().bold(), e);
            eprintln!("\n{}", "Available templates:".yellow());
            eprintln!("  Run: rawk list");
            Err(e)
        }
    }
}
