using System;
using System.Windows.Forms;
namespace SalesWinForms
{
    public class ProductsForm : Form
    {
        public ProductsForm()
        {
            Text = "ProductsForm";
            var lbl = new Label{Dock=DockStyle.Fill, Text="Usa ReportViewer con Report1.rdlc"};
            Controls.Add(lbl);
        }
    }
}
