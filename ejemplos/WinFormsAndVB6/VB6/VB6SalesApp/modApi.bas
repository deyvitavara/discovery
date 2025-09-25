Attribute VB_Name = "modApi"
Option Explicit

Public Function ApiGet(ByVal relative As String) As String
    On Error GoTo EH
    Dim http As Object
    Set http = CreateObject("WinHttp.WinHttpRequest.5.1")
    http.Open "GET", GetSettingValue("ApiBaseUrl") & relative, False
    http.Send
    If http.Status = 200 Then
        ApiGet = http.ResponseText
    Else
        ApiGet = "HTTP " & http.Status & ": " & http.StatusText
    End If
    Exit Function
EH:
    ApiGet = "Error ApiGet: " & Err.Description
End Function
