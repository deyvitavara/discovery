#![allow(clippy::needless_return)]
//! Ejecución de plugins WASM (stub por estabilidad: retorna vacío).
use anyhow::Result;
use serde_json::Value;
use std::path::Path;

pub fn run_wasm_plugins(_dir: &Path, _input_json: &Value) -> Result<Vec<Value>> {
    return Ok(Vec::new());
}
