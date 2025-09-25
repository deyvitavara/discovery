using System.ServiceModel;
namespace AspNetWeb.Services { [ServiceContract] public interface IOrders { [OperationContract] string Ping(); }
public class Orders : IOrders { public string Ping()=> "pong"; } }
