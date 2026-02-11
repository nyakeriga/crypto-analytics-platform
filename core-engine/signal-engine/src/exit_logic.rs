use crate::SignalLayers;

pub fn calculate_exits(direction: &str, entry_price: f64, layers: &SignalLayers) -> (f64, f64) {
    let risk_factor = 1.0 - layers.risk; // higher risk, tighter stops
    let stop_distance = entry_price * 0.01 * (1.0 + risk_factor); // 1% base, adjusted
    let profit_distance = entry_price * 0.02 * (1.0 + layers.momentum); // 2% base, adjusted by momentum

    if direction == "buy" {
        let stop_loss = entry_price - stop_distance;
        let take_profit = entry_price + profit_distance;
        (stop_loss, take_profit)
    } else {
        let stop_loss = entry_price + stop_distance;
        let take_profit = entry_price - profit_distance;
        (stop_loss, take_profit)
    }
}