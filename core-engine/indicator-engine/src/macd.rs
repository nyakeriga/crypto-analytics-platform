pub struct MACD {
    fast_ema: crate::ema::EMA,
    slow_ema: crate::ema::EMA,
    signal_ema: crate::ema::EMA,
    macd: Option<f64>,
}

impl MACD {
    pub fn new(fast_period: usize, slow_period: usize, signal_period: usize) -> Self {
        MACD {
            fast_ema: crate::ema::EMA::new(fast_period),
            slow_ema: crate::ema::EMA::new(slow_period),
            signal_ema: crate::ema::EMA::new(signal_period),
            macd: None,
        }
    }

    pub fn next(&mut self, close: f64) -> Option<f64> {
        let fast = self.fast_ema.next(close)?;
        let slow = self.slow_ema.next(close)?;
        self.macd = Some(fast - slow);
        self.signal_ema.next(self.macd?);
        self.macd
    }
}