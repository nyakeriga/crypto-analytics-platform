import pandas as pd
import numpy as np

def calculate_rsi(series, period=14):
    """
    Calculate Relative Strength Index (RSI).
    """
    delta = series.diff()
    gain = (delta.where(delta > 0, 0)).rolling(window=period).mean()
    loss = (-delta.where(delta < 0, 0)).rolling(window=period).mean()
    rs = gain / loss
    rsi = 100 - (100 / (1 + rs))
    return rsi

def calculate_macd(series, fast_period=12, slow_period=26, signal_period=9):
    """
    Calculate MACD, Signal, and Histogram.
    """
    ema_fast = series.ewm(span=fast_period, adjust=False).mean()
    ema_slow = series.ewm(span=slow_period, adjust=False).mean()
    macd = ema_fast - ema_slow
    signal = macd.ewm(span=signal_period, adjust=False).mean()
    histogram = macd - signal
    return macd, signal, histogram

def build_features(df):
    """
    Build technical features like RSI and MACD from OHLCV data.

    :param df: DataFrame with 'close' column
    :return: DataFrame with added features
    """
    # RSI
    df['rsi'] = calculate_rsi(df['close'])
    
    # MACD
    df['macd'], df['macd_signal'], df['macd_histogram'] = calculate_macd(df['close'])
    
    return df

if __name__ == "__main__":
    # Example usage
    # df = pd.read_csv('btc_usdt_1h_normalized.csv')
    # df = build_features(df)
    # df.to_csv('btc_usdt_1h_features.csv', index=False)
    pass