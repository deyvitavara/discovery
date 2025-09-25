using System.Web;
namespace AspNetWeb.Handlers { public class Download : IHttpHandler { public void ProcessRequest(HttpContext ctx){} public bool IsReusable=>false; } }
