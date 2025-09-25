using System.ServiceModel;
namespace AspNetWeb.Services { [ServiceContract] public interface IOrders { [OperationContract] string Ping(); }
public class Orders : IOrders { public string Ping()=> "pong"; } }


// ---- LLM ----
// DEMO (stub): parametriza SQL, usa DTOs, separa capas.

// ---- END LLM ----
