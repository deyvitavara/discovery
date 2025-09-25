using System;
using System.Windows.Forms;
namespace SalesWinForms
{
    public class MainForm : Form
    {
        public MainForm()
        {
            Text = "MainForm";
            var bLogin = new Button{Text="Login"}; bLogin.Click += (s,e)=> new LoginForm().Show();
            var bProducts = new Button{Text="Products"}; bProducts.Click += (s,e)=> new ProductsForm().Show();
            var bSales = new Button{Text="Sales"}; bSales.Click += (s,e)=> new SalesForm().Show();
            var flow = new FlowLayoutPanel{Dock=DockStyle.Fill};
            flow.Controls.AddRange(new Control[]{bLogin,bProducts,bSales});
            Controls.Add(flow);
        }
    }
}
