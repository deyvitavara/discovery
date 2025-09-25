use anyhow::{Context, Result};
use clap::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::{HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};
use std::result::Result as StdResult;

/// Genera edges reales y un dashboard HTML desde report.json + graph.json.
#[derive(Parser)]
#[command(name="discovery_reporter", about="Discovery: edges + dashboard HTML en un paso")]
struct Args {
    /// Ruta a report.json (el que generaste con el scanner)
    #[arg(long)]
    report: String,
    /// Ruta a graph.json (el que generaste con el scanner)
    #[arg(long)]
    graph: String,
    /// Salida del HTML
    #[arg(long, default_value="dashboard.html")]
    out: String,
    /// Título del reporte
    #[arg(long, default_value="Discovery Report")]
    title: String,
    /// Límite de lectura por archivo (bytes)
    #[arg(long, default_value_t=3_000_000)]
    max_bytes: usize,
}

#[derive(Deserialize)]
struct Report {
    project: Option<String>,
    generated_at: Option<String>,
    tool_version: Option<String>,
    inventory: Inventory,
    findings: Option<Vec<Value>>,
    sizing: Option<Vec<Value>>,
    raid: Option<Vec<Value>>,
    db: Option<Value>,
}
#[derive(Deserialize)]
struct Inventory {
    base_path: String,
    summary: Option<Value>,
    files: Vec<FileRec>,
}
#[derive(Deserialize)]
struct FileRec {
    rel_path: String,
    ext: String,
}
#[derive(Deserialize, Serialize, Clone)]
struct Node { id: String, label: String }
#[derive(Deserialize, Serialize, Clone)]
struct Edge { source: String, target: String }
#[derive(Deserialize, Serialize, Clone)]
struct Graph { nodes: Vec<Node>, edges: Vec<Edge> }

fn is_textual_ext(ext: &str) -> bool {
    matches!(
        ext,
        // .NET / VB6
        "cs" | "vb" | "frm" | "bas" | "cls" | "ctl" | "vbp" |
        // Config y metadata
        "config" | "xml" | "json" | "txt" | "ini" | "props" | "targets" |
        // Servicios / Reportes / Handlers
        "asmx" | "svc" | "ashx" | "rdl" | "rdlc" | "rpt" | "rptc" |
        // ASP clásico y ASP.NET
        "asp" | "inc" | "asa" | "asax" |
        "aspx" | "ascx" | "master" |
        "cshtml" | "vbhtml"
    )
}

