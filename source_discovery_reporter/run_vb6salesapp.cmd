@echo off
REM === Discovery Reporter (Edges + Dashboard) ===
REM Ajusta las rutas si cambias de proyecto
SET REPORT=C:\Deyvi\poc\discoveryblue_v2\salidas\VB6SalesApp\report
SET GRAPH=C:\Deyvi\poc\discoveryblue_v2\salidas\VB6SalesApp\graph
SET OUT=C:\Deyvi\poc\discoveryblue_v2\salidas\VB6SalesApp\dashboard.html

REM Build (si no tienes el binario aún)
REM cargo build --release

REM Ejecuta (usa el exe generado por cargo)
discovery_reporter\target\release\discovery_reporter.exe --report "%REPORT%" --graph "%GRAPH%" --out "%OUT%" --project VB6SalesApp --max-bytes 400000

echo.
echo Abriendo "%OUT%" ...
start "" "%OUT%"
