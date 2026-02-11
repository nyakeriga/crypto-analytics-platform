use crate::{SymbolState, SignalLayers};

pub fn determine_entry_price(state: &SymbolState, direction: &str, layers: &SignalLayers) -> f64 {
    let latest = state.indicators.last().unwrap().ohlcv.close;
    // Simple: use close, but adjust based on OB
    if direction == "buy" && layers.ob > 0.7 {
        latest * 0.995 // slight discount for OB
    } else if direction == "sell" && layers.ob > 0.7 {
        latest * 1.005
    } else {
        latest
    }
}