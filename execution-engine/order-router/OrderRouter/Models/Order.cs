namespace OrderRouter.Models;

public class Order
{
    public string Id { get; set; } = Guid.NewGuid().ToString();
    public string Symbol { get; set; } = string.Empty;
    public string Side { get; set; } = string.Empty; // "buy" or "sell"
    public string Type { get; set; } = string.Empty; // "market", "limit"
    public decimal Quantity { get; set; }
    public decimal? Price { get; set; } // For limit orders
    public string Exchange { get; set; } = string.Empty; // "binance", "bybit", "okx"
    public DateTime Timestamp { get; set; } = DateTime.UtcNow;
    public string Status { get; set; } = "pending"; // "pending", "filled", "cancelled"
}

public class OrderRequest
{
    public string Symbol { get; set; } = string.Empty;
    public string Side { get; set; } = string.Empty;
    public string Type { get; set; } = string.Empty;
    public decimal Quantity { get; set; }
    public decimal? Price { get; set; }
    public string? PreferredExchange { get; set; } // Optional
}

public class OrderResponse
{
    public string OrderId { get; set; } = string.Empty;
    public string Status { get; set; } = string.Empty;
    public string Exchange { get; set; } = string.Empty;
    public string Message { get; set; } = string.Empty;
}