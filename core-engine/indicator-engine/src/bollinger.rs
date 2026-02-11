pub struct Bollinger {
    period: usize,
    closes: Vec<f64>,
    sma: Option<f64>,
}

impl Bollinger {
    pub fn new(period: usize) -> Self {
        Bollinger {
            period,
            closes: Vec::with_capacity(period),
            sma: None,
        }
    }

    pub fn next(&mut self, close: f64) -> (Option<f64>, Option<f64>) {
        self.closes.push(close);
        if self.closes.len() > self.period {
            self.closes.remove(0);
        }
        if self.closes.len() == self.period {
            let sum: f64 = self.closes.iter().sum();
            self.sma = Some(sum / self.period as f64);
            let variance = self.closes.iter().map(|&c| (c - self.sma.unwrap()).powi(2)).sum::<f64>() / self.period as f64;
            let std = variance.sqrt();
            (Some(self.sma.unwrap() + 2.0 * std), Some(self.sma.unwrap() - 2.0 * std))
        } else {
            (None, None)
        }
    }
}