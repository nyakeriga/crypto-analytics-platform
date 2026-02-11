using Confluent.Kafka;
using Microsoft.Extensions.Hosting;
using Microsoft.Extensions.Logging;
using signal_service.Data;
using SharedContracts;
using System.Text.Json;

namespace signal_service.Services;

public class KafkaConsumerService : BackgroundService
{
    private readonly ILogger<KafkaConsumerService> _logger;
    private readonly SignalDbContext _context;
    private readonly IConfiguration _configuration;

    public KafkaConsumerService(ILogger<KafkaConsumerService> logger, SignalDbContext context, IConfiguration configuration)
    {
        _logger = logger;
        _context = context;
        _configuration = configuration;
    }

    protected override async Task ExecuteAsync(CancellationToken stoppingToken)
    {
        var config = new ConsumerConfig
        {
            BootstrapServers = _configuration["Kafka:BootstrapServers"],
            GroupId = "signal-service-group",
            AutoOffsetReset = AutoOffsetReset.Earliest
        };

        using var consumer = new ConsumerBuilder<Ignore, string>(config).Build();
        consumer.Subscribe("signals");

        _logger.LogInformation("Kafka consumer started.");

        while (!stoppingToken.IsCancellationRequested)
        {
            try
            {
                var consumeResult = consumer.Consume(stoppingToken);
                if (consumeResult.Message != null)
                {
                    var signalDto = JsonSerializer.Deserialize<SignalDto>(consumeResult.Message.Value);
                    if (signalDto != null)
                    {
                        // Validate
                        if (string.IsNullOrEmpty(signalDto.Symbol) || signalDto.Confidence < 0 || signalDto.Confidence > 1)
                        {
                            _logger.LogWarning("Invalid signal received: {Signal}", consumeResult.Message.Value);
                            continue;
                        }

                        var signal = new Signal
                        {
                            Symbol = signalDto.Symbol,
                            Type = signalDto.Type,
                            Price = signalDto.Price,
                            Confidence = signalDto.Confidence,
                            Timestamp = signalDto.Timestamp
                        };

                        _context.Signals.Add(signal);
                        await _context.SaveChangesAsync();
                        _logger.LogInformation("Signal saved: {Id}", signal.Id);
                    }
                }
            }
            catch (Exception ex)
            {
                _logger.LogError(ex, "Error processing Kafka message.");
            }
        }
    }
}