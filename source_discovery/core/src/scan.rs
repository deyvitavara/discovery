#![allow(clippy::needless_return)]
//! Escaneo de código (secuencial, estable). Clasifica y cuenta LOC.
use crate::models::{Inventory, InventorySummary, FileRecord, FileCategory};
use anyhow::Result;
use chrono::Utc;
use globwalk::GlobWalkerBuilder;
use std::{path::{Path, PathBuf}};
use bytecount::count;

pub fn classify(path: &Path) -> FileCategory {
    let p = path.to_string_lossy().to_lowercase();
    if p.ends_with(".cs") { return FileCategory::Cs; }
    if p.ends_with(".vb") { return FileCategory::Vb; }
    if p.ends_with(".frm") { return FileCategory::Vb6Form; }
    if p.ends_with(".bas") { return FileCategory::Vb6Module; }
    if p.ends_with(".cls") { return FileCategory::Vb6Class; }
    if p.ends_with(".sln") { return FileCategory::Solution; }
    if p.ends_with(".csproj") { return FileCategory::ProjectCsproj; }
    if p.ends_with(".vbproj") { return FileCategory::ProjectVbproj; }
    if p.ends_with(".asmx") { return FileCategory::Asmx; }
    if p.ends_with(".xaml") { return FileCategory::Xaml; }
    if p.ends_with(".rdl") || p.ends_with(".rpt") { return FileCategory::Report; }
    if p.ends_with(".config") || p.ends_with(".json") || p.ends_with(".yaml") || p.ends_with(".yml") { return FileCategory::Config; }
    return FileCategory::Other;
}

fn count_loc(bytes: &[u8]) -> u64 {
    if bytes.is_empty() { return 0; }
    let mut loc = count(bytes, b'\n') as u64;
    if *bytes.last().unwrap() != b'\n' { loc += 1; }
    return loc;
}

pub fn scan_code(project: &str, base_path: &Path, includes: &[String]) -> Result<Inventory> {
    let mut files: Vec<FileRecord> = Vec::new();
    let mut by_ext: std::collections::BTreeMap<String, u64> = std::collections::BTreeMap::new();
    let mut total_size: u64 = 0;
    let mut total_loc: u64 = 0;

    let patterns: Vec<String> = if includes.is_empty() { vec!["**/*".into()] } else { includes.to_vec() };
    let walker = GlobWalkerBuilder::from_patterns(base_path, &patterns)
        .case_insensitive(true)
        .follow_links(true)
        .build()?;
    for entry in walker.into_iter().filter_map(Result::ok).filter(|e| e.file_type().is_file()) {
        let path = entry.path().to_path_buf();
        let rel = pathdiff::diff_paths(&path, base_path).unwrap_or(PathBuf::from(entry.file_name()));
        let ext = path.extension().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
        let mut rec = FileRecord {
            rel_path: rel.to_string_lossy().to_string(),
            ext: ext.clone(),
            bytes: 0, loc: 0, modified: None, category: classify(&path)
        };
        let data = std::fs::read(&path).unwrap_or_default();
        rec.bytes = data.len() as u64;
        rec.loc = count_loc(&data);
        files.push(rec);
    }

    files.sort_by(|a,b| a.rel_path.cmp(&b.rel_path));

    for rec in &files {
        total_size += rec.bytes;
        total_loc += rec.loc;
        *by_ext.entry(rec.ext.clone()).or_default() += 1;
    }

    let inv = Inventory {
        project: project.to_string(),
        scanned_at: Utc::now(),
        base_path: base_path.to_string_lossy().to_string(),
        files,
        summary: InventorySummary { total_files: by_ext.values().sum(), total_size_bytes: total_size, loc_total: total_loc, by_ext }
    };
    return Ok(inv);
}
