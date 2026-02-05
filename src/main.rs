//! Rawk CLI
//! Modern ML project templates that rawk

use clap::{Parser, Subcommand};
use colored::*;

#[derive(Parser)]
#[command(name = "rawk")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new project from a template
    New {
        /// Project name
        name: String,
        
        /// Template to use
        #[arg(short, long)]
        template: Option<String>,
    },
    
    /// List available templates
    List {
        /// Filter by category
        #[arg(short, long)]
        category: Option<String>,
    },
    
    /// Show template information
    Info {
        /// Template name
        template: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Some(Commands::New { name, template }) => {
            println!("{}", "Creating new project...".green().bold());
            rawk_lib::commands::new::execute(name, template.as_deref())
        }
        Some(Commands::List { category }) => {
            println!("{}", "Available templates:".green().bold());
            rawk_lib::commands::list::execute(category.as_deref())
        }
        Some(Commands::Info { template }) => {
            println!("{}", " Template information:".green().bold());
            rawk_lib::commands::info::execute(template)
        }
        None => {
            println!("{}", "Rawk - Modern ML project templates".green().bold());
            println!();
            println!("Usage: rawk <COMMAND>");
            println!();
            println!("Commands:");
            println!("  new       Create a new project");
            println!("  list      List available templates");
            println!("  info      Show template information");
            println!();
            println!("Run 'rawk --help' for more information");
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}
