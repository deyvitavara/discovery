//! DB: stub por defecto (sin SQL Server). Activa el feature `sqlserver` para implementar real.
use anyhow::{Result, anyhow};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct DbSummary {
    pub tables: u64,
    pub procedures: u64,
    pub triggers: u64,
}

/// Por defecto retorna error controlado (no habilitado).
pub async fn summarize_sqlserver_readonly(_conn: &str, _timeout_secs: u64) -> Result<DbSummary> {
    Err(anyhow!("Soporte SQL Server deshabilitado en esta build. Compila con el feature `sqlserver` e implementa la conexión real."))
}
