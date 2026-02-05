#![allow(unused_imports)]
#![allow(dead_code)]
//! Template handling module
//!
//! Loading, parsing, and validating project templates

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// A project template
#[derive(Debug, Clone)]
pub struct Template {
    /// Template name (e.g., "simple-ml")
    pub name: String,

    /// Full path to template directory
    pub path: PathBuf,

    /// Parsed configuration from rawk.toml
    pub config: TemplateConfig,

    /// Category (e.g., "ml", "llm", "agents")
    pub category: String,
}

/// Template configuration (parsed from rawk.toml)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TemplateConfig {
    /// Template metadata
    pub template: TemplateMetadata,

    /// Requirements (Python version, tools)
    #[serde(default)]
    pub requirements: Requirements,

    /// Questions to ask user
    #[serde(default)]
    pub questions: Vec<Question>,
}

/// Template metadata
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TemplateMetadata {
    pub name: String,
    pub description: String,
    pub version: String,
    pub category: String,
    pub tags: Vec<String>,
}

/// Template requirements
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Requirements {
    /// Python version requirement (e.g., ">=3.10")
    pub python: Option<String>,

    /// Required tools (e.g., ["git", "docker"])
    #[serde(default)]
    pub tools: Vec<String>,
}

/// Question to ask user during project creation
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Question {
    /// Question ID (variable name)
    pub id: String,

    /// Question prompt text
    pub prompt: String,

    /// Default value
    pub default: Option<String>,

    /// Choices (for select questions)
    pub choices: Option<Vec<String>>,
}

// Error type (we'll use anyhow for now, can switch to thiserror later)
pub type TemplateError = anyhow::Error;

/// Get the templates directory path
///
/// Priority:
/// 1. Environment variable: RAWK_TEMPLATES_DIR
/// 2. Current directory: ./templates
/// 3. Binary directory: ../templates
fn get_templates_dir() -> Result<PathBuf> {
    // 1. Check environment variable
    if let Ok(dir) = std::env::var("RAWK_TEMPLATES_DIR") {
        let path = PathBuf::from(dir);
        if path.exists() && path.is_dir() {
            return Ok(path);
        }
    }

    // 2. Check current directory
    let current_dir = PathBuf::from("templates");
    if current_dir.exists() && current_dir.is_dir() {
        return Ok(current_dir);
    }

    // 3. Check relative to binary
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            let templates_path = parent.join("templates");
            if templates_path.exists() && templates_path.is_dir() {
                return Ok(templates_path);
            }
        }
    }

    // Fallback: assume ./templates
    Ok(PathBuf::from("templates"))
}

impl Template {
    /// Load template from a filesystem path
    ///
    /// # Arguments
    /// * `path` - Path to template directory
    ///
    /// # Example
    /// ```no_run
    /// use std::path::PathBuf;
    /// use rawk_lib::Template;
    ///
    /// let path = PathBuf::from("templates/ml/simple-ml");
    /// let template = Template::from_path(path).unwrap();
    /// ```
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        // Check path exists
        if !path.exists() {
            anyhow::bail!("Template path does not exist: {}", path.display());
        }

        if !path.is_dir() {
            anyhow::bail!("Template path is not a directory: {}", path.display());
        }

        // Read rawk.toml
        let config_path = path.join("rawk.toml");
        if !config_path.exists() {
            anyhow::bail!(
                "Template missing rawk.toml configuration: {}",
                path.display()
            );
        }

        let config_content = fs::read_to_string(&config_path)?;
        let config: TemplateConfig = toml::from_str(&config_content)?;

        // Extract name from path
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid template path"))?
            .to_string();

        // Extract category from parent directory
        let category = path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("other")
            .to_string();

        Ok(Self {
            name,
            path: path.to_path_buf(),
            config,
            category,
        })
    }
}

