//! List command implementation

use crate::Template;
use anyhow::Result;
use colored::*;
use std::collections::HashMap;

pub fn execute(category: Option<&str>) -> Result<()> {
    println!("{}", "Available Templates".green().bold());

    let templates = Template::list_all()?;

    // Filter by category if specified
    let filtered: Vec<_> = if let Some(cat) = category {
        templates
            .into_iter()
            .filter(|t| t.category == cat)
            .collect()
    } else {
        templates
    };

    if filtered.is_empty() {
        if let Some(cat) = category {
            println!("\nNo templates found in category '{}'", cat.yellow());
        } else {
            println!("\nNo templates found");
        }
        println!("\nMake sure templates are in the 'templates/' directory");
        return Ok(());
    }

    // Group by category
    let mut by_category: HashMap<String, Vec<&Template>> = HashMap::new();
    for template in &filtered {
        by_category
            .entry(template.category.clone())
            .or_default()
            .push(template);
    }

    // Display grouped by category
    let mut categories: Vec<_> = by_category.keys().collect();
    categories.sort();

    println!();
    for cat in categories {
        println!("{}", format!("{}:", cat.to_uppercase()).cyan().bold());

        let mut templates = by_category[cat].clone();
        templates.sort_by(|a, b| a.name.cmp(&b.name));

        for template in templates {
            println!(
                "  {} - {}",
                template.name.yellow(),
                template.config.template.description
            );

            // Show tags
            if !template.config.template.tags.is_empty() {
                let tags = template
                    .config
                    .template
                    .tags
                    .iter()
                    .map(|t| format!("#{}", t))
                    .collect::<Vec<_>>()
                    .join(" ");
                println!("    {}", tags.dimmed());
            }
        }
        println!();
    }

    println!("{}", "Usage:".bold());
    println!("  rawk new my-project --template <category>/<name>");
    println!("  rawk info <category>/<name>");

    Ok(())
}
