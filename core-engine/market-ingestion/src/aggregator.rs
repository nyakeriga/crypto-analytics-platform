use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use tokio::sync::mpsc::Receiver;
use serde_json;

pub async fn run(mut rx: Receiver<serde_json::Value>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("message.timeout.ms", "5000")
        .create()?;

    while let Some(data) = rx.recv().await {
        let payload = data.to_string();
        let exchange = data.get("exchange").and_then(|e| e.as_str()).unwrap_or("unknown").to_string();

        let data_type = if exchange == "binance" {
            if data.get("stream").and_then(|s| s.as_str()).map_or(false, |s| s.contains("@trade")) {
                "trades"
            } else if data.get("stream").and_then(|s| s.as_str()).map_or(false, |s| s.contains("@depth")) {
                "orderbook"
            } else {
                "unknown"
            }
        } else if exchange == "bybit" {
            if data.get("topic").and_then(|t| t.as_str()) == Some("publicTrade") {
                "trades"
            } else if data.get("topic").and_then(|t| t.as_str()).map_or(false, |t| t.starts_with("orderbook")) {
                "orderbook"
            } else {
                "unknown"
            }
        } else if exchange == "okx" {
            if data.get("arg").and_then(|a| a.get("channel")).and_then(|c| c.as_str()) == Some("trades") {
                "trades"
            } else if data.get("arg").and_then(|a| a.get("channel")).and_then(|c| c.as_str()) == Some("books") {
                "orderbook"
            } else {
                "unknown"
            }
        } else {
            "unknown"
        };

        let topic = format!("market-{}", data_type);

        let record = FutureRecord::to(&topic)
            .payload(payload.as_bytes())
            .key(exchange.as_bytes());

        let _ = producer.send(record, tokio::time::Duration::from_secs(0)).await;
    }

    Ok(())
}