impl Template {
    /// Load template by name (e.g., "ml/simple-ml")
    ///
    /// # Arguments
    /// * `name` - Template name in format "category/name"
    ///
    /// # Example
    /// ```no_run
    /// use rawk_lib::Template;
    ///
    /// let template = Template::load("ml/simple-ml").unwrap();
    /// ```
    pub fn load(name: &str) -> Result<Self> {
        let templates_dir = get_templates_dir()?;

        // Support both "ml/simple-ml" and "simple-ml" formats
        let template_path = if name.contains('/') {
            // Hierarchical: "ml/simple-ml"
            templates_dir.join(name)
        } else {
            // Flat: "simple-ml" - search for it
            Self::find_template_by_name(&templates_dir, name)?
        };

        Self::from_path(template_path)
    }

    /// Find template by name (search all categories)
    fn find_template_by_name(base_dir: &Path, name: &str) -> Result<PathBuf> {
        // Walk through categories
        for entry in fs::read_dir(base_dir)? {
            let entry = entry?;
            let category_path = entry.path();

            if !category_path.is_dir() {
                continue;
            }

            // Check if template exists in this category
            let template_path = category_path.join(name);
            if template_path.exists() && template_path.is_dir() {
                return Ok(template_path);
            }
        }

        anyhow::bail!("Template not found: {}", name);
    }
}

impl Template {
    /// Validate template structure
    ///
    /// Checks:
    /// - rawk.toml exists and is valid
    /// - {{project_slug}} directory exists
    /// - README.md exists
    pub fn validate(&self) -> Result<()> {
        // 1. Check rawk.toml (already loaded, so valid)

        // 2. Check {{project_slug}} directory exists
        let project_dir = self.path.join("{{project_slug}}");
        if !project_dir.exists() {
            anyhow::bail!(
                "Template missing {{{{project_slug}}}} directory: {}",
                self.path.display()
            );
        }

        // 3. Check README.md exists (recommended)
        let readme_path = self.path.join("README.md");
        if !readme_path.exists() {
            // Warning only, not error
            eprintln!("⚠️  Template missing README.md: {}", self.path.display());
        }

        Ok(())
    }
}

impl Template {
    /// List all available templates
    ///
    /// # Example
    /// ```no_run
    /// use rawk_lib::Template;
    ///
    /// let templates = Template::list_all().unwrap();
    /// for template in templates {
    ///     println!("{}: {}", template.name, template.config.template.description);
    /// }
    /// ```
    pub fn list_all() -> Result<Vec<Self>> {
        let templates_dir = get_templates_dir()?;
        let mut templates = Vec::new();
        
        // Walk through categories
        for category_entry in fs::read_dir(&templates_dir)? {
            let category_entry = category_entry?;
            let category_path = category_entry.path();
            
            // Skip non-directories
            if !category_path.is_dir() {
                continue;
            }
            
            // Walk through templates in category
            for template_entry in fs::read_dir(&category_path)? {
                let template_entry = template_entry?;
                let template_path = template_entry.path();
                
                // Skip non-directories
                if !template_path.is_dir() {
                    continue;
                }
                
                // Try to load template
                match Self::from_path(&template_path) {
                    Ok(template) => templates.push(template),
                    Err(e) => {
                        eprintln!(
                            "⚠️  Skipping invalid template {}: {}",
                            template_path.display(),
                            e
                        );
                    }
                }
            }
        }
        
        Ok(templates)
    }
}

impl Template {
    /// Find templates matching a search query
    ///
    /// Searches in:
    /// - Template name
    /// - Description
    /// - Tags
    ///
    /// # Example
    /// ```no_run
    /// use rawk_lib::Template;
    ///
    /// let results = Template::find("pytorch").unwrap();
    /// ```
    pub fn find(query: &str) -> Result<Vec<Self>> {
        let all_templates = Self::list_all()?;
        let query_lower = query.to_lowercase();
        
        let results: Vec<_> = all_templates
            .into_iter()
            .filter(|t| {
                // Search in name
                t.name.to_lowercase().contains(&query_lower)
                    // Search in description
                    || t.config.template.description.to_lowercase().contains(&query_lower)
                    // Search in tags
                    || t.config.template.tags.iter().any(|tag| {
                        tag.to_lowercase().contains(&query_lower)
                    })
            })
            .collect();
        
        Ok(results)
    }
}
