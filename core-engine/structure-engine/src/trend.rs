use crate::{Candle, StructureEvent};
use std::collections::HashMap;
use serde_json::json;

pub fn detect_trend_change(
    symbol: &str,
    candles: &[Candle],
    indicators: &HashMap<String, f64>,
    trends: &mut HashMap<String, String>,
) -> Option<StructureEvent> {
    if candles.len() < 2 {
        return None;
    }

    let latest = &candles[candles.len() - 1];
    let ema_20 = indicators.get("ema_20").copied().unwrap_or(latest.close);
    let current_trend = if latest.close > ema_20 { "bullish" } else { "bearish" };

    let previous_trend = trends.get(symbol).cloned().unwrap_or("neutral".to_string());

    if current_trend != previous_trend {
        trends.insert(symbol.to_string(), current_trend.to_string());
        Some(StructureEvent {
            symbol: symbol.to_string(),
            timestamp: latest.timestamp,
            event_type: "TREND_CHANGE".to_string(),
            details: json!({
                "from": previous_trend,
                "to": current_trend
            }),
        })
    } else {
        None
    }
}