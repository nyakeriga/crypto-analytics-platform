use crate::Signal;

pub fn calculate(_signal: &Signal, risk_level: &str) -> f64 {
    let base_size = 1000.0;
    match risk_level {
        "low" => base_size * 1.0,
        "medium" => base_size * 0.5,
        "high" => base_size * 0.2,
        _ => base_size,
    }
}