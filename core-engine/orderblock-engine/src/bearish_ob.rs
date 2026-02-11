use crate::{Candle, OrderBlockEvent};
use serde_json::json;

pub fn detect_bearish_ob(symbol: &str, candles: &[Candle]) -> Option<OrderBlockEvent> {
    if candles.len() < 10 {
        return None;
    }

    let latest = &candles[candles.len() - 1];
    let prev = &candles[candles.len() - 2];

    // Check if bearish candle
    if latest.close >= latest.open {
        return None;
    }

    // Check if in uptrend: last 5 candles mostly up
    let recent = &candles[candles.len().saturating_sub(6)..candles.len() - 1];
    let up_count = recent.iter().filter(|c| c.close > c.open).count();
    if up_count < 3 {
        return None;
    }

    // Check if large body: body > average of previous bodies
    let body = (latest.close - latest.open).abs();
    let prev_bodies: Vec<f64> = recent.iter().map(|c| (c.close - c.open).abs()).collect();
    let avg_body = prev_bodies.iter().sum::<f64>() / prev_bodies.len() as f64;
    if body < avg_body * 1.5 {
        return None;
    }

    // Bearish OB at the high of the candle
    Some(OrderBlockEvent {
        symbol: symbol.to_string(),
        timestamp: latest.timestamp,
        event_type: "bearish_order_block".to_string(),
        details: json!({
            "level": latest.high,
            "low": latest.low,
            "body": body
        }),
    })
}