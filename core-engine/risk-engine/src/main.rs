use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::Message;
use serde::{Deserialize, Serialize};

mod position_sizing;
mod sl_tp;
mod volatility_filter;

#[derive(Serialize, Deserialize, Debug)]
struct Signal {
    symbol: String,
    price: f64,
    direction: String,
    timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct RiskAdjustedSignal {
    symbol: String,
    price: f64,
    direction: String,
    position_size: f64,
    stop_loss: f64,
    take_profit: f64,
    risk_level: String,
    timestamp: u64,
}

#[tokio::main]
async fn main() {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "risk-engine")
        .set("bootstrap.servers", "localhost:9092")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&["signals"]).expect("Subscription failed");

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Producer creation failed");

    let mut message_stream = consumer.stream();

    while let Some(message) = message_stream.next().await {
        match message {
            Ok(msg) => {
                if let Some(payload) = msg.payload() {
                    if let Ok(signal) = serde_json::from_slice::<Signal>(payload) {
                        let risk_level = assess_risk_level(&signal);
                        if volatility_filter::filter(&signal) {
                            let position_size = position_sizing::calculate(&signal, &risk_level);
                            let (sl, tp) = sl_tp::adjust(&signal, &risk_level);
                            let adjusted = RiskAdjustedSignal {
                                symbol: signal.symbol,
                                price: signal.price,
                                direction: signal.direction,
                                position_size,
                                stop_loss: sl,
                                take_profit: tp,
                                risk_level,
                                timestamp: signal.timestamp,
                            };
                            let payload = serde_json::to_string(&adjusted).unwrap();
                            producer
                                .send(
                                    FutureRecord::to("risk-adjusted-signals")
                                        .payload(&payload)
                                        .key(&adjusted.symbol),
                                    std::time::Duration::from_secs(0),
                                )
                                .await
                                .unwrap();
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn assess_risk_level(signal: &Signal) -> String {
    if signal.price > 1000.0 {
        "high".to_string()
    } else if signal.price > 100.0 {
        "medium".to_string()
    } else {
        "low".to_string()
    }
}