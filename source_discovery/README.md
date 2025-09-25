# Discovery Tool v0.8.1 (estable)

## Compilar
```bash
cargo build --release
```

## Usar
```bash
# 1) Escanear y ver inventario JSON
target/release/discovery-cli scan --path . --include "**/*"

# 2) Analizar (findings + sizing + RAID) y sacar JSON
target/release/discovery-cli analyze --path . --include "**/*" --out out.json

# 3) Exportar grafo (GraphML + JSON)
target/release/discovery-cli graph --path . --include "**/*" --graphml graph.graphml --json graph.json

# 4) Flujo completo
target/release/discovery-cli all --path . --include "**/*" --outdir out
```
