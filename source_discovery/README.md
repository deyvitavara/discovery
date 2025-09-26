Discovery Tool v0.8.1 (estable) + Reporter HTML
0) Requisitos

Rust estable (toolchain stable)

Windows Command Prompt (cmd.exe)

1) Compilar (build)
cargo build --release


Binarios generados:

target\release\discovery-cli.exe

target\release\discovery_reporter.exe

2) Uso rápido (Windows CMD)

Ejemplo usando el proyecto actual como raíz (.).
Ajusta --path y --include si quieres otro directorio/patrones.

2.1 Escanear → inventario (JSON)
target\release\discovery-cli.exe scan --path . --include "**/*"

2.2 Analizar → findings + sizing + RAID (JSON)
target\release\discovery-cli.exe analyze --path . --include "**/*" --out out\report.json


Crea la carpeta out si no existe:

mkdir out

2.3 Grafo → GraphML + JSON
target\release\discovery-cli.exe graph --path . --include "**/*" --graphml out\graph.graphml --json out\graph.json

2.4 Flujo completo (todo en out\)
target\release\discovery-cli.exe all --path . --include "**/*" --outdir out


Esto genera, como mínimo:

out\report.json

out\graph.json

(y, si está habilitado en tu build, out\graph.graphml)

3) Dashboard HTML (Reporter)

Convierte los JSON a un dashboard HTML listo para abrir en el navegador.

target\release\discovery_reporter.exe --report "out\report.json" --graph "out\graph.json" --out "out\dashboard.html" --title "Discovery Report"


Abre:

start "" "out\dashboard.html"

¿Qué muestra el dashboard?

KPIs: proyecto, #archivos, LOC, #hallazgos

Severidad (barras) y tamaños T-shirt (dona)

Estrategias recomendadas (priorizadas a partir de la evidencia)

Top hallazgos (qué/ dónde/ cómo mitigar)

RAID (riesgos/ supuestos/ issues/ dependencias)

Grafo de dependencias (nodos reales + hubs DB/WCF/ASMX/Reports/COM)

SQL Server (opcional): conteos y muestras si integraste el lector RO

4) Ejemplo con ruta absoluta (tu caso)

Supón código en:

C:\Deyvi\poc\discoveryblue_v2\demo\WinFormsAndVB6\VB6\VB6SalesApp

4.1 Escanear
cd /d C:\Deyvi\poc\discoveryblue_v2\demo\WinFormsAndVB6\VB6\VB6SalesApp
C:\ruta\a\tu\repo\target\release\discovery-cli.exe scan --path . --include "**/*"

4.2 Analizar + Grafo
mkdir C:\Deyvi\poc\discoveryblue_v2\salidas\VB6SalesApp

C:\ruta\a\tu\repo\target\release\discovery-cli.exe analyze --path . --include "**/*" --out C:\Deyvi\poc\discoveryblue_v2\salidas\VB6SalesApp\report.json

C:\ruta\a\tu\repo\target\release\discovery-cli.exe graph --path . --include "**/*" --graphml C:\Deyvi\poc\discoveryblue_v2\salidas\VB6SalesApp\graph.graphml --json C:\Deyvi\poc\discoveryblue_v2\salidas\VB6SalesApp\graph.json

4.3 Dashboard HTML
C:\ruta\a\tu\repo\target\release\discovery_reporter.exe ^
  --report "C:\Deyvi\poc\discoveryblue_v2\salidas\VB6SalesApp\report.json" ^
  --graph  "C:\Deyvi\poc\discoveryblue_v2\salidas\VB6SalesApp\graph.json" ^
  --out    "C:\Deyvi\poc\discoveryblue_v2\salidas\VB6SalesApp\dashboard.html" ^
  --title  "Discovery Report"
start "" "C:\Deyvi\poc\discoveryblue_v2\salidas\VB6SalesApp\dashboard.html"

5) Consejos y errores comunes

“El sistema no puede encontrar el archivo especificado (os error 2)”
Revisa:

Que report.json y graph.json existan en la ruta que pasas a --report y --graph.

Que las comillas estén bien (usa " en Windows CMD).

Que la carpeta out\ (o tu --outdir) exista.

Patrones --include
El glob "**/*" funciona en CMD tal cual (con comillas dobles).

Grafo vacío
Si edges = 0, habilita reglas/lecturas en el scanner (llamadas entre formularios, Declare VB6, includes ASP, CodeBehind/ASCX, MVC partials/layouts/actions). El Reporter renderiza lo que llegue en graph.json.
