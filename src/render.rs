//! Rendering engine module
//!
//! Template rendering using minijinja

use anyhow::Result;
use minijinja::Environment;

pub fn render_template(template: &str, context: &serde_json::Value) -> Result<String> {
    let mut env = Environment::new();
    env.add_template("template", template)?;
    let tmpl = env.get_template("template")?;
    Ok(tmpl.render(context)?)
}
