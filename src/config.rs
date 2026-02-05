//! Configuration module
//!
//! Configuration management and parsing

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub template_dir: String,
    pub cache_dir: String,
}
