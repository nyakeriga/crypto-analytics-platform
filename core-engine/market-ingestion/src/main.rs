mod binance_ws;
mod bybit_ws;
mod okx_ws;
mod aggregator;

use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, rx) = mpsc::channel::<serde_json::Value>(1000);

    // Spawn WebSocket handlers for each exchange
    let binance_handle = tokio::spawn(binance_ws::run(tx.clone()));
    let bybit_handle = tokio::spawn(bybit_ws::run(tx.clone()));
    let okx_handle = tokio::spawn(okx_ws::run(tx.clone()));

    // Run aggregator
    let aggregator_handle = tokio::spawn(aggregator::run(rx));

    // Wait for all tasks
    let _ = tokio::try_join!(binance_handle, bybit_handle, okx_handle, aggregator_handle)?;

    Ok(())
}