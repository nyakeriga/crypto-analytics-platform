using Confluent.Kafka;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;
using TradingBot.Models;
using System.Text.Json;

namespace TradingBot.Services;

public class TradingBotService : BackgroundService
{
    private readonly ILogger<TradingBotService> _logger;
    private readonly HttpClient _httpClient;
    private readonly string _orderRouterUrl;

    public TradingBotService(ILogger<TradingBotService> logger, IConfiguration configuration)
    {
        _logger = logger;
        _httpClient = new HttpClient();
        _orderRouterUrl = configuration["OrderRouter:Url"] ?? "http://localhost:5001";
    }

    protected override async Task ExecuteAsync(CancellationToken stoppingToken)
    {
        var config = new ConsumerConfig
        {
            GroupId = "trading-bot",
            BootstrapServers = "localhost:9092",
            AutoOffsetReset = AutoOffsetReset.Earliest
        };

        using var consumer = new ConsumerBuilder<Ignore, string>(config).Build();
        consumer.Subscribe("signals");

        _logger.LogInformation("Trading bot started - monitoring signals for automated trading");

        try
        {
            while (!stoppingToken.IsCancellationRequested)
            {
                try
                {
                    var consumeResult = consumer.Consume(stoppingToken);

                    if (consumeResult?.Message?.Value != null)
                    {
                        var signal = JsonSerializer.Deserialize<Signal>(consumeResult.Message.Value);

                        if (signal != null)
                        {
                            _logger.LogInformation($"Received signal: {signal.Symbol} {signal.Direction} confidence: {signal.ConfidenceScore}");

                            // Execute trade based on signal
                            await ExecuteTradeAsync(signal);
                        }
                    }
                }
                catch (ConsumeException ex)
                {
                    _logger.LogError(ex, "Error consuming signal");
                }
                catch (JsonException ex)
                {
                    _logger.LogError(ex, "Error deserializing signal");
                }
            }
        }
        catch (OperationCanceledException)
        {
            _logger.LogInformation("Trading bot stopping");
        }
    }

    private async Task ExecuteTradeAsync(Signal signal)
    {
        try
        {
            // Check if trading is enabled (kill switch logic would go here)
            if (!await IsTradingEnabledAsync())
            {
                _logger.LogWarning("Trading is disabled via kill switch");
                return;
            }

            // Apply risk management and position sizing
            var orderRequest = BuildOrderRequest(signal);

            if (orderRequest.Quantity <= 0)
            {
                _logger.LogInformation("Order quantity is zero, skipping trade");
                return;
            }

            // Call order-router API
            var response = await _httpClient.PostAsJsonAsync($"{_orderRouterUrl}/api/order/place", orderRequest);

            if (response.IsSuccessStatusCode)
            {
                var orderResponse = await response.Content.ReadFromJsonAsync<OrderResponse>();
                _logger.LogInformation($"Trade executed successfully: {signal.Symbol} {signal.Direction}, OrderId: {orderResponse?.OrderId}");
            }
            else
            {
                var error = await response.Content.ReadAsStringAsync();
                _logger.LogError($"Failed to execute trade: {response.StatusCode} - {error}");
            }
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, $"Error executing trade for {signal.Symbol}");
        }
    }

    private OrderRequest BuildOrderRequest(Signal signal)
    {
        // Position sizing based on confidence and risk management
        var quantity = CalculatePositionSize(signal);

        return new OrderRequest
        {
            Symbol = signal.Symbol,
            Side = signal.Direction,
            Type = "market", // Use market orders for automated trading
            Quantity = quantity,
            Price = null, // Market order
            PreferredExchange = null // Let router decide
        };
    }

    private decimal CalculatePositionSize(Signal signal)
    {
        // Simple position sizing: base size * confidence multiplier * risk adjustment
        const decimal baseSize = 0.01m; // Small base size for crypto
        var confidenceMultiplier = (decimal)signal.ConfidenceScore;
        var riskMultiplier = 1.0m - (decimal)signal.Layers.Risk; // Reduce size if risk is high

        return baseSize * confidenceMultiplier * riskMultiplier;
    }

    private async Task<bool> IsTradingEnabledAsync()
    {
        // Check the kill switch status
        return TradingBot.Controllers.KillSwitchController.IsTradingEnabled();
    }
}