namespace TradingBot.Models;

public class Signal
{
    public string Symbol { get; set; } = string.Empty;
    public long Timestamp { get; set; }
    public string Direction { get; set; } = string.Empty; // "buy" or "sell"
    public double ConfidenceScore { get; set; }
    public SignalLayers Layers { get; set; } = new();
    public double EntryPrice { get; set; }
    public double StopLoss { get; set; }
    public double TakeProfit { get; set; }
}

public class SignalLayers
{
    public double Structure { get; set; }
    public double Ob { get; set; }
    public double Momentum { get; set; }
    public double Volatility { get; set; }
    public double Risk { get; set; }
}

public class OrderRequest
{
    public string Symbol { get; set; } = string.Empty;
    public string Side { get; set; } = string.Empty;
    public string Type { get; set; } = string.Empty;
    public decimal Quantity { get; set; }
    public decimal? Price { get; set; }
    public string? PreferredExchange { get; set; }
}

public class OrderResponse
{
    public string OrderId { get; set; } = string.Empty;
    public string Status { get; set; } = string.Empty;
    public string Exchange { get; set; } = string.Empty;
    public string Message { get; set; } = string.Empty;
}