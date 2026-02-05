//! Template handling module
//!
//! Template loading, validation, and management

use anyhow::Result;
use std::path::PathBuf;

pub struct Template {
    pub name: String,
    pub path: PathBuf,
}

impl Template {
    pub fn load(name: &str) -> Result<Self> {
        Ok(Self {
            name: name.to_string(),
            path: PathBuf::from("templates").join(name),
        })
    }
}
