# WinForms + VB6 Ejemplo (Ventas)

Estructura:
- `CSharp/WinFormsSalesApp`: WinForms (.NET 8) con formularios, lógica, SQL Server y llamadas a Web API.
- `VB6/VB6SalesApp`: VB6 con formularios por módulo (Ventas/Clientes/Productos), lógica y acceso a BD + Web API.
- `SQL/CreateSchema.sql`: Script para crear la BD `SalesDb` y tablas base.

**Pasos rápidos**
1) SQL: Ejecute `SQL/CreateSchema.sql` en su SQL Server.
2) C#: Abra `CSharp/WinFormsSalesApp/WinFormsSalesApp.csproj`, ajuste `appsettings.json` y ejecute.
3) VB6: Abra `VB6/VB6SalesApp/VB6SalesApp.vbp`, revise `config.ini`, active referencias y ejecute.

¡Listo para extender!