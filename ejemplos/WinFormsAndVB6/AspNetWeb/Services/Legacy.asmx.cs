using System.Web.Services;
namespace AspNetWeb.Services { [WebService] public class Legacy : WebService { [WebMethod] public string HelloWorld(){ return "Hello"; } } }
