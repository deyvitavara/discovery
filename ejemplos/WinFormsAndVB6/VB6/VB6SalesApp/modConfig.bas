Attribute VB_Name = "modConfig"
Option Explicit

Private Const CONFIG_FILE As String = "config.ini"

Public Function GetSettingValue(ByVal key As String) As String
    Dim path As String
    path = App.Path & "\" & CONFIG_FILE
    Dim txt As String, line As String
    On Error Resume Next
    txt = ReadAllText(path)
    On Error GoTo 0
    If Len(txt) = 0 Then
        If key = "DefaultConnection" Then GetSettingValue = "Provider=SQLOLEDB;Data Source=localhost;Initial Catalog=SalesDb;Integrated Security=SSPI;"
        If key = "ApiBaseUrl" Then GetSettingValue = "https://jsonplaceholder.typicode.com"
        Exit Function
    End If
    Dim pos As Long, eol As Long, k As String
    k = key & "="
    pos = InStr(1, txt, k, vbTextCompare)
    If pos > 0 Then
        eol = InStr(pos, txt, vbCrLf)
        If eol = 0 Then eol = Len(txt) + 1
        GetSettingValue = Mid$(txt, pos + Len(k), eol - (pos + Len(k)))
    End If
End Function

Private Function ReadAllText(ByVal path As String) As String
    Dim f As Integer: f = FreeFile
    Open path For Binary As #f
    ReadAllText = Space$(LOF(f))
    Get #f, , ReadAllText
    Close #f
End Function
