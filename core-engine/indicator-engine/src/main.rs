mod rsi;
mod macd;
mod ema;
mod vwap;
mod bollinger;

use std::collections::HashMap;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::Message;
use serde::{Deserialize, Serialize};
use tokio::time::Duration;
use futures::StreamExt;

#[derive(Serialize, Deserialize, Clone)]
struct Ohlcv {
    symbol: String,
    timestamp: i64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

#[derive(Serialize, Deserialize)]
struct EnrichedOhlcv {
    #[serde(flatten)]
    ohlcv: Ohlcv,
    rsi: Option<f64>,
    macd: Option<f64>,
    ema: Option<f64>,
    vwap: Option<f64>,
    bollinger_upper: Option<f64>,
    bollinger_lower: Option<f64>,
}

struct IndicatorState {
    rsi: rsi::RSI,
    macd: macd::MACD,
    ema: ema::EMA,
    vwap: vwap::VWAP,
    bollinger: bollinger::Bollinger,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "indicator-engine")
        .set("bootstrap.servers", "localhost:9092")
        .set("auto.offset.reset", "earliest")
        .create()?;

    consumer.subscribe(&["market-ohlcv"])?;

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()?;

    let mut states: HashMap<String, IndicatorState> = HashMap::new();

    let mut message_stream = consumer.stream();

    while let Some(message) = message_stream.next().await {
        let message = message?;
        let payload = message.payload().ok_or("No payload")?;
        let payload_str = std::str::from_utf8(payload)?;
        let ohlcv: Ohlcv = serde_json::from_str(payload_str)?;

        let symbol = ohlcv.symbol.clone();
        let state = states.entry(symbol.clone()).or_insert(IndicatorState {
            rsi: rsi::RSI::new(14),
            macd: macd::MACD::new(12, 26, 9),
            ema: ema::EMA::new(20),
            vwap: vwap::VWAP::new(),
            bollinger: bollinger::Bollinger::new(20),
        });

        let rsi_val = state.rsi.next(ohlcv.close);
        let macd_val = state.macd.next(ohlcv.close);
        let ema_val = state.ema.next(ohlcv.close);
        let vwap_val = Some(state.vwap.next(ohlcv.close, ohlcv.volume));
        let (bollinger_upper, bollinger_lower) = state.bollinger.next(ohlcv.close);

        let enriched = EnrichedOhlcv {
            ohlcv,
            rsi: rsi_val,
            macd: macd_val,
            ema: ema_val,
            vwap: vwap_val,
            bollinger_upper,
            bollinger_lower,
        };

        let payload = serde_json::to_string(&enriched)?;
        let record = FutureRecord::to("indicators")
            .payload(payload.as_bytes())
            .key(symbol.as_bytes());

        let _ = producer.send(record, Duration::from_secs(0)).await;
    }

    Ok(())
}