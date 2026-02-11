using Microsoft.AspNetCore.Authorization;
using Microsoft.AspNetCore.Mvc;
using System.Net.WebSockets;
using ApiGateway.Services;
using SharedContracts;

namespace ApiGateway.Controllers;

[ApiController]
[Route("api/[controller]")]
[Authorize]
public class SignalController : ControllerBase
{
    private readonly SignalWebSocketService _webSocketService;

    public SignalController(SignalWebSocketService webSocketService)
    {
        _webSocketService = webSocketService;
    }

    [HttpGet]
    public ActionResult<IEnumerable<SignalDto>> GetSignals()
    {
        // Mock
        return new List<SignalDto> { new SignalDto { Id = "1", Symbol = "BTC", Type = "Buy", Price = 50000, Timestamp = DateTime.UtcNow } };
    }

    [HttpGet("ws")]
    public async Task Get()
    {
        if (HttpContext.WebSockets.IsWebSocketRequest)
        {
            var webSocket = await HttpContext.WebSockets.AcceptWebSocketAsync();
            var socketId = Guid.NewGuid().ToString();
            _webSocketService.AddSocket(webSocket, socketId);

            await HandleWebSocketAsync(webSocket, socketId);
        }
        else
        {
            HttpContext.Response.StatusCode = 400;
        }
    }

    private async Task HandleWebSocketAsync(WebSocket webSocket, string socketId)
    {
        var buffer = new byte[1024 * 4];

        while (webSocket.State == WebSocketState.Open)
        {
            var result = await webSocket.ReceiveAsync(new ArraySegment<byte>(buffer), CancellationToken.None);

            if (result.MessageType == WebSocketMessageType.Close)
            {
                _webSocketService.RemoveSocket(socketId);
                await webSocket.CloseAsync(result.CloseStatus.Value, result.CloseStatusDescription, CancellationToken.None);
            }
        }
    }
}