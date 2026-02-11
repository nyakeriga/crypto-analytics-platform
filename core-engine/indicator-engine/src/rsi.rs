pub struct RSI {
    period: usize,
    avg_gain: f64,
    avg_loss: f64,
    prev_close: Option<f64>,
    count: usize,
}

impl RSI {
    pub fn new(period: usize) -> Self {
        RSI {
            period,
            avg_gain: 0.0,
            avg_loss: 0.0,
            prev_close: None,
            count: 0,
        }
    }

    pub fn next(&mut self, close: f64) -> Option<f64> {
        if let Some(prev) = self.prev_close {
            let change = close - prev;
            let gain = if change > 0.0 { change } else { 0.0 };
            let loss = if change < 0.0 { -change } else { 0.0 };
            if self.count < self.period {
                self.avg_gain += gain;
                self.avg_loss += loss;
                self.count += 1;
                if self.count == self.period {
                    self.avg_gain /= self.period as f64;
                    self.avg_loss /= self.period as f64;
                    if self.avg_loss == 0.0 {
                        return Some(100.0);
                    }
                    let rs = self.avg_gain / self.avg_loss;
                    return Some(100.0 - (100.0 / (1.0 + rs)));
                }
            } else {
                self.avg_gain = (self.avg_gain * (self.period - 1) as f64 + gain) / self.period as f64;
                self.avg_loss = (self.avg_loss * (self.period - 1) as f64 + loss) / self.period as f64;
                if self.avg_loss == 0.0 {
                    return Some(100.0);
                }
                let rs = self.avg_gain / self.avg_loss;
                return Some(100.0 - (100.0 / (1.0 + rs)));
            }
        }
        self.prev_close = Some(close);
        None
    }
}