using System.Web.Mvc;
namespace AspNetWeb.Controllers
{
    public class HomeController : Controller
    {
        public ActionResult Index(){ return View(); }
        public ActionResult Products(){ return Content("Products"); }
    }
}


// ---- LLM ----
// DEMO (stub): parametriza SQL, usa DTOs, separa capas.

// ---- END LLM ----
