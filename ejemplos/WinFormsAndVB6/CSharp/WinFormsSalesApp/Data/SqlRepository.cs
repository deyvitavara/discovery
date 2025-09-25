using System;
using System.Collections.Generic;
using System.Data;
using System.Data.SqlClient;
using WinFormsSalesApp.Models;

namespace WinFormsSalesApp.Data
{
    public class SqlRepository
    {
        private readonly string _connStr;

        public SqlRepository(string connectionString)
        {
            _connStr = connectionString;
        }

        public List<Customer> GetCustomers()
        {
            var list = new List<Customer>();
            using var cn = new SqlConnection(_connStr);
            using var cmd = new SqlCommand("SELECT Id, Name, Email FROM Customers ORDER BY Id", cn);
            cn.Open();
            using var rd = cmd.ExecuteReader();
            while (rd.Read())
            {
                list.Add(new Customer
                {
                    Id = rd.GetInt32(0),
                    Name = rd.GetString(1),
                    Email = rd.GetString(2)
                });
            }
            return list;
        }

        public int AddCustomer(Customer c)
        {
            using var cn = new SqlConnection(_connStr);
            using var cmd = new SqlCommand("INSERT INTO Customers(Name, Email) VALUES(@Name,@Email); SELECT SCOPE_IDENTITY();", cn);
            cmd.Parameters.Add("@Name", SqlDbType.NVarChar, 100).Value = c.Name;
            cmd.Parameters.Add("@Email", SqlDbType.NVarChar, 100).Value = c.Email;
            cn.Open();
            var id = Convert.ToInt32(cmd.ExecuteScalar());
            return id;
        }

        public void UpdateCustomer(Customer c)
        {
            using var cn = new SqlConnection(_connStr);
            using var cmd = new SqlCommand("UPDATE Customers SET Name=@Name, Email=@Email WHERE Id=@Id", cn);
            cmd.Parameters.Add("@Name", SqlDbType.NVarChar, 100).Value = c.Name;
            cmd.Parameters.Add("@Email", SqlDbType.NVarChar, 100).Value = c.Email;
            cmd.Parameters.Add("@Id", SqlDbType.Int).Value = c.Id;
            cn.Open();
            cmd.ExecuteNonQuery();
        }

        public void DeleteCustomer(int id)
        {
            using var cn = new SqlConnection(_connStr);
            using var cmd = new SqlCommand("DELETE FROM Customers WHERE Id=@Id", cn);
            cmd.Parameters.Add("@Id", SqlDbType.Int).Value = id;
            cn.Open();
            cmd.ExecuteNonQuery();
        }
    }
}
