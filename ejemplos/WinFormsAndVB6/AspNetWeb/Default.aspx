<%@ Page Language="C#" AutoEventWireup="true" CodeBehind="Default.aspx.cs" Inherits="AspNetWeb._Default" MasterPageFile="~/Site.Master" %>
<%@ Register Src="Controls/ProductList.ascx" TagPrefix="uc" TagName="ProductList" %>
<asp:Content ContentPlaceHolderID="MainContent" runat="server">
    <uc:ProductList runat="server" ID="pl1" />
</asp:Content>
