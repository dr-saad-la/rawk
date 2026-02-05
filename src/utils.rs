//! Utility functions
//!
//! Helper functions used across the codebase

use anyhow::Result;
use std::path::Path;

/// Check if a path exists and is a directory
pub fn is_valid_directory(path: &Path) -> bool {
    path.exists() && path.is_dir()
}

/// Create directory if it doesn't exist
pub fn ensure_directory(path: &Path) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}
