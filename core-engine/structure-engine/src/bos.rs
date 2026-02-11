use crate::{Candle, StructureEvent};
use std::collections::HashMap;
use serde_json::json;

pub fn detect_bos(symbol: &str, candles: &[Candle], _indicators: &HashMap<String, f64>) -> Option<StructureEvent> {
    if candles.len() < 20 {
        return None;
    }

    let latest = &candles[candles.len() - 1];
    // For BOS, check against more recent structure, say last 5 candles
    let recent_high = candles.iter().rev().skip(1).take(5).map(|c| c.high).fold(f64::NEG_INFINITY, f64::max);
    let recent_low = candles.iter().rev().skip(1).take(5).map(|c| c.low).fold(f64::INFINITY, f64::min);

    if latest.high > recent_high {
        Some(StructureEvent {
            symbol: symbol.to_string(),
            timestamp: latest.timestamp,
            event_type: "BOS_BULLISH".to_string(),
            details: json!({
                "broken_high": recent_high,
                "new_high": latest.high
            }),
        })
    } else if latest.low < recent_low {
        Some(StructureEvent {
            symbol: symbol.to_string(),
            timestamp: latest.timestamp,
            event_type: "BOS_BEARISH".to_string(),
            details: json!({
                "broken_low": recent_low,
                "new_low": latest.low
            }),
        })
    } else {
        None
    }
}