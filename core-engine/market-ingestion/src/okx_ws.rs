use tokio::sync::mpsc::Sender;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures::prelude::*;
use serde_json;

pub async fn run(tx: Sender<serde_json::Value>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = "wss://wsaws.okx.com:8443/ws/v5/public";

    loop {
        let (ws_stream, _) = connect_async(url).await?;
        let (mut write, mut read) = ws_stream.split();

        // Subscribe to trade and orderbook streams
        let subscribe_msg = serde_json::json!({
            "op": "subscribe",
            "args": [
                {"channel": "trades", "instId": "BTC-USDT"},
                {"channel": "books", "instId": "BTC-USDT"}
            ]
        });
        write.send(Message::Text(subscribe_msg.to_string())).await?;

        while let Some(message) = read.next().await {
            let message = message?;
            if let Message::Text(text) = message {
                if let Ok(mut data) = serde_json::from_str::<serde_json::Value>(&text) {
                    if data.get("arg").is_some() && data.get("data").is_some() { // It's a data message
                        data["exchange"] = serde_json::Value::String("okx".to_string());
                        let _ = tx.send(data).await;
                    }
                }
            }
        }

        // Reconnect on disconnect
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}