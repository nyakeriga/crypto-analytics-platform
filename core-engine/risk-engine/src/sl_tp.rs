use crate::Signal;

pub fn adjust(signal: &Signal, risk_level: &str) -> (f64, f64) {
    let sl_percent = match risk_level {
        "low" => 0.01,
        "medium" => 0.02,
        "high" => 0.05,
        _ => 0.02,
    };
    let tp_percent = match risk_level {
        "low" => 0.02,
        "medium" => 0.04,
        "high" => 0.10,
        _ => 0.04,
    };
    let sl = if signal.direction == "buy" {
        signal.price * (1.0 - sl_percent)
    } else {
        signal.price * (1.0 + sl_percent)
    };
    let tp = if signal.direction == "buy" {
        signal.price * (1.0 + tp_percent)
    } else {
        signal.price * (1.0 - tp_percent)
    };
    (sl, tp)
}