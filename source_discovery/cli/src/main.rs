use anyhow::Result;
use clap::{Parser, Subcommand};
use serde_json::json;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "discovery-cli")]
#[command(about = "Discovery Tool - scan / analyze / graph / report", long_about = None)]
struct Cli {
    /// Proyecto (nombre lógico) usado en reportes
    #[arg(short, long, default_value_t=String::from("project"))]
    project: String,

    /// Base path a escanear
    #[arg(short, long, default_value_t=String::from("."))]
    path: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Escanea el código y saca inventario JSON
    Scan {
        #[arg(long, default_value_t=String::from("**/*"))]
        include: String,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Analiza reglas (hallazgos, sizing, RAID) y saca JSON
    Analyze {
        #[arg(long, default_value_t=String::from("**/*"))]
        include: String,
        #[arg(long)]
        out: Option<PathBuf>,
    },
    /// Exporta graphml y json del grafo
    Graph {
        #[arg(long, default_value_t=String::from("graph.graphml"))]
        graphml: String,
        #[arg(long, default_value_t=String::from("graph.json"))]
        json: String,
        #[arg(long, default_value_t=String::from("**/*"))]
        include: String,
    },
    /// Flujo completo: scan + analyze + graph + reporte JSON
    All {
        #[arg(long, default_value_t=String::from("**/*"))]
        include: String,
        #[arg(long, default_value_t=String::from("out"))]
        outdir: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let base = std::path::Path::new(&cli.path);
    std::fs::create_dir_all(base)?;

    match &cli.command {
        Commands::Scan { include, out } => {
            let inv = core::scan::scan_code(&cli.project, base, &[include.clone()])?;
            let s = serde_json::to_string_pretty(&inv)?;
            if let Some(outp) = out {
                std::fs::write(outp, s)?;
            } else {
                println!("{}", s);
            }
        }
        Commands::Analyze { include, out } => {
            let inv = core::scan::scan_code(&cli.project, base, &[include.clone()])?;
            let (findings, sizing, raid) = core::rules::analyze_rules(&inv)?;
            let v = json!({
                "inventory": inv,
                "findings": findings,
                "sizing": sizing,
                "raid": raid
            });
            let s = report::render_json(&v)?;
            if let Some(outp) = out {
                std::fs::write(outp, s)?;
            } else {
                println!("{}", s);
            }
        }
        Commands::Graph { graphml, json: json_out, include } => {
            let inv = core::scan::scan_code(&cli.project, base, &[include.clone()])?;
            core::graph::export_graphml(&inv, &std::path::Path::new(graphml))?;
            core::graph::export_graph_json(&inv, &std::path::Path::new(json_out))?;
            println!("GraphML -> {}", graphml);
            println!("GraphJSON -> {}", json_out);
        }
        Commands::All { include, outdir } => {
            let outdir = std::path::Path::new(outdir);
            std::fs::create_dir_all(outdir)?;
            let inv = core::scan::scan_code(&cli.project, base, &[include.clone()])?;
            let (findings, sizing, raid) = core::rules::analyze_rules(&inv)?;
            core::graph::export_graphml(&inv, &outdir.join("graph.graphml"))?;
            core::graph::export_graph_json(&inv, &outdir.join("graph.json"))?;

            let view = report::compose_view(&cli.project,
                                            &serde_json::to_value(&inv)?,
                                            &serde_json::to_value(&findings)?,
                                            &serde_json::to_value(&sizing)?,
                                            &serde_json::to_value(&raid)?);
            let s = report::render_json(&view)?;
            std::fs::write(outdir.join("report.json"), s)?;
            println!("OK -> {}", outdir.display());
        }
    }

    Ok(())
}
