mod simulator;
mod performance_metrics;

use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::Message;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::mpsc;
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize)]
struct Signal {
    symbol: String,
    timestamp: i64,
    signal_type: String, // e.g., "buy", "sell"
    price: f64,
    // other fields
}

#[derive(Debug, Serialize, Deserialize)]
struct Ohlcv {
    symbol: String,
    timestamp: i64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct BacktestResult {
    symbol: String,
    period_start: i64,
    period_end: i64,
    sharpe_ratio: f64,
    win_rate: f64,
    total_return: f64,
    // other metrics
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Database connection
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;

    // Kafka consumer for signals
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "backtest-engine")
        .set("bootstrap.servers", "localhost:9092")
        .set("auto.offset.reset", "earliest")
        .create()?;

    consumer.subscribe(&["risk-adjusted-signals"])?;

    // Kafka producer for results
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()?;

    // Channel for OHLCV data (assuming another consumer or from DB)
    let (tx, mut rx) = mpsc::channel(100);

    // For simplicity, assume OHLCV is consumed from another topic or DB
    // Here, we'll simulate or fetch from DB

    let mut signals = HashMap::new();
    let mut ohlcv_data = HashMap::new();

    loop {
        match consumer.recv().await {
            Ok(message) => {
                let payload = message.payload().unwrap();
                let signal: Signal = serde_json::from_slice(payload)?;

                // Collect signals
                signals.entry(signal.symbol.clone()).or_insert_with(Vec::new).push(signal);

                // Fetch OHLCV data for the symbol (simplified)
                // In real, query DB or consume from topic
                let ohlcv = fetch_ohlcv(&pool, &signal.symbol).await?;
                ohlcv_data.insert(signal.symbol.clone(), ohlcv);

                // Run backtest
                if let Some(signals) = signals.get(&signal.symbol) {
                    if let Some(ohlcv) = ohlcv_data.get(&signal.symbol) {
                        let trades = simulator::walk_forward_backtest(signals, ohlcv, 1000, 500); // window 1000 bars, step 500
                        let metrics = performance_metrics::calculate_metrics(&trades);

                        let result = BacktestResult {
                            symbol: signal.symbol.clone(),
                            period_start: 0, // TODO
                            period_end: 0,
                            sharpe_ratio: metrics.sharpe_ratio,
                            win_rate: metrics.win_rate,
                            total_return: metrics.total_return,
                        };

                        // Publish to Kafka
                        let payload = serde_json::to_string(&result)?;
                        producer.send(
                            FutureRecord::to("backtest-results")
                                .payload(&payload)
                                .key(&signal.symbol),
                            std::time::Duration::from_secs(0),
                        ).await?;

                        // Store in DB
                        store_result(&pool, &result).await?;
                    }
                }
            }
            Err(e) => eprintln!("Kafka error: {}", e),
        }
    }
}

async fn fetch_ohlcv(pool: &PgPool, symbol: &str) -> Result<Vec<Ohlcv>, sqlx::Error> {
    // Simplified query
    let rows = sqlx::query!("SELECT * FROM ohlcv WHERE symbol = $1 ORDER BY timestamp", symbol)
        .fetch_all(pool)
        .await?;

    let ohlcv = rows.into_iter().map(|row| Ohlcv {
        symbol: row.symbol,
        timestamp: row.timestamp,
        open: row.open,
        high: row.high,
        low: row.low,
        close: row.close,
        volume: row.volume,
    }).collect();

    Ok(ohlcv)
}

async fn store_result(pool: &PgPool, result: &BacktestResult) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO backtest_results (symbol, period_start, period_end, sharpe_ratio, win_rate, total_return) VALUES ($1, $2, $3, $4, $5, $6)",
        result.symbol,
        result.period_start,
        result.period_end,
        result.sharpe_ratio,
        result.win_rate,
        result.total_return
    )
    .execute(pool)
    .await?;

    Ok(())
}