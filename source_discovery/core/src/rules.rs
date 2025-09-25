#![allow(clippy::needless_return)]
//! Motor de reglas de riesgo, sizing (T‑shirt) y RAID; integra heurísticas, DSL y plugins.
use crate::models::{Inventory, FileCategory};
use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Finding {
    pub id: String,
    pub severity: String,
    pub file: String,
    pub line: Option<u64>,
    pub evidence: String,
    pub recommendation: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SizingItem {
    pub module: String,
    pub loc: u64,
    pub findings: u64,
    pub size: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RaidItem {
    pub risk: String,
    pub probability: String,
    pub impact: String,
    pub mitigation: String,
    pub owner: String,
}

pub fn analyze_rules(inv: &Inventory) -> Result<(Vec<Finding>, Vec<SizingItem>, Vec<RaidItem>)> {
    let base = Path::new(&inv.base_path);
    let mut findings: Vec<Finding> = Vec::new();

    for f in &inv.files {
        let path = base.join(&f.rel_path);
        let tl = std::fs::read_to_string(&path).unwrap_or_default().to_lowercase();

        if (matches!(f.category, FileCategory::Cs | FileCategory::Vb) && tl.contains("system.data.sqlclient")) &&
           (tl.contains("system.windows.forms") || tl.contains("initializecomponent(")) {
            findings.push(Finding {
                id: "UI_SQL_DIRECT".into(),
                severity: "high".into(),
                file: f.rel_path.clone(),
                line: None,
                evidence: "UI + SqlClient en el mismo archivo".into(),
                recommendation: "Extraer lógica/acceso a datos a servicios (ACL) y llamar vía API.".into(),
            });
        }

        if tl.contains("system.servicemodel") || f.rel_path.to_lowercase().ends_with(".asmx") {
            findings.push(Finding {
                id: "LEGACY_SOAP".into(),
                severity: "medium".into(),
                file: f.rel_path.clone(),
                line: None,
                evidence: "Uso de WCF/ASMX".into(),
                recommendation: "Publicar REST/gRPC equivalentes y migrar consumidores; mantener compatibilidad temporal.".into(),
            });
        }

        let re_secret = regex::Regex::new(r"(?i)(password=|pwd=|secret|apikey|connectionstring)").unwrap();
        if re_secret.is_match(&tl) {
            findings.push(Finding {
                id: "SECRETS_IN_CODE".into(),
                severity: "high".into(),
                file: f.rel_path.clone(),
                line: None,
                evidence: "Posible secreto en código/config".into(),
                recommendation: "Mover secretos a un gestor (Vault/KeyVault/Secret Manager) y rotación.".into(),
            });
        }

        if tl.contains("crystaldecisions") || f.rel_path.to_lowercase().ends_with(".rpt") {
            findings.push(Finding {
                id: "CRYSTAL_REPORTS".into(),
                severity: "medium".into(),
                file: f.rel_path.clone(),
                line: None,
                evidence: "Crystal Decisions detectado".into(),
                recommendation: "Servicio central de reportes/PDF, desacoplar del cliente.".into(),
            });
        }
        if f.rel_path.to_lowercase().ends_with(".rdl") || tl.contains("microsoft.reporting.winforms") {
            findings.push(Finding {
                id: "SSRS_REPORTS".into(),
                severity: "low".into(),
                file: f.rel_path.clone(),
                line: None,
                evidence: "SSRS/ReportViewer detectado".into(),
                recommendation: "Servicio central de reportes/PDF, desacoplar del cliente.".into(),
            });
        }
    }

    if let Ok(mut extra) = crate::rule_dsl::apply_yaml_rules(base, inv) {
        findings.append(&mut extra);
    }

    let mut sizing: Vec<SizingItem> = Vec::new();
    for f in &inv.files {
        let size = if f.loc < 200 { "XS" } else if f.loc < 800 { "S" } else if f.loc < 2000 { "M" } else if f.loc < 5000 { "L" } else { "XL" };
        let fc = findings.iter().filter(|x| x.file == f.rel_path).count() as u64;
        sizing.push(SizingItem { module: f.rel_path.clone(), loc: f.loc, findings: fc, size: size.into() });
    }

    let mut raid: Vec<RaidItem> = Vec::new();
    let has_ui_sql = findings.iter().any(|f| f.id == "UI_SQL_DIRECT");
    let has_com = inv.files.iter().any(|f| {
        let p = base.join(&f.rel_path);
        let t = std::fs::read_to_string(&p).unwrap_or_default().to_lowercase();
        t.contains("<comreference")
    });
    let has_wcf = findings.iter().any(|f| f.id == "LEGACY_SOAP");
    let has_secrets = findings.iter().any(|f| f.id == "SECRETS_IN_CODE");
    let has_reports = findings.iter().any(|f| f.id == "CRYSTAL_REPORTS" || f.id == "SSRS_REPORTS");

    if has_ui_sql {
        raid.push(RaidItem {
            risk: "Lógica/SQL en UI".into(), probability: "Alta".into(), impact: "Alto".into(),
            mitigation: "Extraer a servicios (ACL), pruebas golden-master, flags de activación.".into(), owner: "Arquitectura".into()
        });
    }
    if has_com {
        raid.push(RaidItem {
            risk: "Dependencia COM/OCX".into(), probability: "Media".into(), impact: "Alto".into(),
            mitigation: "Reemplazo por .NET nativo/wrappers; plan de retiro ActiveX.".into(), owner: "Integraciones".into()
        });
    }
    if has_wcf {
        raid.push(RaidItem {
            risk: "SOAP (WCF/ASMX)".into(), probability: "Media".into(), impact: "Medio".into(),
            mitigation: "REST/gRPC equivalentes; pruebas contractuales.".into(), owner: "APIs".into()
        });
    }
    if has_secrets {
        raid.push(RaidItem {
            risk: "Secretos en código/config".into(), probability: "Alta".into(), impact: "Alto".into(),
            mitigation: "Vault/KeyVault/Secret Manager, rotación, SAST.".into(), owner: "Seguridad".into()
        });
    }
    if has_reports {
        raid.push(RaidItem {
            risk: "Crystal/SSRS en cliente".into(), probability: "Media".into(), impact: "Medio".into(),
            mitigation: "Servicio central de PDF/Reportes; plan de reemplazo.".into(), owner: "Reporting".into()
        });
    }

    return Ok((findings, sizing, raid));
}
