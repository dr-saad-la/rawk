//! List command implementation

use anyhow::Result;

pub fn execute(category: Option<&str>) -> Result<()> {
    println!("Listing templates");
    if let Some(cat) = category {
        println!("Category: {}", cat);
    }
    Ok(())
}
