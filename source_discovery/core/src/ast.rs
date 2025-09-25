#![allow(clippy::needless_return)]
//! Hints heurísticos para C# y VB.NET (sin tree-sitter).
use crate::models::{Inventory, FileCategory};
use anyhow::Result;
use std::{collections::BTreeMap, path::Path};

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct AstHint {
    pub is_winforms_ui: bool,
    pub has_initialize_component: bool,
    pub uses_sqlclient: bool,
    pub uses_crystal: bool,
    pub uses_reportviewer: bool,
}

fn analyze_csharp_text(src: &str) -> AstHint {
    let mut h = AstHint::default();
    let sl = src.to_lowercase();
    if sl.contains("system.windows.forms") { h.is_winforms_ui = true; }
    if sl.contains("initializecomponent(") { h.has_initialize_component = true; }
    if sl.contains("system.data.sqlclient") { h.uses_sqlclient = true; }
    if sl.contains("crystaldecisions") { h.uses_crystal = true; }
    if sl.contains("microsoft.reporting.winforms") || sl.contains("reportviewer") { h.uses_reportviewer = true; }
    return h;
}

fn analyze_vb_text(src: &str) -> AstHint {
    let mut h = AstHint::default();
    let sl = src.to_lowercase();
    if sl.contains("system.windows.forms") { h.is_winforms_ui = true; }
    if sl.contains("initializecomponent(") { h.has_initialize_component = true; }
    if sl.contains("system.data.sqlclient") { h.uses_sqlclient = true; }
    if sl.contains("crystaldecisions") { h.uses_crystal = true; }
    if sl.contains("microsoft.reporting.winforms") || sl.contains("reportviewer") { h.uses_reportviewer = true; }
    return h;
}

pub fn analyze_ast(inv: &Inventory) -> Result<BTreeMap<String, AstHint>> {
    let mut map = BTreeMap::new();
    let base = Path::new(&inv.base_path);
    for f in &inv.files {
        if let FileCategory::Cs = f.category {
            let p = base.join(&f.rel_path);
            if let Ok(text) = std::fs::read_to_string(p) {
                map.insert(f.rel_path.clone(), analyze_csharp_text(&text));
            }
        }
    }
    return Ok(map);
}

pub fn analyze_ast_vb(inv: &Inventory) -> Result<BTreeMap<String, AstHint>> {
    let mut map = BTreeMap::new();
    let base = Path::new(&inv.base_path);
    for f in &inv.files {
        if let FileCategory::Vb = f.category {
            let p = base.join(&f.rel_path);
            if let Ok(text) = std::fs::read_to_string(p) {
                map.insert(f.rel_path.clone(), analyze_vb_text(&text));
            }
        }
    }
    return Ok(map);
}
