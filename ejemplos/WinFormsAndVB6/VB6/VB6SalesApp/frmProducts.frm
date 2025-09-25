VERSION 5.00
Begin VB.Form frmProducts 
   Caption         =   "Products"
End
Attribute VB_Name = "frmProducts"
Private Sub cmdReport_Click()
    Dim rptApp
    Set rptApp = CreateObject("CrystalRuntime.Application")
    Dim rpt
    Set rpt = rptApp.OpenReport("Reports\Legacy.rpt")
    rpt.PrintOut False
End Sub
