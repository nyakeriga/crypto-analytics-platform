using OrderRouter.Models;
using Microsoft.Extensions.Logging;

namespace OrderRouter.Services;

public interface IOrderRouterService
{
    Task<OrderResponse> RouteOrderAsync(OrderRequest request);
}

public class OrderRouterService : IOrderRouterService
{
    private readonly ILogger<OrderRouterService> _logger;
    private readonly Dictionary<string, IExchangeConnector> _exchanges;

    public OrderRouterService(ILogger<OrderRouterService> logger)
    {
        _logger = logger;
        _exchanges = new Dictionary<string, IExchangeConnector>
        {
            ["binance"] = new BinanceConnector(),
            ["bybit"] = new BybitConnector(),
            ["okx"] = new OkxConnector()
        };
    }

    public async Task<OrderResponse> RouteOrderAsync(OrderRequest request)
    {
        try
        {
            string selectedExchange = SelectExchange(request);
            var connector = _exchanges[selectedExchange];

            var order = new Order
            {
                Symbol = request.Symbol,
                Side = request.Side,
                Type = request.Type,
                Quantity = request.Quantity,
                Price = request.Price,
                Exchange = selectedExchange
            };

            var response = await connector.PlaceOrderAsync(order);

            _logger.LogInformation($"Order {order.Id} routed to {selectedExchange}");

            return new OrderResponse
            {
                OrderId = order.Id,
                Status = response.Status,
                Exchange = selectedExchange,
                Message = response.Message
            };
        }
        catch (Exception ex)
        {
            _logger.LogError(ex, "Error routing order");
            return new OrderResponse
            {
                Status = "failed",
                Message = ex.Message
            };
        }
    }

    private string SelectExchange(OrderRequest request)
    {
        // Simple logic: prefer preferred exchange if available, else round-robin or based on some criteria
        if (!string.IsNullOrEmpty(request.PreferredExchange) && _exchanges.ContainsKey(request.PreferredExchange))
        {
            return request.PreferredExchange;
        }

        // For demo, cycle through exchanges
        var exchanges = _exchanges.Keys.ToArray();
        return exchanges[new Random().Next(exchanges.Length)];
    }
}

// Exchange connector interfaces and implementations
public interface IExchangeConnector
{
    Task<OrderResponse> PlaceOrderAsync(Order order);
}

public class BinanceConnector : IExchangeConnector
{
    public async Task<OrderResponse> PlaceOrderAsync(Order order)
    {
        // Simulate API call
        await Task.Delay(100);
        return new OrderResponse
        {
            Status = "filled",
            Message = $"Order placed on Binance for {order.Symbol}"
        };
    }
}

public class BybitConnector : IExchangeConnector
{
    public async Task<OrderResponse> PlaceOrderAsync(Order order)
    {
        await Task.Delay(100);
        return new OrderResponse
        {
            Status = "filled",
            Message = $"Order placed on Bybit for {order.Symbol}"
        };
    }
}

public class OkxConnector : IExchangeConnector
{
    public async Task<OrderResponse> PlaceOrderAsync(Order order)
    {
        await Task.Delay(100);
        return new OrderResponse
        {
            Status = "filled",
            Message = $"Order placed on OKX for {order.Symbol}"
        };
    }
}