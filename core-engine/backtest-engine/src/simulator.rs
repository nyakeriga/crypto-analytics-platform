use crate::{Signal, Ohlcv};
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Trade {
    pub entry_time: i64,
    pub exit_time: i64,
    pub entry_price: f64,
    pub exit_price: f64,
    pub pnl: f64,
    pub side: String, // "long" or "short"
}

pub fn run_backtest(signals: &[Signal], ohlcv: &[Ohlcv]) -> Vec<Trade> {
    let mut trades = Vec::new();
    let mut open_positions = VecDeque::new();

    // Sort signals by timestamp
    let mut sorted_signals = signals.to_vec();
    sorted_signals.sort_by_key(|s| s.timestamp);

    // Sort OHLCV by timestamp
    let mut sorted_ohlcv = ohlcv.to_vec();
    sorted_ohlcv.sort_by_key(|o| o.timestamp);

    let mut ohlcv_iter = sorted_ohlcv.iter().peekable();

    for signal in sorted_signals {
        // Advance OHLCV to current signal time
        while let Some(ohlc) = ohlcv_iter.peek() {
            if ohlc.timestamp < signal.timestamp {
                ohlcv_iter.next();
            } else {
                break;
            }
        }

        if let Some(current_ohlc) = ohlcv_iter.peek() {
            if signal.signal_type == "buy" {
                // Open long position
                open_positions.push_back((signal.timestamp, signal.price, "long"));
            } else if signal.signal_type == "sell" {
                // Open short position
                open_positions.push_back((signal.timestamp, signal.price, "short"));
            }
        }

        // Check for exits (simplified: exit after some time or opposite signal)
        // For simplicity, assume exit on next signal or after 10 bars
        if !open_positions.is_empty() {
            let (entry_time, entry_price, side) = open_positions.front().unwrap();
            let bars_since_entry = (signal.timestamp - entry_time) / 60000; // assume 1min bars
            if bars_since_entry > 10 || (side == "long" && signal.signal_type == "sell") || (side == "short" && signal.signal_type == "buy") {
                let exit_price = signal.price;
                let pnl = if *side == "long" {
                    exit_price - *entry_price
                } else {
                    *entry_price - exit_price
                };
                trades.push(Trade {
                    entry_time: *entry_time,
                    exit_time: signal.timestamp,
                    entry_price: *entry_price,
                    exit_price,
                    pnl,
                    side: side.clone(),
                });
                open_positions.pop_front();
            }
        }
    }

    // Close remaining positions at last price
    if let Some(last_ohlc) = sorted_ohlcv.last() {
        for (entry_time, entry_price, side) in open_positions {
            let pnl = if side == "long" {
                last_ohlc.close - entry_price
            } else {
                entry_price - last_ohlc.close
            };
            trades.push(Trade {
                entry_time,
                exit_time: last_ohlc.timestamp,
                entry_price,
                exit_price: last_ohlc.close,
                pnl,
                side,
            });
        }
    }

    trades
}

pub fn walk_forward_backtest(signals: &[Signal], ohlcv: &[Ohlcv], window_size: usize, step_size: usize) -> Vec<Trade> {
    let mut all_trades = Vec::new();

    let total_len = ohlcv.len();
    let mut start = 0;

    while start + window_size < total_len {
        let end = start + window_size;
        let test_signals: Vec<_> = signals.iter().filter(|s| s.timestamp >= ohlcv[start].timestamp && s.timestamp < ohlcv[end].timestamp).cloned().collect();
        let test_ohlcv = &ohlcv[start..end];

        let trades = run_backtest(&test_signals, test_ohlcv);
        all_trades.extend(trades);

        start += step_size;
    }

    all_trades
}