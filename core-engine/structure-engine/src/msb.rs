use crate::{Candle, StructureEvent};
use std::collections::HashMap;
use serde_json::json;

pub fn detect_msb(symbol: &str, candles: &[Candle], _indicators: &HashMap<String, f64>) -> Option<StructureEvent> {
    if candles.len() < 10 {
        return None;
    }

    let latest = &candles[candles.len() - 1];
    let prev_high = candles.iter().rev().skip(1).take(9).map(|c| c.high).fold(f64::NEG_INFINITY, f64::max);
    let prev_low = candles.iter().rev().skip(1).take(9).map(|c| c.low).fold(f64::INFINITY, f64::min);

    // Simple MSB: if high > prev_high or low < prev_low
    if latest.high > prev_high {
        Some(StructureEvent {
            symbol: symbol.to_string(),
            timestamp: latest.timestamp,
            event_type: "MSB_BULLISH".to_string(),
            details: json!({
                "broken_high": prev_high,
                "new_high": latest.high
            }),
        })
    } else if latest.low < prev_low {
        Some(StructureEvent {
            symbol: symbol.to_string(),
            timestamp: latest.timestamp,
            event_type: "MSB_BEARISH".to_string(),
            details: json!({
                "broken_low": prev_low,
                "new_low": latest.low
            }),
        })
    } else {
        None
    }
}