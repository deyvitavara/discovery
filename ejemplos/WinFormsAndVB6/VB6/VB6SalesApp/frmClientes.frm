VERSION 5.00
Begin VB.Form frmClientes 
   Caption         =   "Clientes"
   ClientHeight    =   4095
   ClientLeft      =   45
   ClientTop       =   330
   ClientWidth     =   6210
   ScaleHeight     =   4095
   ScaleWidth      =   6210
   StartUpPosition =   2  'CenterScreen
   Begin VB.TextBox txtNombre 
      Height          =   285
      Left            =   120
      TabIndex        =   0
      Top             =   360
      Width           =   2295
   End
   Begin VB.TextBox txtEmail 
      Height          =   285
      Left            =   2520
      TabIndex        =   1
      Top             =   360
      Width           =   3495
   End
   Begin VB.CommandButton cmdAgregar 
      Caption         =   "Agregar"
      Height          =   375
      Left            =   120
      TabIndex        =   2
      Top             =   840
      Width           =   1215
   End
   Begin VB.CommandButton cmdListar 
      Caption         =   "Listar"
      Height          =   375
      Left            =   1440
      TabIndex        =   3
      Top             =   840
      Width           =   1215
   End
   Begin VB.ListBox lstClientes 
      Height          =   2205
      Left            =   120
      TabIndex        =   4
      Top             =   1320
      Width           =   5895
   End
   Begin VB.Label Label1 
      Caption         =   "Nombre"
      Height          =   255
      Left            =   120
      TabIndex        =   5
      Top             =   120
      Width           =   855
   End
   Begin VB.Label Label2 
      Caption         =   "Email"
      Height          =   255
      Left            =   2520
      TabIndex        =   6
      Top             =   120
      Width           =   735
   End
End
Attribute VB_Name = "frmClientes"
Attribute VB_GlobalNameSpace = False
Attribute VB_Creatable = False
Attribute VB_PredeclaredId = True
Attribute VB_Exposed = False

Private Sub cmdAgregar_Click()
    If Len(Trim$(txtNombre.Text)) = 0 Or Len(Trim$(txtEmail.Text)) = 0 Then
        MsgBox "Complete nombre y email."
        Exit Sub
    End If
    DbExec "INSERT INTO Customers(Name, Email) VALUES(?,?)", Array(txtNombre.Text, txtEmail.Text)
    txtNombre.Text = "": txtEmail.Text = ""
    cmdListar_Click
End Sub

Private Sub cmdListar_Click()
    Dim rs As Object
    Set rs = DbQuery("SELECT Id, Name, Email FROM Customers ORDER BY Id")
    lstClientes.Clear
    If Not rs Is Nothing Then
        Do Until rs.EOF
            lstClientes.AddItem rs.Fields(0).Value & " - " & rs.Fields(1).Value & " <" & rs.Fields(2).Value & ">"
            rs.MoveNext
        Loop
        rs.Close
        Set rs = Nothing
    End If
End Sub
