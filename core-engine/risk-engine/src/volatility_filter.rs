use crate::Signal;

pub fn filter(signal: &Signal) -> bool {
    // Simple filter: reject if price > 500, assuming high volatility
    signal.price < 500.0
}