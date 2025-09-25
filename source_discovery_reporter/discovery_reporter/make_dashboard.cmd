@echo off
setlocal
set TOOL=%~dp0target\release\discovery_reporter.exe
set OUTDIR=C:\Deyvi\poc\discoveryblue_v2\salidas\VB6SalesApp
set R=%OUTDIR%\report.json
set G=%OUTDIR%\graph.json

if not exist "%R%" if exist "%OUTDIR%\report" ren "%OUTDIR%\report" "report.json"
if not exist "%G%" if exist "%OUTDIR%\graph" ren "%OUTDIR%\graph"  "graph.json"

"%TOOL%" --report "%R%" --graph "%G%" --out "%OUTDIR%\dashboard.html" --title "Discovery Report"
if errorlevel 1 exit /b 1
start "" "%OUTDIR%\dashboard.html"
endlocal
