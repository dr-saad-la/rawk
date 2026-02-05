//! Info command implementation

use anyhow::Result;

pub fn execute(template: &str) -> Result<()> {
    println!("Template info: {}", template);
    Ok(())
}
