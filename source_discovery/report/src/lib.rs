#![allow(clippy::needless_return)]
//! Renderizado simple de reporte JSON consolidado.
use anyhow::Result;
use serde_json::{json, Value};
use chrono::Utc;

pub fn render_json(view: &serde_json::Value) -> Result<String> {
    // Inyecta metadatos básicos
    let mut v = view.clone();
    if let Some(obj) = v.as_object_mut() {
        obj.insert("generated_at".into(), json!(Utc::now()));
        obj.insert("tool_version".into(), json!("0.8.1"));
    }
    let s = serde_json::to_string_pretty(&v)?;
    return Ok(s);
}

/// Crea una vista estándar a partir de piezas sueltas.
pub fn compose_view(project: &str, inventory: &Value, findings: &Value, sizing: &Value, raid: &Value) -> Value {
    json!({
        "project": project,
        "inventory": inventory,
        "findings": findings,
        "sizing": sizing,
        "raid": raid
    })
}
