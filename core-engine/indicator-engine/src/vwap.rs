pub struct VWAP {
    sum_pv: f64,
    sum_v: f64,
}

impl VWAP {
    pub fn new() -> Self {
        VWAP {
            sum_pv: 0.0,
            sum_v: 0.0,
        }
    }

    pub fn next(&mut self, price: f64, volume: f64) -> f64 {
        self.sum_pv += price * volume;
        self.sum_v += volume;
        self.sum_pv / self.sum_v
    }
}