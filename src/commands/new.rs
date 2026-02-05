//! New command implementation

use anyhow::Result;

pub fn execute(name: &str, template: Option<&str>) -> Result<()> {
    println!("Creating project: {}", name);
    if let Some(tmpl) = template {
        println!("Using template: {}", tmpl);
    }
    Ok(())
}
