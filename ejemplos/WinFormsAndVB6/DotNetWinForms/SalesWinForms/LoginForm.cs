using System;
using System.Data.SqlClient;
using System.Windows.Forms;
namespace SalesWinForms
{
    public class LoginForm : Form
    {
        public LoginForm()
        {
            Text = "LoginForm";
            var b = new Button{Text="Test DB"}; b.Click += (s,e)=> TestDb();
            Controls.Add(b);
        }
        void TestDb()
        {
            var cs = "Data Source=.;Initial Catalog=Sales;Integrated Security=True";
            using var cn = new SqlConnection(cs);
            try { cn.Open(); MessageBox.Show("OK DB"); } catch {}
        }
    }
}
