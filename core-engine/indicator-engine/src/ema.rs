pub struct EMA {
    period: usize,
    value: Option<f64>,
    multiplier: f64,
}

impl EMA {
    pub fn new(period: usize) -> Self {
        EMA {
            period,
            value: None,
            multiplier: 2.0 / (period as f64 + 1.0),
        }
    }

    pub fn next(&mut self, close: f64) -> Option<f64> {
        if let Some(val) = self.value {
            self.value = Some((close * self.multiplier) + (val * (1.0 - self.multiplier)));
        } else {
            self.value = Some(close);
        }
        self.value
    }
}