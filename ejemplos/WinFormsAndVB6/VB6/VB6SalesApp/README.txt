VB6SalesApp

**Qué trae**
- Formularios: Login, Principal (botones), Clientes (insertar/listar), Productos, Ventas.
- Lógica: módulo `modDb.bas` (ADODB), `modApi.bas` (WinHTTP), `modConfig.bas` (lee `config.ini`).
- Acceso a SQL Server: OLE DB (SQLOLEDB) con comandos simples.
- Llamada a Web Service: GET a `ApiBaseUrl + /posts/1`.

**Cómo abrir**
1. Abra `VB6SalesApp.vbp` en VB6 IDE.
2. En *Project > References*, marque:
   - **Microsoft ActiveX Data Objects 2.x Library**
   - **Microsoft XML, v6.0** (o use WinHTTP ya incluido)
3. Ajuste `config.ini` con su cadena de conexión y `ApiBaseUrl`.
4. Ejecute (F5).

**Notas**
- `frmClientes` inserta y lista de la tabla `Customers`.
- Puede ampliar Productos/Ventas siguiendo el patrón de `DbQuery`/`DbExec`.