fn ensure_hubs(graph: &mut Graph) {
    let hubs = [("h_DB","DB"),("h_WCF","WCF"),("h_ASMX","ASMX"),("h_Reports","Reports"),("h_COM","COM")];
    for (id,label) in hubs {
        if !graph.nodes.iter().any(|n| n.id==id) {
            graph.nodes.push(Node{ id:id.into(), label:label.into() });
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    // 1) Cargar JSONs
    let report_str = fs::read_to_string(&args.report)
        .with_context(|| format!("Leyendo {}", &args.report))?;
    let graph_str  = fs::read_to_string(&args.graph)
        .with_context(|| format!("Leyendo {}", &args.graph))?;

    let report: Report = serde_json::from_str(&report_str).context("report.json inválido")?;
    let mut graph: Graph = serde_json::from_str(&graph_str).context("graph.json inválido")?;

    // 2) Mapear label->id (los nodos de archivos deben tener label==nombre de archivo)
    let mut label_to_id: HashMap<String, String> = HashMap::new();
    for n in &graph.nodes {
        label_to_id.insert(n.label.clone(), n.id.clone());
    }
    ensure_hubs(&mut graph);

    // Índice de archivos para relaciones internas (archivo→archivo) y helpers
    let mut stem_to_node: HashMap<String, String> = HashMap::new();
    let mut name_patterns: HashMap<String, Regex> = HashMap::new();
    for n in &graph.nodes {
        if n.id.starts_with("h_") { continue; }

        let stem = n.label.split('.').next().unwrap_or(&n.label).to_lowercase();
        if !stem.is_empty() {
            stem_to_node.insert(stem.clone(), n.id.clone());
            if stem.starts_with('_') {
                stem_to_node.insert(stem.trim_start_matches('_').to_string(), n.id.clone());
            }
        }
        let pat = format!(
            r"(?ix)\b(?:load|unload|show|hide|new)\s+{name}\b|\b{name}\s*\.\w+|\bcall\s+{name}\b|\b{name}\s*\(",
            name = regex::escape(&n.label.split('.').next().unwrap_or(&n.label).to_lowercase())
        );
        if let Ok(re) = Regex::new(&pat) {
            name_patterns.insert(n.label.split('.').next().unwrap_or(&n.label).to_lowercase(), re);
        }
    }

    // 3) Reglas de detección (regex)
    // WCF (código y .config)
    let re_wcf_code = Regex::new(r"(?i)(system\.servicemodel|ServiceContract|OperationContract|BasicHttpBinding|WsHttpBinding)").unwrap();
    let re_wcf_cfg  = Regex::new(r"(?is)<\s*system\.serviceModel\b|<\s*bindings\b|<\s*client\b\s*>\s*<\s*endpoint").unwrap();

    // ASMX/SVC/ASHX (código .NET, URL, VB6 via MSSOAP)
    let re_asmx_url  = Regex::new(r#"(?i)https?://[^\s"'<>]+\.(?:asmx|svc|ashx)(?:\?|/|")"#).unwrap();
    let re_asmx_net  = Regex::new(r"(?i)(SoapHttpClientProtocol|WebService|SoapDocumentMethod|System\.Web\.Services)").unwrap();
    let re_asmx_ext  = Regex::new(r"(?i)\.asmx\b|webservice|wsdl").unwrap();
    let re_vb6_msoap = Regex::new(r"(?i)MSSOAP\.SoapClient\d+|SoapClient\d+").unwrap();

    // Reports (VB6 Crystal OCX y .NET)
    let re_reports_vb6 = Regex::new(r"(?i)(CrystalReport\w*|CRViewer|CRAXDRT|OpenReport|ReportSource|\.rpt\b)").unwrap();
    let re_reports_net = Regex::new(r"(?i)(CrystalDecisions|Microsoft\.Reporting\.WinForms|ReportViewer|\.rdl\b|\.rdlc\b)").unwrap();

    // COM / ActiveX (incluye ASP clásico Server.CreateObject)
    let re_com   = Regex::new(r"(?i)(CreateObject\s*\(|GetObject\s*\(|Server\.CreateObject\s*\(|reference=|object=|progid)").unwrap();

    // SQL (DB) ADO .NET y OLEDB/ADO clásico
    let re_sql   = Regex::new(r"(?i)(system\.data\.sqlclient|microsoft\.data\.sqlclient|adodb\.|provider=|oledb|data\s+source=|Initial\s+Catalog=)").unwrap();

    // ASP clásico includes
    let re_asp_include = Regex::new(r#"(?is)<!--\s*#include\s+(?:file|virtual)\s*=\s*"([^"]+)""#).unwrap();

    // WebForms
    let re_codebehind   = Regex::new(r#"(?i)\bCodeBehind\s*=\s*"([^"]+\.(?:cs|vb))""#).unwrap();
    let re_register_ascx= Regex::new(r#"(?i)<%@\s*Register[^>]*\s+Src\s*=\s*"([^"]+\.ascx)""#).unwrap();

    // MVC Partials/Layouts/Actions
    let re_mvc_partial  = Regex::new(r#"(?i)@\s*html\.(?:renderpartial|partial)(?:async)?\s*\(\s*["']([^"']+)["']"#).unwrap();
    let re_mvc_layout   = Regex::new(r#"(?i)\bLayout\s*=\s*["']([^"']+)["']"#).unwrap();
    let re_mvc_action   = Regex::new(
        r#"(?i)@html\.(?:actionlink|renderaction|action)\s*\(\s*(?:"[^"]*"|'[^']*')?\s*,?\s*["']([^"']+)["']\s*,\s*["']([^"']+)["']"#
    ).unwrap();

    // 4) Generar edges leyendo textos reales
    let base = PathBuf::from(&report.inventory.base_path);
    let mut edges: Vec<Edge> = graph.edges.clone();
    let mut seen: HashSet<(String,String)> =
        edges.iter().map(|e| (e.source.clone(), e.target.clone())).collect();

    for f in &report.inventory.files {
        if !is_textual_ext(&f.ext.to_lowercase()) { continue; }

        let fname = Path::new(&f.rel_path)
            .file_name().unwrap_or_default()
            .to_string_lossy().to_string();
        let Some(node_id) = label_to_id.get(&fname) else { continue };

        let full = base.join(&f.rel_path);

        // METADATA
        let md = match fs::metadata(&full) {
            StdResult::Ok(v) => v,
            StdResult::Err(_) => continue,
        };
        if md.len() as usize > args.max_bytes { continue; }

        // TEXTO
        let txt = match fs::read_to_string(&full) {
            StdResult::Ok(v) => v,
            StdResult::Err(_) => continue,
        };
        let t = txt.to_lowercase();

        let mut hits: Vec<&str> = Vec::new();

        // DB
        if re_sql.is_match(&t) { hits.push("h_DB"); }

        // WCF (código o .config)
        if re_wcf_code.is_match(&t) || re_wcf_cfg.is_match(&t) { hits.push("h_WCF"); }

        // ASMX/SVC/ASHX (extensión, url, clases .NET o SOAP VB6)
        if re_asmx_ext.is_match(&t) || re_asmx_url.is_match(&t) || re_asmx_net.is_match(&t) || re_vb6_msoap.is_match(&t) {
            hits.push("h_ASMX");
        }

        // Reports (VB6 Crystal / .NET ReportViewer / RDL/RDLC/RPT)
        if re_reports_vb6.is_match(&t) || re_reports_net.is_match(&t) {
            hits.push("h_Reports");
        }

        // COM / ActiveX (incluye Server.CreateObject en ASP clásico)
        if re_com.is_match(&t) || f.ext.eq_ignore_ascii_case("vbp") {
            hits.push("h_COM");
        }

        hits.sort(); hits.dedup();

        // ---------- Relaciones específicas por tecnología ----------
        let ext_lc = f.ext.to_lowercase();

        // ASP clásico: includes
        if ext_lc=="asp" || ext_lc=="inc" || ext_lc=="asa" {
            for cap in re_asp_include.captures_iter(&txt) {
                let rel = cap[1].replace('\\', "/");
                if let Some(name) = Path::new(&rel).file_name() {
                    let inc = name.to_string_lossy().to_string();
                    if let Some(target_id) = label_to_id.get(&inc) {
                        let key = (node_id.clone(), target_id.clone());
                        if seen.insert(key.clone()) {
                            edges.push(Edge{ source: node_id.clone(), target: target_id.clone() });
                        }
                    }
                }
            }
        }

        // ASP.NET WebForms: CodeBehind + Register Src
        if ext_lc=="aspx" || ext_lc=="ascx" || ext_lc=="master" {
            if let Some(cap) = re_codebehind.captures(&txt) {
                let rel = cap[1].replace('\\', "/");
                if let Some(name) = Path::new(&rel).file_name() {
                    let cb = name.to_string_lossy().to_string();
                    if let Some(target_id) = label_to_id.get(&cb) {
                        let key = (node_id.clone(), target_id.clone());
                        if seen.insert(key.clone()) {
                            edges.push(Edge{ source: node_id.clone(), target: target_id.clone() });
                        }
                    }
                }
            }
            for cap in re_register_ascx.captures_iter(&txt) {
                let rel = cap[1].replace('\\', "/");
                if let Some(name) = Path::new(&rel).file_name() {
                    let ascx = name.to_string_lossy().to_string();
                    if let Some(target_id) = label_to_id.get(&ascx) {
                        let key = (node_id.clone(), target_id.clone());
                        if seen.insert(key.clone()) {
                            edges.push(Edge{ source: node_id.clone(), target: target_id.clone() });
                        }
                    }
                }
            }
        }

        // ASP.NET MVC: Partials, Layout, Controllers
        if ext_lc=="cshtml" || ext_lc=="vbhtml" {
            // View -> Partial
            for cap in re_mvc_partial.captures_iter(&t) {
                let mut stem = cap[1].trim().trim_matches('"').trim_matches('\'').to_string();
                if let Some(name) = Path::new(&stem).file_name() {
                    stem = name.to_string_lossy().to_string();
                }
                let clean = stem.trim_start_matches('_').to_lowercase();
                if let Some(target_id) = stem_to_node.get(&clean) {
                    let key = (node_id.clone(), target_id.clone());
                    if seen.insert(key.clone()) {
                        edges.push(Edge{ source: node_id.clone(), target: target_id.clone() });
                    }
                }
            }
            // View -> Layout
            if let Some(cap) = re_mvc_layout.captures(&t) {
                let mut lay = cap[1].trim().trim_matches('"').trim_matches('\'').to_string();
                if let Some(name) = Path::new(&lay).file_name() {
                    lay = name.to_string_lossy().to_string();
                } else if !lay.contains('.') {
                    let cand1 = format!("{}.cshtml", lay);
                    let cand2 = format!("{}.vbhtml", lay);
                    if let Some(target_id) = label_to_id.get(&cand1).or_else(|| label_to_id.get(&cand2)) {
                        let key = (node_id.clone(), target_id.clone());
                        if seen.insert(key.clone()) {
                            edges.push(Edge{ source: node_id.clone(), target: target_id.clone() });
                        }
                    }
                }
                if let Some(target_id) = label_to_id.get(&lay) {
                    let key = (node_id.clone(), target_id.clone());
                    if seen.insert(key.clone()) {
                        edges.push(Edge{ source: node_id.clone(), target: target_id.clone() });
                    }
                }
            }
            // View -> Controller (Action, Controller)
            for cap in re_mvc_action.captures_iter(&t) {
                let controller_stem = format!("{}controller", cap[2].to_lowercase());
                if let Some(target_id) = stem_to_node.get(&controller_stem) {
                    let key = (node_id.clone(), target_id.clone());
                    if seen.insert(key.clone()) {
                        edges.push(Edge{ source: node_id.clone(), target: target_id.clone() });
                    }
                }
            }
        }

        // Relaciones internas genéricas archivo→archivo
        let self_stem = fname.split('.').next().unwrap_or(&fname).to_lowercase();
        for (stem, re_pat) in &name_patterns {
            if stem == &self_stem { continue; }
            if re_pat.is_match(&t) {
                if let Some(target_id) = stem_to_node.get(stem) {
                    let key = (node_id.clone(), target_id.clone());
                    if seen.insert(key.clone()) {
                        edges.push(Edge{ source: node_id.clone(), target: target_id.clone() });
                    }
                }
            }
        }

        // Hubs
        for hub in hits {
            let key = (node_id.clone(), hub.to_string());
            if seen.insert(key.clone()) {
                edges.push(Edge{ source: node_id.clone(), target: hub.into() });
            }
        }
    }

    graph.edges = edges;
    // Guardar graph_edges.json junto al original
    let graph_edges_path = Path::new(&args.graph).with_file_name("graph_edges.json");
    fs::write(&graph_edges_path, serde_json::to_string_pretty(&graph)?)?;

    // 5) Construir el HTML
    let view = json!({
        "project": report.project.unwrap_or_else(|| "project".into()),
        "generated_at": report.generated_at,
        "tool_version": report.tool_version,
        "inventory": { "summary": report.inventory.summary.unwrap_or(json!({})) },
        "findings": report.findings.unwrap_or_default(),
        "sizing": report.sizing.unwrap_or_default(),
        "raid": report.raid.unwrap_or_default(),
        "db": report.db.unwrap_or(json!(null))
    });
    let html = DASH
        .replace("%TITLE%", &args.title)
        .replace("%VIEW%", &serde_json::to_string(&view)?)
        .replace("%GRAPH%", &serde_json::to_string(&graph)?);
    fs::write(&args.out, html).with_context(|| format!("Escribiendo {}", &args.out))?;
    println!("OK -> {}", &args.out);
    Ok(())
}

// HTML autosuficiente (Chart.js + vis-network)
// NOTA: usamos r#### para que las cadenas tipo "#14532d" NO rompan el raw string.
const DASH: &str = r####"<!doctype html><html lang="es"><head>
<meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1">
<title>%TITLE%</title>
<link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;600;700&display=swap" rel="stylesheet">
<style>
:root{--bg:#0f172a;--card:#0b1220;--muted:#9aa6b2;--ok:#22c55e;--warn:#f59e0b;--bad:#ef4444;--ink:#e5e7eb}
*{box-sizing:border-box}body{margin:0;background:var(--bg);color:var(--ink);font-family:Inter,system-ui}
header{padding:20px 24px;border-bottom:1px solid #1f2937;display:flex;justify-content:space-between}
.wrap{padding:20px;display:grid;gap:16px}
.card{background:#0b1220;border:1px solid #1f2937;border-radius:16px;padding:16px}
.kpis{display:grid;gap:12px;grid-template-columns:repeat(4,1fr)}
.kpi{background:#09101a;border:1px solid #1f2937;border-radius:12px;padding:12px}
.kpi h3{margin:0 0 6px 0;font-size:12px;color:var(--muted)}.kpi .v{font-size:22px;font-weight:700}
.grid{display:grid;gap:16px}@media(min-width:1100px){.grid{grid-template-columns:1.3fr 1fr}}
.legend{display:flex;gap:12px;flex-wrap:wrap;margin:8px 0}.help{font-size:12px;color:#cbd5e1}
.dot{width:10px;height:10px;border-radius:999px;display:inline-block}
.dot.high{background:var(--bad)}.dot.medium{background:var(--warn)}.dot.low{background:var(--ok)}
.dot.hub-DB{background:#14532d}.dot.hub-WCF{background:#713f12}.dot.hub-ASMX{background:#5b2106}.dot.hub-Reports{background:#0b3b76}.dot.hub-COM{background:#3f1d38}
table{width:100%;border-collapse:collapse;font-size:13px}th,td{border-bottom:1px solid #1f2937;padding:8px 6px;text-align:left}th{color:#aab4c4}
.sev-high{color:var(--bad);font-weight:700}.sev-medium{color:var(--warn);font-weight:700}.sev-low{color:var(--ok);font-weight:700}
.badge{font-size:10px;padding:3px 6px;border:1px solid #1f2937;border-radius:999px;background:#09101a;color:#cbd5e1}
#net{width:100%;height:720px;border:1px solid #1f2937;border-radius:12px;background:#0b1220}
.note{background:#0a1524;border:1px dashed #1e293b;border-radius:10px;padding:10px;margin-top:10px;color:#cbd5e1}
.card-graph{grid-column:1 / -1}
.grid-raid-graph{grid-template-columns:1fr}
</style>
</head><body>
<header><h1>%TITLE%</h1><div class="badge" id="meta"></div></header>
<div class="wrap">

<div class="kpis">
  <div class="kpi"><h3>Proyecto</h3><div class="v" id="k_project"></div></div>
  <div class="kpi"><h3>Archivos</h3><div class="v" id="k_files"></div></div>
  <div class="kpi"><h3>LOC totales</h3><div class="v" id="k_loc"></div></div>
  <div class="kpi"><h3>Findings</h3><div class="v" id="k_findings"></div></div>
</div>

<div class="grid">
  <div class="card">
    <h2>Hallazgos y sizing <span class="badge">¿Qué estoy viendo?</span></h2>
    <div class="help">Los <b>hallazgos</b> son reglas detectadas (ej. “secretos en código”, “uso de COM”). Severidad:
      <span class="sev-high">Alta</span> (acción inmediata),
      <span class="sev-medium">Media</span> (planificar),
      <span class="sev-low">Baja</span> (oportunidad). El gráfico de dona muestra <b>tamaño por módulo</b> (T-shirt sizing).</div>
    <div class="legend">
      <div><span class="dot high"></span> Alta</div>
      <div><span class="dot medium"></span> Media</div>
      <div><span class="dot low"></span> Baja</div>
      <div class="badge">Sizing: XS≤100, S 101-500, M 501-2k, L 2k-5k, XL >5k LOC</div>
    </div>
    <canvas id="chartSeverity" height="150" style="margin-top:10px"></canvas>
    <canvas id="chartSizes" height="160" style="margin-top:10px"></canvas>
    <div class="note" id="noteSeverity" style="display:none">Sin hallazgos detectados por las reglas actuales.</div>
  </div>
  <div class="card">
    <h2>Estrategias recomendadas <span class="badge">priorizadas</span></h2>
    <div id="estrategias" class="help">Se infieren de la evidencia (VB6, COM, WCF/ASMX, DB, secretos, etc.).</div>
  </div>
</div>

<div class="card">
  <h2>Top hallazgos <span class="badge" id="bfnd"></span></h2>
  <div class="help">Qué, dónde y cómo mitigarlo.</div>
  <table><thead><tr><th>ID</th><th>Severidad</th><th>Archivo</th><th>Evidencia</th><th>Recomendación</th></tr></thead><tbody id="tbodyFind"></tbody></table>
  <div class="note" id="emptyFind" style="display:none">No se detectaron hallazgos con las reglas actuales.</div>
</div>

<div class="grid grid-raid-graph">
  <div class="card">
    <h2>RAID <span class="badge">glosario</span></h2>
    <div class="help">RAID = <b>Riesgos</b>, <b>Supuestos</b>, <b>Issues</b>, <b>Dependencias</b>. Ayuda a planificar y mitigar.</div>
    <table><thead><tr><th>Riesgo</th><th>Prob.</th><th>Impacto</th><th>Mitigación</th><th>Owner</th></tr></thead><tbody id="tbodyRaid"></tbody></table>
    <div class="note" id="emptyRaid" style="display:none">Sin entradas RAID.</div>
  </div>
  <div class="card card-graph">
    <h2>Grafo de dependencias <span class="badge" id="bedges"></span></h2>
    <div class="legend">
      <div><span class="dot hub-DB"></span> DB</div>
      <div><span class="dot hub-WCF"></span> WCF</div>
      <div><span class="dot hub-ASMX"></span> ASMX</div>
      <div><span class="dot hub-Reports"></span> Reports</div>
      <div><span class="dot hub-COM"></span> COM/ActiveX</div>
    </div>
    <div id="net"></div>
    <div class="note" id="emptyGraph" style="display:none">No hay relaciones detectadas (edges). Al habilitar reglas de llamadas el grafo se poblará.</div>
  </div>
</div>

<div class="card" id="dbCard" style="display:none">
  <h2>SQL Server (read-only) <span class="badge">conteos</span></h2>
  <div id="dbSummary"></div>
</div>

</div>
<script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
<script src="https://unpkg.com/vis-network/standalone/umd/vis-network.min.js"></script>
<script>
window.REPORT=%VIEW%;window.GRAPH=%GRAPH%;
const R=window.REPORT||{},G=window.GRAPH||{nodes:[],edges:[]};
const sum=(R.inventory&&R.inventory.summary)||{},find=(R.findings||[]),sizing=(R.sizing||[]),raid=(R.raid||[]),DB=R.db;
document.getElementById('meta').textContent=(R.generated_at?("Generado: "+R.generated_at):"")+(R.tool_version?(" · v"+R.tool_version):"");
document.getElementById('k_project').textContent=R.project||"project";
document.getElementById('k_files').textContent=(sum.total_files||0);
document.getElementById('k_loc').textContent=(sum.loc_total||0);
document.getElementById('k_findings').textContent=(find.length||0);

// Severidad
const sev={high:0,medium:0,low:0}; find.forEach(f=>{const s=(f.severity||"").toLowerCase(); if(sev[s]!=null) sev[s]++;});
new Chart(document.getElementById('chartSeverity'),{type:'bar',
 data:{labels:['Alta','Media','Baja'],datasets:[{label:'Cantidad',data:[sev.high,sev.medium,sev.low]}]},
 options:{plugins:{legend:{display:false},tooltip:{callbacks:{label:(c)=>{const t=['Alta','Media','Baja'][c.dataIndex];const e=t==='Alta'?'Riesgo crítico.':t==='Media'?'Planificar mitigación.':'Deuda menor.';return `${c.formattedValue} · ${e}`;}}}},scales:{y:{beginAtZero:true,ticks:{precision:0}}}}
});
if(!find.length){document.getElementById('noteSeverity').style.display='block';}

// Sizing dona
const order=['XS','S','M','L','XL']; const count={XS:0,S:0,M:0,L:0,XL:0};
sizing.forEach(s=>{if(count[s.size]!=null)count[s.size]++;});
new Chart(document.getElementById('chartSizes'),{type:'doughnut',
 data:{labels:order,datasets:[{data:order.map(k=>count[k])}]},
 options:{plugins:{tooltip:{callbacks:{label:(c)=>{const map={XS:'≤100 LOC',S:'101-500',M:'501-2k',L:'2k-5k',XL:'>5k'};return `${c.label}: ${c.formattedValue} módulos (${map[c.label]})`;}}}}}
});

// Estrategias
const estrategias=[]; const ids=new Set(find.map(f=>String(f.id||'').toUpperCase())); const labs=new Set((G.nodes||[]).map(n=>n.id));
const lower=(R.inventory&&R.inventory.files||[]).map(f=>(f.rel_path||'').toLowerCase());
const hasVB6=lower.some(p=>p.endsWith('.frm')||p.endsWith('.bas')||p.endsWith('.vbp'));
function add(p,t,why,doit){estrategias.push({p,t,why,doit});}
if(ids.has('SECRETS_IN_CODE')) add(1,'Secretos en código','Posibles claves en código/config.','Mover a Vault/KeyVault/Secret Manager y rotación; SAST continuo.');
if(labs.has('h_COM')) add(2,'Dependencias COM/ActiveX','Componentes COM/OCX detectados.','Retiro o wrappers .NET; feature flags para minimizar impacto.');
if(labs.has('h_WCF')||labs.has('h_ASMX')) add(3,'Servicios SOAP legados','WCF/ASMX presentes.','Publicar REST/gRPC equivalentes y migrar consumidores.');
if(labs.has('h_DB')) add(4,'Acoplamiento a BD','Acceso SQL directo.','Encapsular en servicios/repositorios; API gateway.');
if(labs.has('h_Reports')) add(5,'Reportería embebida','Crystal/Reports en cliente.','Centralizar (servicio PDF) y estandarizar plantillas.');
if(hasVB6) add(6,'Modernización UI VB6','Formularios VB6 detectados.','Migración progresiva a .NET/WPF o Web para pantallas críticas.');
estrategias.sort((a,b)=>a.p-b.p);
document.getElementById('estrategias').innerHTML = estrategias.length
 ? '<ol>'+estrategias.map(e=>`<li><b>${e.t}</b><br><span class="help">Por qué: ${e.why}</span><br><span class="help">Siguiente paso: ${e.doit}</span></li>`).join('')+'</ol>'
 : '<div class="note">No se detectaron señales suficientes para proponer estrategias.</div>';

// Top hallazgos
const tb=document.getElementById('tbodyFind');
find.slice(0,200).forEach(f=>{
  const s=(f.severity||'').toLowerCase(); const css=s==='high'?'sev-high':s==='medium'?'sev-medium':'sev-low';
  tb.insertAdjacentHTML('beforeend',`<tr><td>${f.id||''}</td><td class="${css}">${f.severity||''}</td><td>${f.file||''}</td><td>${f.evidence||''}</td><td>${f.recommendation||''}</td></tr>`);
});
if(!find.length){document.getElementById('emptyFind').style.display='block';}
document.getElementById('bfnd').textContent=`${find.length||0} encontrado(s)`;

// RAID
const tr=document.getElementById('tbodyRaid'); (raid||[]).forEach(r=>{
  tr.insertAdjacentHTML('beforeend',`<tr><td>${r.risk||''}</td><td>${r.probability||''}</td><td>${r.impact||''}</td><td>${r.mitigation||''}</td><td>${r.owner||''}</td></tr>`);
});
if(!(raid&&raid.length)){document.getElementById('emptyRaid').style.display='block';}

// Grafo
const hubsColors={DB:"#14532d",WCF:"#713f12",ASMX:"#5b2106",Reports:"#0b3b76",COM:"#3f1d38"};
const nodes=[],edges=[]; (G.nodes||[]).forEach(n=>{
  const isHub=(n.id||'').startsWith('h_'); const hub=isHub?n.id.slice(2):null;
  nodes.push({id:n.id,label:n.label,shape:isHub?'box':'ellipse',color:isHub?{background:hubsColors[hub]||'#334155',border:'#1f2937'}:undefined,font:{color:'#e5e7eb',size:12}});
});
(G.edges||[]).forEach(e=>edges.push({from:e.source,to:e.target,arrows:'to'}));
document.getElementById('bedges').textContent=`${edges.length||0} relación(es)`;
if(!edges.length){document.getElementById('emptyGraph').style.display='block';}
const data={nodes:new vis.DataSet(nodes),edges:new vis.DataSet(edges)};
const hasEdges=edges.length>0;
new vis.Network(document.getElementById('net'),data,{
  layout: hasEdges?{hierarchical:{direction:'LR',nodeSpacing:120,levelSeparation:160}}:{improvedLayout:true},
  physics: hasEdges?false:{stabilization:true},
  interaction:{hover:true,navigationButtons:true,keyboard:true}
});

if(DB){
  document.getElementById("dbCard").style.display="block";
  const s=DB.summary||{}, samp=DB.samples||{};
  document.getElementById("dbSummary").innerHTML=
    `<p><b>Tablas:</b> ${s.tables??"-"} &nbsp; <b>Procedimientos:</b> ${s.procedures??"-"} &nbsp; <b>Triggers:</b> ${s.triggers??"-"}</p>
     <p class="help"><b>Top tablas</b>: <code>${(samp.tables||[]).slice(0,10).join(", ")}</code></p>
     <p class="help"><b>Top SPs</b>: <code>${(samp.procedures||[]).slice(0,10).join(", ")}</code></p>`;
}
</script></body></html>"####;
