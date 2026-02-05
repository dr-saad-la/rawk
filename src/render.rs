//! Rendering engine module
//!
//! Template rendering using minijinja

use anyhow::{Context, Result};
use minijinja::Environment;
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::Template;

/// Render context containing all variables for template rendering
#[derive(Debug, Clone)]
pub struct RenderContext {
    /// Project name (e.g., "My ML Project")
    pub project_name: String,

    /// Project slug (e.g., "my-ml-project")
    pub project_slug: String,

    /// Project description
    pub project_description: String,

    /// Author name
    pub author_name: String,

    /// Author email
    pub author_email: String,

    /// Additional custom variables
    pub variables: HashMap<String, Value>,
}

impl RenderContext {
    /// Create a new render context
    pub fn new(
        project_name: String,
        project_slug: String,
        project_description: String,
        author_name: String,
        author_email: String,
    ) -> Self {
        Self {
            project_name,
            project_slug,
            project_description,
            author_name,
            author_email,
            variables: HashMap::new(),
        }
    }

    /// Add a custom variable
    pub fn add_variable(&mut self, key: String, value: Value) {
        self.variables.insert(key, value);
    }

    /// Convert to JSON value for minijinja
    pub fn to_json(&self) -> Value {
        let mut map = serde_json::Map::new();

        map.insert(
            "project_name".to_string(),
            Value::String(self.project_name.clone()),
        );
        map.insert(
            "project_slug".to_string(),
            Value::String(self.project_slug.clone()),
        );
        map.insert(
            "project_description".to_string(),
            Value::String(self.project_description.clone()),
        );
        map.insert(
            "author_name".to_string(),
            Value::String(self.author_name.clone()),
        );
        map.insert(
            "author_email".to_string(),
            Value::String(self.author_email.clone()),
        );

        // Add Python-safe module name (project-slug -> project_slug)
        let python_module = self.project_slug.replace('-', "_");
        map.insert("python_module".to_string(), Value::String(python_module));

        // Add custom variables
        for (key, value) in &self.variables {
            map.insert(key.clone(), value.clone());
        }

        Value::Object(map)
    }
}

/// Template renderer
pub struct Renderer {
    /// Template to render
    template: Template,

    /// Output directory
    output_dir: PathBuf,

    /// Render context
    context: RenderContext,

    /// Minijinja environment
    env: Environment<'static>,
}

impl Renderer {
    /// Create a new renderer
    pub fn new(template: Template, output_dir: PathBuf, context: RenderContext) -> Self {
        let env = Environment::new();

        Self {
            template,
            output_dir,
            context,
            env,
        }
    }

    /// Substitute variables in a path string
    ///
    /// Replaces {{project_slug}} and other simple variables
    fn substitute_path(&self, path: &str) -> String {
        let mut result = path.to_string();

        // Replace common variables
        result = result.replace("{{project_slug}}", &self.context.project_slug);
        result = result.replace("{{project_name}}", &self.context.project_name);

        // Handle Python module naming: project-slug -> project_slug
        let python_module = self.context.project_slug.replace('-', "_");
        result = result.replace("{{project_slug.replace('-','_')}}", &python_module);

        result
    }

    /// Check if a file should be skipped
    fn should_skip(&self, path: &Path) -> bool {
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        // Skip template configuration files
        if file_name == "rawk.toml" || file_name == "README.md" {
            // Only skip if it's at the template root
            if let Some(parent) = path.parent()
                && parent == self.template.path
            {
                return true;
            }
        }
        // Skip common unwanted files/directories
        matches!(
            file_name,
            ".git" | ".DS_Store" | "__pycache__" | "*.pyc" | ".gitkeep"
        )
    }

    /// Check if a file is a Jinja template
    fn is_template_file(&self, path: &Path) -> bool {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext == "jinja")
            .unwrap_or(false)
    }

    /// Render a single template file
    fn render_file(&self, template_path: &Path, output_path: &Path) -> Result<()> {
        // Read template content
        let template_content = fs::read_to_string(template_path)
            .with_context(|| format!("Failed to read template: {}", template_path.display()))?;

        // Render with minijinja
        let rendered = match self
            .env
            .render_str(&template_content, self.context.to_json())
        {
            Ok(r) => r,
            Err(e) => {
                // Better error message
                eprintln!("❌ Template rendering error:");
                eprintln!("   File: {}", template_path.display());
                eprintln!("   Error: {}", e);
                eprintln!();
                eprintln!("💡 Available variables:");
                let ctx = self.context.to_json();
                if let Value::Object(map) = ctx {
                    for key in map.keys() {
                        eprintln!("   - {}", key);
                    }
                }
                anyhow::bail!("Template rendering failed: {}", e);
            }
        };

        // Write to output
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        fs::write(output_path, rendered)
            .with_context(|| format!("Failed to write file: {}", output_path.display()))?;

        Ok(())
    }

    /// Copy a non-template file
    fn copy_file(&self, source: &Path, destination: &Path) -> Result<()> {
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        fs::copy(source, destination)
            .with_context(|| format!("Failed to copy file: {}", source.display()))?;

        Ok(())
    }

    /// Render the entire project
    pub fn render(&self) -> Result<()> {
        // Check if output directory already exists
        if self.output_dir.exists() {
            anyhow::bail!(
                "Output directory already exists: {}",
                self.output_dir.display()
            );
        }

        // Create output directory
        fs::create_dir_all(&self.output_dir).with_context(|| {
            format!(
                "Failed to create output directory: {}",
                self.output_dir.display()
            )
        })?;

        // Find the template source directory ({{project_slug}})
        let template_source = self.template.path.join("{{project_slug}}");

        if !template_source.exists() {
            anyhow::bail!(
                "Template source directory not found: {}",
                template_source.display()
            );
        }

        // Walk through template directory
        for entry in WalkDir::new(&template_source)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            // Skip the root directory itself
            if path == template_source {
                continue;
            }

            // Skip unwanted files
            if self.should_skip(path) {
                continue;
            }

            // Calculate relative path
            let relative_path = path
                .strip_prefix(&template_source)
                .context("Failed to calculate relative path")?;

            // Substitute variables in path
            let relative_str = relative_path.to_str().context("Invalid UTF-8 in path")?;
            let substituted_path = self.substitute_path(relative_str);

            // Calculate output path
            let output_path = self.output_dir.join(&substituted_path);

            // Handle directories
            if path.is_dir() {
                fs::create_dir_all(&output_path).with_context(|| {
                    format!("Failed to create directory: {}", output_path.display())
                })?;
                continue;
            }

            // Handle files
            if self.is_template_file(path) {
                // Remove .jinja extension for output
                let output_path = if let Some(stem) = output_path.file_stem() {
                    output_path.with_file_name(stem)
                } else {
                    output_path
                };

                self.render_file(path, &output_path)?;
            } else {
                self.copy_file(path, &output_path)?;
            }
        }

        Ok(())
    }
}
