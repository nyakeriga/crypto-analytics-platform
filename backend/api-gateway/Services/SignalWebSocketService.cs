using System.Collections.Concurrent;
using System.Net.WebSockets;
using System.Text;
using Confluent.Kafka;
using SharedContracts;

namespace ApiGateway.Services;

public class SignalWebSocketService : BackgroundService
{
    private readonly ConcurrentDictionary<string, WebSocket> _sockets = new();
    private readonly ILogger<SignalWebSocketService> _logger;

    public SignalWebSocketService(ILogger<SignalWebSocketService> logger)
    {
        _logger = logger;
    }

    public void AddSocket(WebSocket socket, string id)
    {
        _sockets.TryAdd(id, socket);
    }

    public void RemoveSocket(string id)
    {
        _sockets.TryRemove(id, out _);
    }

    public async Task BroadcastSignalAsync(SignalDto signal)
    {
        var message = System.Text.Json.JsonSerializer.Serialize(signal);
        var buffer = Encoding.UTF8.GetBytes(message);

        foreach (var socket in _sockets.Values)
        {
            if (socket.State == WebSocketState.Open)
            {
                await socket.SendAsync(new ArraySegment<byte>(buffer), WebSocketMessageType.Text, true, CancellationToken.None);
            }
        }
    }

    protected override async Task ExecuteAsync(CancellationToken stoppingToken)
    {
        var config = new ConsumerConfig
        {
            BootstrapServers = "localhost:9092", // Replace with actual
            GroupId = "api-gateway-signals",
            AutoOffsetReset = AutoOffsetReset.Earliest
        };

        using var consumer = new ConsumerBuilder<Ignore, string>(config).Build();
        consumer.Subscribe("signals-topic"); // Replace with actual topic

        while (!stoppingToken.IsCancellationRequested)
        {
            try
            {
                var consumeResult = consumer.Consume(stoppingToken);
                var signal = System.Text.Json.JsonSerializer.Deserialize<SignalDto>(consumeResult.Message.Value);
                if (signal != null)
                {
                    await BroadcastSignalAsync(signal);
                }
            }
            catch (Exception ex)
            {
                _logger.LogError(ex, "Error consuming Kafka message");
            }
        }
    }
}