VERSION 5.00
Begin VB.Form frmMain 
   Caption         =   "Main"
End
Attribute VB_Name = "frmMain"
Private Sub Form_Load()
    Load frmLogin: frmLogin.Show
End Sub
Private Sub mnuProducts_Click()
    frmProducts.Show
End Sub
Private Sub mnuSales_Click()
    frmSales.Show
End Sub
