using System;
using System.Net.Http;
using System.Net.Http.Json;
using System.Threading.Tasks;

namespace WinFormsSalesApp.Services
{
    public class ApiClient : IDisposable
    {
        private readonly HttpClient _http;

        public ApiClient(string baseUrl)
        {
            _http = new HttpClient { BaseAddress = new Uri(baseUrl) };
        }

        public Task<T?> GetAsync<T>(string relative) => _http.GetFromJsonAsync<T>(relative);

        public async Task<HttpResponseMessage> PostJsonAsync<T>(string relative, T payload)
        {
            var resp = await _http.PostAsJsonAsync(relative, payload);
            resp.EnsureSuccessStatusCode();
            return resp;
        }

        public void Dispose() => _http.Dispose();
    }
}
