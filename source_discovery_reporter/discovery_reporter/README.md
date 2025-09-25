# discovery_reporter

Un solo comando para:
1) Leer `report.json` + `graph.json` generados por tu scanner.
2) Inferir **edges reales** (DB, WCF, ASMX, Reports, COM) mirando los fuentes.
3) Crear `graph_edges.json` y el **dashboard HTML** (con leyendas, estrategias y grafo).

## Compilación
```bat
cd discovery_reporter
cargo build --release
```

## Uso
```bat
target\release\discovery_reporter.exe ^
  --report "C:\ruta\report.json" ^
  --graph  "C:\ruta\graph.json" ^
  --out    "C:\ruta\dashboard.html" ^
  --title  "Discovery Report"
```
Si tus archivos se llaman `report` / `graph` sin `.json`, renómbralos a `report.json` y `graph.json`.
