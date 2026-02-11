use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::Message;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc};

mod bullish_ob;
mod bearish_ob;
mod liquidity_zones;

#[derive(Deserialize, Clone)]
struct Candle {
    timestamp: i64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}

#[derive(Deserialize)]
struct IndicatorMessage {
    symbol: String,
    candle: Candle,
    indicators: HashMap<String, f64>,
}

#[derive(Serialize)]
struct OrderBlockEvent {
    symbol: String,
    timestamp: i64,
    event_type: String,
    details: serde_json::Value,
}

type SymbolHistory = Arc<Mutex<HashMap<String, Vec<Candle>>>>;

#[tokio::main]
async fn main() {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "orderblock-engine")
        .set("bootstrap.servers", "localhost:9092")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&["indicators"]).expect("Subscription failed");

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Producer creation failed");

    let history: SymbolHistory = Arc::new(Mutex::new(HashMap::new()));

    loop {
        match consumer.recv().await {
            Ok(message) => {
                if let Some(payload) = message.payload() {
                    if let Ok(indicator_msg) = serde_json::from_slice::<IndicatorMessage>(payload) {
                        let mut hist = history.lock().await;
                        let candles = hist.entry(indicator_msg.symbol.clone()).or_insert(Vec::new());
                        candles.push(indicator_msg.candle.clone());
                        if candles.len() > 200 {  // Keep more history for OB detection
                            candles.remove(0);
                        }

                        // Detect order blocks and liquidity zones
                        if let Some(event) = bullish_ob::detect_bullish_ob(&indicator_msg.symbol, &candles) {
                            publish_event(&producer, event).await;
                        }
                        if let Some(event) = bearish_ob::detect_bearish_ob(&indicator_msg.symbol, &candles) {
                            publish_event(&producer, event).await;
                        }
                        if let Some(events) = liquidity_zones::detect_liquidity_zones(&indicator_msg.symbol, &candles) {
                            for event in events {
                                publish_event(&producer, event).await;
                            }
                        }
                    }
                }
            }
            Err(e) => eprintln!("Kafka error: {}", e),
        }
    }
}

async fn publish_event(producer: &FutureProducer, event: OrderBlockEvent) {
    let payload = serde_json::to_string(&event).unwrap();
    let record = FutureRecord::to("orderblocks")
        .payload(&payload)
        .key(&event.symbol);
    if let Err(e) = producer.send(record, std::time::Duration::from_secs(0)).await {
        eprintln!("Publish error: {}", e);
    }
}