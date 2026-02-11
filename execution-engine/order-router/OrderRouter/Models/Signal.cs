namespace OrderRouter.Models;

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