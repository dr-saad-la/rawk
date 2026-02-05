//! New command implementation

use anyhow::Result;
use crate::Template;
use colored::*;

pub fn execute(name: &str, template: Option<&str>) -> Result<()> {
    println!("{}", " Rawk - Creating Project".green().bold());
    println!();
    
    if let Some(tmpl_name) = template {
        // Load template
        match Template::load(tmpl_name) {
            Ok(template) => {
                println!("{}: {}", "Project Name".bold(), name.cyan());
                println!("{}: {}/{}", "Template".bold(), template.category.cyan(), template.name.cyan());
                println!("{}: {}", "Description".bold(), template.config.template.description);
                println!();
                
                // Validate template
                if let Err(e) = template.validate() {
                    eprintln!("{} {}", "⚠️  Template validation warning:".yellow(), e);
                    println!();
                }
                
                println!("{}", "✓ Template loaded successfully!".green());
                println!();
                println!("{}", "Next steps (implementation coming soon):".bold());
                println!("  1. Ask configuration questions");
                println!("  2. Render template files");
                println!("  3. Create project directory");
                println!("  4. Initialize git repository");
                println!();
                println!("{}", "For now, you can manually inspect the template at:".dimmed());
                println!("  {}", template.path.display().to_string().dimmed());
                
                Ok(())
            }
            Err(e) => {
                eprintln!("{} {}", "✗ Failed to load template:".red().bold(), e);
                eprintln!();
                eprintln!("{}", "Try:".yellow());
                eprintln!("  rawk list              # See all templates");
                eprintln!("  rawk search <query>    # Search templates");
                Err(e)
            }
        }
    } else {
        println!("{}: {}", "Project Name".bold(), name.cyan());
        println!("{}: {}", "Template".bold(), "Not specified".red());
        println!();
        eprintln!("{}", "⚠️  No template specified!".yellow().bold());
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
