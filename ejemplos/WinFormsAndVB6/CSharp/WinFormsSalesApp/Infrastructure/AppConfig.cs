using System.IO;
using System.Text.Json;

namespace WinFormsSalesApp.Infrastructure
{
    public class AppConfig
    {
        public ConnectionStrings ConnectionStrings { get; set; } = new();
        public Api Api { get; set; } = new();

        public static AppConfig Load()
        {
            var path = Path.Combine(AppContext.BaseDirectory, "appsettings.json");
            var json = File.ReadAllText(path);
            return JsonSerializer.Deserialize<AppConfig>(json) ?? new AppConfig();
        }
    }

    public class ConnectionStrings
    {
        public string DefaultConnection { get; set; } = "";
    }

    public class Api
    {
        public string BaseUrl { get; set; } = "";
    }
}
