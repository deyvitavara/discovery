#![allow(clippy::needless_return)]
//! Modelos serializables compartidos entre CLI/Report/Reglas.
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Inventory {
    pub project: String,
    pub scanned_at: DateTime<Utc>,
    pub base_path: String,
    pub files: Vec<FileRecord>,
    pub summary: InventorySummary,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InventorySummary {
    pub total_files: u64,
    pub total_size_bytes: u64,
    pub loc_total: u64,
    pub by_ext: std::collections::BTreeMap<String, u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileRecord {
    pub rel_path: String,
    pub ext: String,
    pub bytes: u64,
    pub loc: u64,
    pub modified: Option<String>, // texto legible
    pub category: FileCategory,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum FileCategory {
    Cs,
    Vb,
    Vb6Form,
    Vb6Module,
    Vb6Class,
    Solution,
    ProjectCsproj,
    ProjectVbproj,
    Config,
    Asmx,
    Xaml,
    Report,
    Other,
}
impl Default for FileCategory { fn default() -> Self { FileCategory::Other } }
