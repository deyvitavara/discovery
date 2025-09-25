VERSION 5.00
Begin VB.Form frmSales 
   Caption         =   "Sales"
End
Attribute VB_Name = "frmSales"
Private Sub cmdSoap_Click()
    Dim sc
    Set sc = CreateObject("MSSOAP.SoapClient30")
    sc.MSSoapInit "http://localhost/Services/Legacy.asmx?wsdl"
    MsgBox sc.HelloWorld()
End Sub
Private Sub cmdExcel_Click()
    Dim xl
    Set xl = CreateObject("Excel.Application")
    xl.Workbooks.Add
    xl.Visible = False
    xl.Application.Quit
End Sub
