#![allow(clippy::needless_return)]
//! Exporta grafo de dependencias (GraphML y JSON) a partir del inventario.
use crate::models::{Inventory, FileCategory};
use anyhow::Result;
use serde::Serialize;
use std::path::Path;

pub fn export_graphml(inv: &Inventory, out_path: &Path) -> Result<()> {
    let mut s = String::new();
    s.push_str("<?xml version='1.0' encoding='utf-8'?>");
    s.push_str("<graphml xmlns='http://graphml.graphdrawing.org/xmlns'>");
    s.push_str("<graph id='G' edgedefault='directed'>");

    for (i, f) in inv.files.iter().enumerate() {
        s.push_str(&format!("<node id='n{}'><data key='label'>{}</data></node>", i, xml_escape(&f.rel_path)));
    }

    for hub in ["DB", "WCF", "ASMX", "Reports", "COM"] {
        s.push_str(&format!("<node id='h_{0}'><data key='label'>{0}</data></node>", hub));
    }

    let base = Path::new(&inv.base_path);
    for (i, f) in inv.files.iter().enumerate() {
        let p = base.join(&f.rel_path);
        let tl = std::fs::read_to_string(&p).unwrap_or_default().to_lowercase();

        let mut add = |hub: &str| s.push_str(&format!("<edge source='n{}' target='h_{}'/>", i, hub));

        match f.category {
            FileCategory::Cs | FileCategory::Vb => {
                if tl.contains("system.data.sqlclient") { add("DB"); }
                if tl.contains("crystaldecisions") || f.rel_path.to_lowercase().ends_with(".rpt") { add("Reports"); }
                if tl.contains("microsoft.reporting.winforms") || f.rel_path.to_lowercase().ends_with(".rdl") { add("Reports"); }
                if tl.contains("servicecontract") || tl.contains("system.servicemodel") { add("WCF"); }
                if f.rel_path.to_lowercase().ends_with(".asmx") { add("ASMX"); }
                if tl.contains("<comreference") { add("COM"); }
            }
            FileCategory::Asmx => add("ASMX"),
            FileCategory::Report => add("Reports"),
            _ => {}
        }
    }

    s.push_str("</graph>");
    s.push_str("</graphml>");
    std::fs::write(out_path, s)?;
    return Ok(());
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace("'", "&apos;")
        .replace('"', "&quot;")
}


#[derive(Serialize)]
struct NodeJson { id: String, label: String }
#[derive(Serialize)]
struct EdgeJson { source: String, target: String }
#[derive(Serialize)]
struct GraphJson { nodes: Vec<NodeJson>, edges: Vec<EdgeJson> }

pub fn export_graph_json(inv: &Inventory, out_path: &Path) -> Result<()> {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    for (i, f) in inv.files.iter().enumerate() {
        nodes.push(NodeJson { id: format!("n{}", i), label: f.rel_path.clone() });
    }
    for hub in ["DB", "WCF", "ASMX", "Reports", "COM"] {
        nodes.push(NodeJson { id: format!("h_{}", hub), label: hub.to_string() });
    }

    let base = Path::new(&inv.base_path);
    for (i, f) in inv.files.iter().enumerate() {
        let p = base.join(&f.rel_path);
        let tl = std::fs::read_to_string(&p).unwrap_or_default().to_lowercase();
        let id = format!("n{}", i);
        let ensure = |hub: &str| EdgeJson { source: id.clone(), target: format!("h_{}", hub) };

        match f.category {
            FileCategory::Cs | FileCategory::Vb => {
                if tl.contains("system.data.sqlclient") { edges.push(ensure("DB")); }
                if tl.contains("crystaldecisions") || f.rel_path.to_lowercase().ends_with(".rpt") { edges.push(ensure("Reports")); }
                if tl.contains("microsoft.reporting.winforms") || f.rel_path.to_lowercase().ends_with(".rdl") { edges.push(ensure("Reports")); }
                if tl.contains("servicecontract") || tl.contains("system.servicemodel") { edges.push(ensure("WCF")); }
                if f.rel_path.to_lowercase().ends_with(".asmx") { edges.push(ensure("ASMX")); }
                if tl.contains("<comreference") { edges.push(ensure("COM")); }
            }
            FileCategory::Asmx => edges.push(ensure("ASMX")),
            FileCategory::Report => edges.push(ensure("Reports")),
            _ => {}
        }
    }
    let json = serde_json::to_string_pretty(&GraphJson { nodes, edges })?;
    std::fs::write(out_path, json)?;
    return Ok(());
}
