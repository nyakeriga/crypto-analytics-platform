mod multi_timeframe;
mod signal_scoring;
mod entry_logic;
mod exit_logic;

use std::collections::HashMap;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::producer::{FutureProducer, FutureRecord};
use serde::{Deserialize, Serialize};
use tokio::time::Duration;

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

#[derive(Serialize, Deserialize)]
struct StructureEvent {
    symbol: String,
    timestamp: i64,
    event_type: String,
    details: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
struct OrderBlockEvent {
    symbol: String,
    timestamp: i64,
    event_type: String,
    details: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
struct Signal {
    symbol: String,
    timestamp: i64,
    direction: String, // "buy" or "sell"
    confidence_score: f64,
    layers: SignalLayers,
    entry_price: f64,
    stop_loss: f64,
    take_profit: f64,
}

#[derive(Serialize, Deserialize)]
struct SignalLayers {
    structure: f64,
    ob: f64,
    momentum: f64,
    volatility: f64,
    risk: f64,
}

struct SymbolState {
    indicators: Vec<EnrichedOhlcv>,
    structure_events: Vec<StructureEvent>,
    orderblock_events: Vec<OrderBlockEvent>,
    multi_timeframe_data: multi_timeframe::MultiTimeframeData,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "signal-engine")
        .set("bootstrap.servers", "localhost:9092")
        .set("auto.offset.reset", "earliest")
        .create()?;

    consumer.subscribe(&["indicators", "structure-events", "orderblocks"])?;

    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()?;

    let mut states: HashMap<String, SymbolState> = HashMap::new();

    let mut message_stream = consumer.stream();

    while let Some(message) = message_stream.next().await {
        let message = message?;
        let payload = message.payload_view::<str>()?.unwrap();
        let topic = message.topic();

        match topic {
            "indicators" => {
                let enriched: EnrichedOhlcv = serde_json::from_str(payload)?;
                let symbol = enriched.ohlcv.symbol.clone();
                let state = states.entry(symbol.clone()).or_insert(SymbolState {
                    indicators: Vec::new(),
                    structure_events: Vec::new(),
                    orderblock_events: Vec::new(),
                    multi_timeframe_data: multi_timeframe::MultiTimeframeData::new(),
                });
                state.indicators.push(enriched.clone());
                if state.indicators.len() > 100 {
                    state.indicators.remove(0);
                }
                state.multi_timeframe_data.update(&enriched.ohlcv);
            }
            "structure-events" => {
                let event: StructureEvent = serde_json::from_str(payload)?;
                let symbol = event.symbol.clone();
                let state = states.entry(symbol).or_insert(SymbolState {
                    indicators: Vec::new(),
                    structure_events: Vec::new(),
                    orderblock_events: Vec::new(),
                    multi_timeframe_data: multi_timeframe::MultiTimeframeData::new(),
                });
                state.structure_events.push(event);
                if state.structure_events.len() > 50 {
                    state.structure_events.remove(0);
                }
            }
            "orderblocks" => {
                let event: OrderBlockEvent = serde_json::from_str(payload)?;
                let symbol = event.symbol.clone();
                let state = states.entry(symbol).or_insert(SymbolState {
                    indicators: Vec::new(),
                    structure_events: Vec::new(),
                    orderblock_events: Vec::new(),
                    multi_timeframe_data: multi_timeframe::MultiTimeframeData::new(),
                });
                state.orderblock_events.push(event);
                if state.orderblock_events.len() > 50 {
                    state.orderblock_events.remove(0);
                }
            }
            _ => {}
        }

        // After processing, check if we can generate a signal
        for (symbol, state) in &states {
            if !state.indicators.is_empty() {
                match generate_signal(symbol, state).await {
                    Some(signal) => {
                        println!("Generated signal for {}: {} confidence {:.2}", symbol, signal.direction, signal.confidence_score);
                        let payload = serde_json::to_string(&signal)?;
                        let record = FutureRecord::to("signals")
                            .payload(&payload)
                            .key(symbol);
                        match producer.send(record, Duration::from_secs(0)).await {
                            Ok(_) => println!("Signal sent to Kafka for {}", symbol),
                            Err(e) => eprintln!("Failed to send signal to Kafka for {}: {}", symbol, e),
                        }
                    }
                    None => {
                        // Low confidence signals are normal, don't log
                    }
                }
            }
        }
    }

    Ok(())
}

async fn generate_signal(symbol: &str, state: &SymbolState) -> Option<Signal> {
    let latest_indicator = state.indicators.last()?;

    // Try to get AI-enhanced layers, fallback to traditional if AI fails
    let layers = match signal_scoring::calculate_layers(state).await {
        Ok(layers) => layers,
        Err(e) => {
            eprintln!("AI scoring failed: {}, using traditional scoring", e);
            let structure_score = (state.structure_events.len() as f64 / 10.0).min(1.0);
            let ob_score = (state.orderblock_events.len() as f64 / 10.0).min(1.0);
            let momentum_score = if let Some(rsi) = latest_indicator.rsi {
                if rsi > 70.0 { 0.8 } else if rsi < 30.0 { 0.2 } else { 0.5 }
            } else { 0.5 };
            let volatility_score = if let (Some(upper), Some(lower)) = (latest_indicator.bollinger_upper, latest_indicator.bollinger_lower) {
                let range = upper - lower;
                let close = latest_indicator.ohlcv.close;
                (range / close * 10.0).min(1.0)
            } else { 0.5 };
            let risk_score = 1.0 - volatility_score;

            SignalLayers {
                structure: structure_score,
                ob: ob_score,
                momentum: momentum_score,
                volatility: volatility_score,
                risk: risk_score,
            }
        }
    };

    let total_score = layers.structure + layers.ob + layers.momentum + layers.volatility + layers.risk;
    let confidence = total_score / 5.0; // average

    if confidence > 0.6 { // threshold
        let direction = if layers.momentum > 0.5 { "buy" } else { "sell" };
        let entry_price = entry_logic::determine_entry_price(state, direction, &layers);
        let (stop_loss, take_profit) = exit_logic::calculate_exits(direction, entry_price, &layers);

        Some(Signal {
            symbol: symbol.to_string(),
            timestamp: latest_indicator.ohlcv.timestamp,
            direction: direction.to_string(),
            confidence_score: confidence,
            layers,
            entry_price,
            stop_loss,
            take_profit,
        })
    } else {
        None
    }
}