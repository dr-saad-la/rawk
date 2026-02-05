//! Search command implementation

use anyhow::Result;
use crate::Template;

pub fn execute(query: &str) -> Result<()> {
    println!("Searching for '{}'...\n", query);
    
    let results = Template::find(query)?;
    
    if results.is_empty() {
        println!("No templates found matching '{}'", query);
        println!("\nTry:");
        println!("  rawk search pytorch");
        println!("  rawk search ml");
        println!("  rawk search agent");
        return Ok(());
    }
    
    println!("Found {} template(s):\n", results.len());
    
    for template in results {
        println!("  {}/{}", template.category, template.name);
        println!("    {}", template.config.template.description);
        
        if !template.config.template.tags.is_empty() {
            println!("    Tags: {}", template.config.template.tags.join(", "));
        }
        println!();
    }
    
    println!("Use 'rawk info <template>' for more information");
    
    Ok(())
}
