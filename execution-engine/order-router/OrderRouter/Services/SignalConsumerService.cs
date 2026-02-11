using Confluent.Kafka;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;
using OrderRouter.Models;
using System.Text.Json;

namespace OrderRouter.Services;

public class SignalConsumerService : BackgroundService
{
    private readonly ILogger<SignalConsumerService> _logger;
    private readonly IOrderRouterService _orderRouterService;

    public SignalConsumerService(
        ILogger<SignalConsumerService> logger,
        IOrderRouterService orderRouterService)
    {
        _logger = logger;
        _orderRouterService = orderRouterService;
    }

    protected override async Task ExecuteAsync(CancellationToken stoppingToken)
    {
        var config = new ConsumerConfig
        {
            GroupId = "order-router",
            BootstrapServers = "localhost:9092",
            AutoOffsetReset = AutoOffsetReset.Earliest
        };

        using var consumer = new ConsumerBuilder<Ignore, string>(config).Build();
        consumer.Subscribe("signals");

        _logger.LogInformation("Signal consumer started");

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

                            // Only execute high confidence signals
                            if (signal.ConfidenceScore > 0.7)
                            {
                                await ExecuteSignalAsync(signal);
                            }
                            else
                            {
                                _logger.LogInformation($"Signal confidence {signal.ConfidenceScore} below threshold 0.7, skipping");
                            }
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
            _logger.LogInformation("Signal consumer stopping");
        }
    }

    private async Task ExecuteSignalAsync(Signal signal)
    {
        try
        {
            var orderRequest = new OrderRequest
            {
                Symbol = signal.Symbol,
                Side = signal.Direction,
                Type = "market", // or "limit" based on signal
                Quantity = (decimal)CalculateQuantity(signal),
                Price = signal.Direction == "buy" ? (decimal?)signal.EntryPrice : null, // null for market orders
                PreferredExchange = null // Let router decide
            };

            var response = await _orderRouterService.RouteOrderAsync(orderRequest);

            _logger.LogInformation($"Executed order for signal: {signal.Symbol}, Response: {response.Status}");
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, $"Error executing signal for {signal.Symbol}");
        }
    }

    private double CalculateQuantity(Signal signal)
    {
        // Simple position sizing based on confidence and risk
        // In production, this would consider portfolio size, risk management, etc.
        double baseQuantity = 0.01; // Small base quantity for crypto
        double confidenceMultiplier = signal.ConfidenceScore;

        return baseQuantity * confidenceMultiplier;
    }
}