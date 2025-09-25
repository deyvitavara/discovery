using System;
using System.Windows.Forms;
using WinFormsSalesApp.Infrastructure;
using WinFormsSalesApp.Services;

namespace WinFormsSalesApp.UI
{
    public class MainForm : Form
    {
        private readonly AppConfig _config;

        public MainForm(AppConfig cfg)
        {
            _config = cfg;
            Text = "WinFormsSalesApp";
            Width = 900; Height = 600;

            var menu = new MenuStrip();
            var mVentas = new ToolStripMenuItem("Ventas");
            var mClientes = new ToolStripMenuItem("Clientes", null, (_,__) => new CustomersForm(_config).ShowDialog());
            var mProductos = new ToolStripMenuItem("Productos", null, (_,__) => MessageBox.Show("TODO: CRUD Productos"));
            var mOrdenes = new ToolStripMenuItem("Órdenes", null, (_,__) => MessageBox.Show("TODO: CRUD Órdenes"));
            mVentas.DropDownItems.AddRange(new []{ mClientes, mProductos, mOrdenes });

            var mApi = new ToolStripMenuItem("API");
            var mPing = new ToolStripMenuItem("Probar GET /posts/1", null, async (_,__) =>
            {
                using var api = new ApiClient(_config.Api.BaseUrl);
                var post = await api.GetAsync<object>("posts/1");
                MessageBox.Show(System.Text.Json.JsonSerializer.Serialize(post, new System.Text.Json.JsonSerializerOptions{WriteIndented=true}), "API OK");
            });
            mApi.DropDownItems.Add(mPing);

            var mConfig = new ToolStripMenuItem("Configuración");
            var mConn = new ToolStripMenuItem("Ver conexión", null, (_,__) => MessageBox.Show(_config.ConnectionStrings.DefaultConnection, "Cadena de conexión"));
            mConfig.DropDownItems.Add(mConn);

            menu.Items.AddRange(new []{ mVentas, mApi, mConfig });
            MainMenuStrip = menu;
            Controls.Add(menu);

            var lbl = new Label(){ Left=20, Top=60, AutoSize=true, Text="Bienvenido. Use el menú para navegar."};
            Controls.Add(lbl);
        }
    }
}
