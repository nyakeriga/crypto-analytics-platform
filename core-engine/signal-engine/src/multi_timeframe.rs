use crate::Ohlcv;
use std::collections::HashMap;

pub struct MultiTimeframeData {
    pub timeframe_5m: Vec<Ohlcv>,
    pub timeframe_15m: Vec<Ohlcv>,
    pub timeframe_1h: Vec<Ohlcv>,
}

impl MultiTimeframeData {
    pub fn new() -> Self {
        MultiTimeframeData {
            timeframe_5m: Vec::new(),
            timeframe_15m: Vec::new(),
            timeframe_1h: Vec::new(),
        }
    }

    pub fn update(&mut self, ohlcv: &Ohlcv) {
        // Assume input is 1m candles
        self.update_timeframe(&mut self.timeframe_5m, ohlcv, 5 * 60 * 1000); // 5 minutes in ms
        self.update_timeframe(&mut self.timeframe_15m, ohlcv, 15 * 60 * 1000);
        self.update_timeframe(&mut self.timeframe_1h, ohlcv, 60 * 60 * 1000);
    }

    fn update_timeframe(&mut self, timeframe: &mut Vec<Ohlcv>, ohlcv: &Ohlcv, interval_ms: i64) {
        let bucket = (ohlcv.timestamp / interval_ms) * interval_ms;
        if let Some(last) = timeframe.last_mut() {
            if last.timestamp == bucket {
                // Update existing
                last.high = last.high.max(ohlcv.high);
                last.low = last.low.min(ohlcv.low);
                last.close = ohlcv.close;
                last.volume += ohlcv.volume;
            } else {
                // New bucket
                timeframe.push(Ohlcv {
                    symbol: ohlcv.symbol.clone(),
                    timestamp: bucket,
                    open: ohlcv.open,
                    high: ohlcv.high,
                    low: ohlcv.low,
                    close: ohlcv.close,
                    volume: ohlcv.volume,
                });
            }
        } else {
            timeframe.push(Ohlcv {
                symbol: ohlcv.symbol.clone(),
                timestamp: bucket,
                open: ohlcv.open,
                high: ohlcv.high,
                low: ohlcv.low,
                close: ohlcv.close,
                volume: ohlcv.volume,
            });
        }
        // Keep only last 100
        if timeframe.len() > 100 {
            timeframe.remove(0);
        }
    }
}