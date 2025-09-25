#![allow(clippy::needless_return)]
//! Cargador de reglas YAML (DSL simple con regex y metadatos).
use serde::Deserialize;
use std::{fs, path::Path};
use crate::rules::Finding;
use anyhow::Result;

#[derive(Debug, Deserialize)]
struct RuleFile {
    id: String,
    severity: String,
    #[serde(default)]
    regexes: Vec<String>,
    #[serde(default)]
    evidence: Option<String>,
    #[serde(default)]
    recommendation: Option<String>,
}

pub fn apply_yaml_rules(base: &Path, inv: &crate::models::Inventory) -> Result<Vec<Finding>> {
    let dir = base.join("rules.d");
    let mut findings = Vec::new();
    if !dir.exists() { return Ok(findings); }
    for entry in fs::read_dir(&dir)? {
        let p = entry?.path();
        if p.extension().and_then(|s| s.to_str()).map(|e| e.eq_ignore_ascii_case("yml") || e.eq_ignore_ascii_case("yaml")).unwrap_or(false) {
            let text = fs::read_to_string(&p)?;
            let rf: RuleFile = serde_yaml::from_str(&text)?;
            let res: Vec<regex::Regex> = rf.regexes.iter().filter_map(|s| regex::Regex::new(s).ok()).collect();
            for f in &inv.files {
                let path = base.join(&f.rel_path);
                if let Ok(src) = fs::read_to_string(&path) {
                    for re in &res {
                        if re.is_match(&src) {
                            findings.push(Finding {
                                id: rf.id.clone(),
                                severity: rf.severity.clone(),
                                file: f.rel_path.clone(),
                                line: None,
                                evidence: rf.evidence.clone().unwrap_or_else(|| format!("Matched {}", re.as_str())),
                                recommendation: rf.recommendation.clone().unwrap_or_default(),
                            });
                            break;
                        }
                    }
                }
            }
        }
    }
    return Ok(findings);
}
