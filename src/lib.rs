//! # Rawk Library
//!
//! Core functionality for the Rawk template system.
//!
//! This library provides the building blocks for:
//! - Template loading and parsing
//! - Project generation
//! - Configuration management
//! - Rendering engine
//!
//! ## Example
//!
//! ```rust,no_run
//! use rawk_lib::{Template, render};
//!
//! // Load a template
//! let tmpl = Template::load("ml/simple-ml").unwrap();
//!
//! // Render template with context
//! let context = serde_json::json!({
//!     "project_name": "My ML Project",
//!     "author": "Dr. Saad"
//! });
//! let rendered = render::render_template("{{ project_name }}", &context).unwrap();
//! ```

// Module declarations
pub mod commands;
pub mod config;
pub mod render;
pub mod template;
pub mod utils;

// Re-export commonly used items
pub use config::Config;
pub use render::{RenderContext, Renderer};
pub use template::Template;

// Re-export command modules for easy access
pub use commands::{info, list, new};

// Library metadata
/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library name
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Library description
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

// Library-level error type
pub type Result<T> = anyhow::Result<T>;
