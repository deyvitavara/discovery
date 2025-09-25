using System.Web.Services;
namespace AspNetWeb.Services { [WebService] public class Legacy : WebService { [HttpPost] public string HelloWorld(){ return "Hello"; } } }


// ---- LLM ----
// DEMO (stub): parametriza SQL, usa DTOs, separa capas.

// ---- END LLM ----
