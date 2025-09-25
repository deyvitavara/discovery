using System;
using System.ServiceModel;
using System.Web.Services.Protocols;
using System.Windows.Forms;
namespace SalesWinForms
{
    public class LegacyService : SoapHttpClientProtocol
    {
        public LegacyService(){ Url = "http://localhost/Services/Legacy.asmx"; }
        public string HelloWorld(){ return "Hello"; }
    }
    public class SalesForm : Form
    {
        public SalesForm()
        {
            Text = "SalesForm";
            var bSoap = new Button{Text="ASMX"}; bSoap.Click += (s,e)=> new LegacyService().HelloWorld();
            var bWcf  = new Button{Text="WCF"};  bWcf.Click  += (s,e)=> TestWcf();
            var bCom  = new Button{Text="COM"};  bCom.Click  += (s,e)=> UseExcelCom();
            var flow = new FlowLayoutPanel{Dock=DockStyle.Fill};
            flow.Controls.AddRange(new Control[]{bSoap,bWcf,bCom});
            Controls.Add(flow);
        }
        void TestWcf()
        {
            var binding = new BasicHttpBinding();
            var addr    = new EndpointAddress("http://localhost/Services/Orders.svc");
        }
        void UseExcelCom()
        {
            var t = Type.GetTypeFromProgID("Excel.Application");
            if(t!=null){ var excel = Activator.CreateInstance(t); }
        }
    }
}
