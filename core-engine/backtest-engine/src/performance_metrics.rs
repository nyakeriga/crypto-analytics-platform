use crate::simulator::Trade;
use statrs::statistics::{Data, Statistics};

#[derive(Debug)]
pub struct Metrics {
    pub sharpe_ratio: f64,
    pub win_rate: f64,
    pub total_return: f64,
    pub max_drawdown: f64,
    pub total_trades: usize,
}

pub fn calculate_metrics(trades: &[Trade]) -> Metrics {
    if trades.is_empty() {
        return Metrics {
            sharpe_ratio: 0.0,
            win_rate: 0.0,
            total_return: 0.0,
            max_drawdown: 0.0,
            total_trades: 0,
        };
    }

    let pnls: Vec<f64> = trades.iter().map(|t| t.pnl).collect();
    let data = Data::new(pnls.clone());

    // Sharpe ratio: mean / std, assuming risk-free rate 0
    let mean = data.mean().unwrap_or(0.0);
    let std = data.std_dev().unwrap_or(1.0);
    let sharpe_ratio = if std > 0.0 { mean / std } else { 0.0 };

    // Win rate
    let winning_trades = pnls.iter().filter(|&&p| p > 0.0).count();
    let win_rate = winning_trades as f64 / trades.len() as f64;

    // Total return
    let total_return = pnls.iter().sum();

    // Max drawdown
    let mut cumulative = 0.0;
    let mut peak = 0.0;
    let mut max_drawdown = 0.0;
    for &pnl in &pnls {
        cumulative += pnl;
        if cumulative > peak {
            peak = cumulative;
        }
        let drawdown = peak - cumulative;
        if drawdown > max_drawdown {
            max_drawdown = drawdown;
        }
    }

    Metrics {
        sharpe_ratio,
        win_rate,
        total_return,
        max_drawdown,
        total_trades: trades.len(),
    }
}