using System;
using System.Windows.Forms;
using WinFormsSalesApp.Infrastructure;

namespace WinFormsSalesApp.UI
{
    public class LoginForm : Form
    {
        TextBox txtUser = new TextBox(){ Left=100, Top=30, Width=200, PlaceholderText="usuario"};
        TextBox txtPass = new TextBox(){ Left=100, Top=70, Width=200, UseSystemPasswordChar=true, PlaceholderText="clave"};
        Button btnLogin = new Button(){ Left=100, Top=110, Width=200, Text="Ingresar"};
        Label lbl = new Label(){ Left=20, Top=10, Text="Inicio de sesión", AutoSize=true };

        public LoginForm()
        {
            Text = "WinFormsSalesApp - Login";
            Controls.Add(lbl);
            Controls.Add(new Label(){Left=20, Top=30, Text="Usuario", AutoSize=true});
            Controls.Add(txtUser);
            Controls.Add(new Label(){Left=20, Top=70, Text="Clave", AutoSize=true});
            Controls.Add(txtPass);
            Controls.Add(btnLogin);
            AcceptButton = btnLogin;
            btnLogin.Click += BtnLogin_Click;
            StartPosition = FormStartPosition.CenterScreen;
            FormBorderStyle = FormBorderStyle.FixedDialog;
            MaximizeBox = false;
            MinimizeBox = false;
            Width = 340; Height = 200;
        }

        private void BtnLogin_Click(object? sender, EventArgs e)
        {
            if (string.IsNullOrWhiteSpace(txtUser.Text) || string.IsNullOrWhiteSpace(txtPass.Text))
            {
                MessageBox.Show("Ingrese usuario y clave.", "Aviso", MessageBoxButtons.OK, MessageBoxIcon.Information);
                return;
            }
            // Demo: permitir todo
            var cfg = AppConfig.Load();
            Hide();
            new MainForm(cfg).Show();
        }
    }
}
