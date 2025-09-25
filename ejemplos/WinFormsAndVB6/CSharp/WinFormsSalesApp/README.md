# WinFormsSalesApp (.NET 8, Windows Forms)

**Qué trae**
- Formularios: Login, Main (menú), Clientes (CRUD básico).
- Lógica/Capas: `Models`, `Data` (SqlRepository), `Services` (ApiClient), `Infrastructure` (config).
- Acceso a SQL Server vía `System.Data.SqlClient` (consultas parametrizadas).
- Llamadas a Web API con `HttpClient` (ejemplo GET a `/posts/1`).

**Cómo ejecutar**
1. Abra `WinFormsSalesApp.csproj` con Visual Studio 2022+ (o `dotnet build`).
2. Ajuste `appsettings.json`:
   - `"ConnectionStrings:DefaultConnection"`: su cadena de conexión a SQL Server.
   - `"Api:BaseUrl"`: su endpoint de API.
3. Cree la base de datos y tablas con `../SQL/CreateSchema.sql`.
4. F5 para ejecutar. Ingrese cualquier usuario/clave (demo) y use el menú *Ventas > Clientes*.

**Notas**
- El CRUD de Productos y Órdenes está preparado para implementar.
- Puede convertir a capas/proyectos separados si lo desea.
