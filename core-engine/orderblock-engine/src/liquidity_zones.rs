use crate::{Candle, OrderBlockEvent};
use serde_json::json;

pub fn detect_liquidity_zones(symbol: &str, candles: &[Candle]) -> Option<Vec<OrderBlockEvent>> {
    if candles.len() < 20 {
        return None;
    }

    let recent = &candles[candles.len().saturating_sub(20)..];
    let support = recent.iter().map(|c| c.low).fold(f64::INFINITY, f64::min);
    let resistance = recent.iter().map(|c| c.high).fold(f64::NEG_INFINITY, f64::max);

    let latest = &candles[candles.len() - 1];

    // Publish support and resistance
    Some(vec![
        OrderBlockEvent {
            symbol: symbol.to_string(),
            timestamp: latest.timestamp,
            event_type: "support_zone".to_string(),
            details: json!({
                "level": support
            }),
        },
        OrderBlockEvent {
            symbol: symbol.to_string(),
            timestamp: latest.timestamp,
            event_type: "resistance_zone".to_string(),
            details: json!({
                "level": resistance
            }),
        },
    ])
}