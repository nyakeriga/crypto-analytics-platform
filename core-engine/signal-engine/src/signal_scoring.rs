use crate::{SymbolState, SignalLayers};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize)]
struct PredictionRequest {
    features: Vec<f64>,
    model_name: String,
}

#[derive(Deserialize)]
struct PredictionResponse {
    prediction: f64,
    model_version: String,
}

pub async fn calculate_layers(state: &SymbolState) -> Result<SignalLayers, Box<dyn Error + Send + Sync>> {
    let structure_score = calculate_structure_score(state);
    let ob_score = calculate_ob_score(state);
    let momentum_score = calculate_momentum_score(state);
    let volatility_score = calculate_volatility_score(state);
    let risk_score = calculate_risk_score(state);

    // Extract features for AI model
    let features = extract_features(state, &SignalLayers {
        structure: structure_score,
        ob: ob_score,
        momentum: momentum_score,
        volatility: volatility_score,
        risk: risk_score,
    });

    // Get AI confidence score
    let ai_confidence = get_ai_confidence(&features).await?;

    Ok(SignalLayers {
        structure: structure_score,
        ob: ob_score,
        momentum: momentum_score,
        volatility: volatility_score,
        risk: risk_score,
    })
}

fn calculate_structure_score(state: &SymbolState) -> f64 {
    // Simple: count recent structure events, normalize to 0-1
    let count = state.structure_events.len() as f64;
    (count / 10.0).min(1.0)
}

fn calculate_ob_score(state: &SymbolState) -> f64 {
    // Count orderblock events
    let count = state.orderblock_events.len() as f64;
    (count / 10.0).min(1.0)
}

fn calculate_momentum_score(state: &SymbolState) -> f64 {
    if let Some(latest) = state.indicators.last() {
        if let Some(rsi) = latest.rsi {
            // RSI > 70 overbought, <30 oversold
            if rsi > 70.0 {
                0.8
            } else if rsi < 30.0 {
                0.2
            } else {
                0.5
            }
        } else {
            0.5
        }
    } else {
        0.5
    }
}

fn calculate_volatility_score(state: &SymbolState) -> f64 {
    if let Some(latest) = state.indicators.last() {
        if let (Some(upper), Some(lower)) = (latest.bollinger_upper, latest.bollinger_lower) {
            let range = upper - lower;
            let close = latest.ohlcv.close;
            // High volatility if range is large relative to close
            let vol = range / close;
            (vol * 10.0).min(1.0)
        } else {
            0.5
        }
    } else {
        0.5
    }
}

fn calculate_risk_score(state: &SymbolState) -> f64 {
    // Simple: inverse of volatility or something
    1.0 - calculate_volatility_score(state)
}

fn extract_features(state: &SymbolState, layers: &SignalLayers) -> Vec<f64> {
    let mut features = Vec::new();

    // Add layer scores
    features.push(layers.structure);
    features.push(layers.ob);
    features.push(layers.momentum);
    features.push(layers.volatility);
    features.push(layers.risk);

    // Add recent indicator values
    if let Some(latest) = state.indicators.last() {
        features.push(latest.ohlcv.close);
        features.push(latest.ohlcv.volume);
        if let Some(rsi) = latest.rsi { features.push(rsi); } else { features.push(50.0); }
        if let Some(macd) = latest.macd { features.push(macd); } else { features.push(0.0); }
        if let Some(ema) = latest.ema { features.push(ema); } else { features.push(latest.ohlcv.close); }
        if let Some(vwap) = latest.vwap { features.push(vwap); } else { features.push(latest.ohlcv.close); }
        if let Some(upper) = latest.bollinger_upper { features.push(upper); } else { features.push(latest.ohlcv.close); }
        if let Some(lower) = latest.bollinger_lower { features.push(lower); } else { features.push(latest.ohlcv.close); }
    }

    // Add event counts
    features.push(state.structure_events.len() as f64);
    features.push(state.orderblock_events.len() as f64);

    features
}

async fn get_ai_confidence(features: &[f64]) -> Result<f64, Box<dyn Error + Send + Sync>> {
    let client = Client::new();
    let request = PredictionRequest {
        features: features.to_vec(),
        model_name: "signal_confidence".to_string(),
    };

    let response = client
        .post("http://localhost:8000/predict")
        .json(&request)
        .send()
        .await?;

    if response.status().is_success() {
        let prediction: PredictionResponse = response.json().await?;
        Ok(prediction.prediction.clamp(0.0, 1.0)) // Ensure between 0 and 1
    } else {
        // Fallback to traditional scoring if AI fails
        Ok(features.iter().sum::<f64>() / features.len() as f64)
    }
}