# Discovery Blue — Discovery técnico basado en código

Convierte repositorios reales en un **dashboard accionable** con:
- **Mapa de dependencias** (grafo) entre módulos y hubs (servicios, reportes, COM).
- **Hallazgos priorizados** con severidad y recomendación.
- **Sizing por módulo** (T-Shirt: XS–XL) para estimaciones iniciales.
- **RAID** (riesgos/assumptions/issues/dependencias) curado.

**Objetivo:** pasar de opiniones a **evidencia** para decidir *qué modernizar primero*, con **velocidad y claridad**.

---

## Componentes

- `discovery` (scanner): genera inventario y señales desde el código fuente.
- `discovery_reporter` (reporter): enriquece el grafo con *edges reales* y produce el **dashboard HTML**.

> El flujo estándar es: **código → report.json + graph.json → dashboard.html**

---

## Qué genera (artefactos)

salidas/
├─ report.json # Inventario (archivos, LOC, hallazgos, sizing, RAID, opcional DB)
├─ graph.json # Nodos base (archivos, hubs)
├─ graph_edges.json # Grafo enriquecido con relaciones reales
└─ dashboard.html # Reporte final navegable (KPI, hallazgos, estrategias, grafo)


---

## Requisitos

- Windows / Linux / macOS (CLI).
- Repositorio con código (VB6, .NET WinForms/Libs, ASP clásico, ASP.NET WebForms/MVC, config, reportes, etc.).

---

## Uso rápido (Windows CMD)

1) **Ejecuta el scanner** sobre tu proyecto:

```bat
discovery.exe ^
  --base "C:\Deyvi\poc\discoveryblue\demo\WinFormsAndVB6" ^
  --out  "C:\Deyvi\poc\salidas\VB6SalesApp"

Esto deja report.json y graph.json en la carpeta de salida.

Genera el dashboard (enriqueciendo el grafo con edges reales):
discovery_reporter.exe ^
  --report "C:\Deyvi\poc\salidas\VB6SalesApp\report.json" ^
  --graph  "C:\Deyvi\poc\salidas\VB6SalesApp\graph.json" ^
  --out    "C:\Deyvi\poc\salidas\VB6SalesApp\dashboard.html" ^
  --title  "Discovery Blue — VB6SalesApp"


Abre dashboard.html en tu navegador.

Tip: crea un run_report.cmd con las 2 líneas para regenerar el HTML con doble click.

Uso (CLI)
discovery_reporter.exe --report <ruta a report.json> --graph <ruta a graph.json> [opciones]

Opciones:
  --out     <archivo>   Salida HTML (por defecto: dashboard.html)
  --title   <texto>     Título visible en el reporte
  --max-bytes <N>       Límite de lectura por archivo para heurísticas (def: 3,000,000)


¿Qué detecta el reporter?

Servicios:

WCF/ASMX/SOAP: clases/atributos típicos, endpoints en .config, referencias .asmx/.svc/.ashx, uso de MSSOAP en VB6/ASP clásico.

Reportes: Crystal Reports (VB6/.NET), ReportViewer, RDL/RDLC/RPT.

COM/ActiveX: CreateObject/GetObject/Server.CreateObject, referencias VBP, etc.

Relaciones internas (archivo → archivo):

Llamadas por nombre (VB6/.NET).

ASP clásico: <!--#include ... -->.

WebForms: CodeBehind= y Register Src=.

MVC: Html.Partial/RenderPartial, Layout =, ActionLink/Action.

El grafo posiciona hubs (DB, WCF, ASMX, Reports, COM) y conecta cada archivo con sus dependencias y referencias.

Estructura mínima de report.json
{
  "project": "VB6SalesApp",
  "generated_at": "2025-09-10T10:30:00Z",
  "tool_version": "x.y.z",
  "inventory": {
    "base_path": "C:\\ruta\\al\\repo",
    "summary": { "total_files": 1234, "loc_total": 98765 },
    "files": [
      { "rel_path": "VB6/frmMain.frm", "ext": "frm" },
      { "rel_path": "WebApp/Views/Home/Index.cshtml", "ext": "cshtml" }
    ]
  },
  "findings": [
    { "id": "SECRETS_IN_CODE", "severity": "High", "file": "app.config", "evidence": "connectionString", "recommendation": "Mover a Vault/KeyVault" }
  ],
  "sizing": [
    { "module": "Ventas", "size": "M" }
  ],
  "raid": [
    { "risk": "Dependencia COM", "probability": "Media", "impact": "Alto", "mitigation": "Wrapper .NET y retiro", "owner": "Integraciones" }
  ],
  "db": {
    "summary": { "tables": 245, "procedures": 310, "triggers": 28 },
    "samples": { "tables": ["Orders", "Customers"], "procedures": ["usp_GetOrders"] }
  }
}


Estructura mínima de graph.json
{
  "nodes": [
    { "id": "n1", "label": "frmMain.frm" },
    { "id": "n2", "label": "frmVentas.frm" },
    { "id": "h_DB", "label": "DB" }
  ],
  "edges": [
    { "source": "n1", "target": "n2" }
  ]
}

El reporter añadirá edges detectados y guardará graph_edges.json.

¿Qué verás en el dashboard.html?

KPIs: proyecto, #archivos, LOC, #hallazgos.

Hallazgos y sizing: barras por severidad + dona XS–XL con tooltip explicativo.

Estrategias recomendadas: inferidas de la evidencia (p.ej., COM/WCF/DB/Reportes/VB6).

Top hallazgos: tabla con evidencia y recomendación concreta.

RAID: tabla glosario editable en JSON.

Grafo de dependencias: navegable, a todo el ancho, con hubs y flechas “from → to”.

Buenas prácticas (consultivo)

Evidencia, no opiniones: todo sale del código real y configuración.

Respeto por tu stack: no se etiqueta “legado”; se mapean convivencias y dependencias para priorizar correctamente.

Priorización clara: severidad + tamaño → orden de ejecución razonado.

Riesgo controlado: RAID visible tempranamente para planificar mitigaciones.

Solución de problemas

“No encuentra report.json/graph.json”
Revisa las rutas exactas (incluye el .json). Ejemplo válido:
--report "C:\...\salidas\App\report.json" (no la carpeta).

Grafo sin relaciones
Asegura que el repo incluya archivos de texto (frm/bas/cs/vb/aspx/cshtml/config, etc.).
Sube --max-bytes si tienes archivos grandes: --max-bytes 6000000.

Imagen del grafo pequeña
El HTML ya fija 100% de ancho y 720px de alto. Puedes editar #net{ height:... } si quieres más.

Extensión (fácil)

Añade nuevas reglas (regex) en el reporter para detectar otras integraciones.

Amplía sizing y RAID en report.json.

Exporta el dashboard a PDF con tu navegador (Imprimir → Guardar como PDF).

Licencia y crédito

© Discovery Blue. Hecho para equipos que necesitan claridad rápida para decidir y ejecutar con foco.


Si quieres, también te dejo un **README corto** para el subproyecto `discovery_reporter` (si lo mantienes separado):

```markdown
# discovery_reporter

Enriquece `graph.json` con **edges reales** leyendo el código y produce `dashboard.html`.

## Uso

```bat
discovery_reporter.exe ^
  --report "C:\proyecto\salidas\report.json" ^
  --graph  "C:\proyecto\salidas\graph.json" ^
  --out    "C:\proyecto\salidas\dashboard.html" ^
  --title  "Discovery Blue — Mi Sistema"


 
