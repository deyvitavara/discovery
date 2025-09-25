VERSION 5.00
Begin VB.Form frmLogin 
   Caption         =   "Login"
End
Attribute VB_Name = "frmLogin"
Private Sub cmdOk_Click()
    Call OpenDb("Provider=SQLOLEDB;Data Source=.;Initial Catalog=Sales;User Id=sa;Password=123;")
End Sub
