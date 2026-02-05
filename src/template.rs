//! Template handling module
//!
//! Loading, parsing, and validating project templates

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

// ============================================================================
// Types
// ============================================================================

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
    pub template: TemplateMetadata,
    #[serde(default)]
    pub requirements: Requirements,
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
    pub python: Option<String>,
    #[serde(default)]
    pub tools: Vec<String>,
}

/// Question to ask user during project creation
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Question {
    pub id: String,
    pub prompt: String,
    pub default: Option<String>,
    pub choices: Option<Vec<String>>,
}

// ============================================================================
// Template Implementation
// ============================================================================

impl Template {
    // ------------------------------------------------------------------------
    // Construction & Loading
    // ------------------------------------------------------------------------

    /// Load template from a filesystem path
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();

        // Validate path
        if !path.exists() {
            anyhow::bail!("Template path does not exist: {}", path.display());
        }
        if !path.is_dir() {
            anyhow::bail!("Template path is not a directory: {}", path.display());
        }

        // Load config
        let config = Self::load_config(path)?;

        // Extract metadata from path
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| anyhow::anyhow!("Invalid template path"))?
            .to_string();

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

    /// Load template by name (e.g., "ml/simple-ml")
    pub fn load(name: &str) -> Result<Self> {
        let templates_dir = get_templates_dir()?;

        let template_path = if name.contains('/') {
            templates_dir.join(name)
        } else {
            Self::find_template_by_name(&templates_dir, name)?
        };

        Self::from_path(template_path)
    }

    // ------------------------------------------------------------------------
    // Validation
    // ------------------------------------------------------------------------

    /// Validate template structure
    pub fn validate(&self) -> Result<()> {
        // Check {{project_slug}} directory
        let project_dir = self.path.join("{{project_slug}}");
        if !project_dir.exists() {
            anyhow::bail!(
                "Template missing {{{{project_slug}}}} directory: {}",
                self.path.display()
            );
        }

        // Check README.md (warning only)
        let readme_path = self.path.join("README.md");
        if !readme_path.exists() {
            eprintln!("⚠️  Template missing README.md: {}", self.path.display());
        }

        Ok(())
    }

    // ------------------------------------------------------------------------
    // Discovery & Search
    // ------------------------------------------------------------------------

    /// List all available templates
    pub fn list_all() -> Result<Vec<Self>> {
        let templates_dir = get_templates_dir()?;
        let mut templates = Vec::new();

        for category_entry in fs::read_dir(&templates_dir)? {
            let category_path = category_entry?.path();
            if !category_path.is_dir() {
                continue;
            }

            for template_entry in fs::read_dir(&category_path)? {
                let template_path = template_entry?.path();
                if !template_path.is_dir() {
                    continue;
                }

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

    /// Find templates matching a search query
    pub fn find(query: &str) -> Result<Vec<Self>> {
        let all_templates = Self::list_all()?;
        let query_lower = query.to_lowercase();

        let results = all_templates
            .into_iter()
            .filter(|t| t.matches_query(&query_lower))
            .collect();

        Ok(results)
    }

    // ------------------------------------------------------------------------
    // Private Helpers
    // ------------------------------------------------------------------------

    /// Load template config from path
    fn load_config(path: &Path) -> Result<TemplateConfig> {
        let config_path = path.join("rawk.toml");
        if !config_path.exists() {
            anyhow::bail!(
                "Template missing rawk.toml configuration: {}",
                path.display()
            );
        }

        let config_content = fs::read_to_string(&config_path)?;
        let config: TemplateConfig = toml::from_str(&config_content)?;
        Ok(config)
    }

    /// Find template by name (search all categories)
    fn find_template_by_name(base_dir: &Path, name: &str) -> Result<PathBuf> {
        for entry in fs::read_dir(base_dir)? {
            let category_path = entry?.path();
            if !category_path.is_dir() {
                continue;
            }

            let template_path = category_path.join(name);
            if template_path.exists() && template_path.is_dir() {
                return Ok(template_path);
            }
        }

        anyhow::bail!("Template not found: {}", name);
    }

    /// Check if template matches search query
    fn matches_query(&self, query_lower: &str) -> bool {
        self.name.to_lowercase().contains(query_lower)
            || self
                .config
                .template
                .description
                .to_lowercase()
                .contains(query_lower)
            || self
                .config
                .template
                .tags
                .iter()
                .any(|tag| tag.to_lowercase().contains(query_lower))
    }
}

// ============================================================================
// Free Functions
// ============================================================================

/// Get the templates directory path
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
    if let Ok(exe_path) = std::env::current_exe()
        && let Some(parent) = exe_path.parent()
    {
        let templates_path = parent.join("templates");
        if templates_path.exists() && templates_path.is_dir() {
            return Ok(templates_path);
        }
    }

    // Fallback
    Ok(PathBuf::from("templates"))
}
