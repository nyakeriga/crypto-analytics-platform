import pandas as pd
import numpy as np
from ta.trend import ADXIndicator

def calculate_trend_strength(df, period=14):
    """
    Calculate trend strength using ADX.

    Parameters:
    df (pd.DataFrame): DataFrame with 'high', 'low', 'close' columns.
    period (int): Period for ADX calculation.

    Returns:
    pd.DataFrame: DataFrame with added 'trend_strength' column.
    """
    adx_indicator = ADXIndicator(high=df['high'], low=df['low'], close=df['close'], window=period)
    df['trend_strength'] = adx_indicator.adx()
    return df

def calculate_msb_count(df, window=20):
    """
    Calculate MSB (Market Structure Break) count in a rolling window.
    MSB is considered when close breaks the previous high (bullish) or low (bearish).

    Parameters:
    df (pd.DataFrame): DataFrame with 'high', 'low', 'close' columns.
    window (int): Rolling window to count MSBs.

    Returns:
    pd.DataFrame: DataFrame with added 'msb_count' column.
    """
    # Bullish MSB: close > shift(high)
    bullish_msb = (df['close'] > df['high'].shift(1)).astype(int)
    # Bearish MSB: close < shift(low)
    bearish_msb = (df['close'] < df['low'].shift(1)).astype(int)
    df['msb_count'] = (bullish_msb + bearish_msb).rolling(window=window).sum()
    return df

def calculate_ob_count(df, window=20, volume_threshold=1.5):
    """
    Calculate OB (Order Block) count in a rolling window.
    OB is considered when volume is above threshold times average volume.

    Parameters:
    df (pd.DataFrame): DataFrame with 'volume' column.
    window (int): Rolling window for average volume.
    volume_threshold (float): Multiplier for volume threshold.

    Returns:
    pd.DataFrame: DataFrame with added 'ob_count' column.
    """
    avg_volume = df['volume'].rolling(window=window).mean()
    ob = (df['volume'] > avg_volume * volume_threshold).astype(int)
    df['ob_count'] = ob.rolling(window=window).sum()
    return df