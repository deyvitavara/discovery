using System;
using System.Linq;
using System.Windows.Forms;
using WinFormsSalesApp.Infrastructure;
using WinFormsSalesApp.Data;
using WinFormsSalesApp.Models;
using System.ComponentModel;

namespace WinFormsSalesApp.UI
{
    public class CustomersForm : Form
    {
        private readonly SqlRepository _repo;
        private BindingList<Customer> _data = new BindingList<Customer>();
        private DataGridView _grid = new DataGridView(){ Left=10, Top=10, Width=560, Height=300, ReadOnly=true, AutoGenerateColumns=true, SelectionMode=DataGridViewSelectionMode.FullRowSelect};
        private TextBox txtName = new TextBox(){ Left=10, Top=320, Width=280, PlaceholderText="Nombre"};
        private TextBox txtEmail = new TextBox(){ Left=300, Top=320, Width=270, PlaceholderText="Email"};
        private Button btnAdd = new Button(){ Left=10, Top=360, Width=100, Text="Agregar"};
        private Button btnEdit = new Button(){ Left=120, Top=360, Width=100, Text="Editar"};
        private Button btnDel = new Button(){ Left=230, Top=360, Width=100, Text="Eliminar"};
        private Button btnRefresh = new Button(){ Left=340, Top=360, Width=100, Text="Refrescar"};

        public CustomersForm(AppConfig cfg)
        {
            Text = "Clientes";
            Width = 600; Height = 450;
            _repo = new SqlRepository(cfg.ConnectionStrings.DefaultConnection);
            Controls.AddRange(new Control[]{ _grid, txtName, txtEmail, btnAdd, btnEdit, btnDel, btnRefresh });

            Load += (_,__) => RefreshData();
            btnRefresh.Click += (_,__) => RefreshData();
            btnAdd.Click += BtnAdd_Click;
            btnEdit.Click += BtnEdit_Click;
            btnDel.Click += BtnDel_Click;
        }

        private void RefreshData()
        {
            _data = new BindingList<Customer>(_repo.GetCustomers());
            _grid.DataSource = _data;
        }

        private void BtnAdd_Click(object? sender, EventArgs e)
        {
            if (string.IsNullOrWhiteSpace(txtName.Text) || string.IsNullOrWhiteSpace(txtEmail.Text))
            {
                MessageBox.Show("Complete nombre y email.");
                return;
            }
            var c = new Customer{ Name = txtName.Text.Trim(), Email = txtEmail.Text.Trim() };
            _repo.AddCustomer(c);
            RefreshData();
            txtName.Clear(); txtEmail.Clear();
        }

        private void BtnEdit_Click(object? sender, EventArgs e)
        {
            if (_grid.SelectedRows.Count == 0) return;
            var c = (Customer)_grid.SelectedRows[0].DataBoundItem;
            var name = Microsoft.VisualBasic.Interaction.InputBox("Nuevo nombre:", "Editar", c.Name);
            var email = Microsoft.VisualBasic.Interaction.InputBox("Nuevo email:", "Editar", c.Email);
            if (!string.IsNullOrWhiteSpace(name) && !string.IsNullOrWhiteSpace(email))
            {
                c.Name = name; c.Email = email;
                _repo.UpdateCustomer(c);
                RefreshData();
            }
        }

        private void BtnDel_Click(object? sender, EventArgs e)
        {
            if (_grid.SelectedRows.Count == 0) return;
            var c = (Customer)_grid.SelectedRows[0].DataBoundItem;
            if (MessageBox.Show($"¿Eliminar {c.Name}?", "Confirmar", MessageBoxButtons.YesNo) == DialogResult.Yes)
            {
                _repo.DeleteCustomer(c.Id);
                RefreshData();
            }
        }
    }
}
