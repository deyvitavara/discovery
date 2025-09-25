<!--#include file="inc/db.inc" -->
<%
Dim cn:Set cn=Server.CreateObject("ADODB.Connection")
cn.Open "Provider=SQLOLEDB;Data Source=.;Initial Catalog=Sales;User Id=sa;Password=123;"
Dim cr:Set cr=Server.CreateObject("CrystalRuntime.Application")
%>
