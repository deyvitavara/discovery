Version 1.0 Class
Attribute VB_Name = "clsReport"
Dim crx As Object
Private Sub Class_Initialize()
    Set crx = CreateObject("CRAXDRT.Application")
End Sub
