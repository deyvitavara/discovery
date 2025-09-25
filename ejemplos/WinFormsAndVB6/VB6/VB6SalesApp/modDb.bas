Attribute VB_Name = "modDb"
Public Sub OpenDb(ByVal cs As String)
    Dim cn
    Set cn = CreateObject("ADODB.Connection")
    cn.Open cs
End Sub
