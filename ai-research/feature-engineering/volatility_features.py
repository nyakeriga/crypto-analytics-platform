import pandas as pd
import numpy as np
from ta.volatility import AverageTrueRange, BollingerBands

def calculate_atr(df, period=14):
    """
    Calculate Average True Range (ATR).

    Parameters:
    df (pd.DataFrame): DataFrame with 'high', 'low', 'close' columns.
    period (int): Period for ATR calculation.

    Returns:
    pd.DataFrame: DataFrame with added 'atr' column.
    """
    atr_indicator = AverageTrueRange(high=df['high'], low=df['low'], close=df['close'], window=period)
    df['atr'] = atr_indicator.average_true_range()
    return df

def calculate_bollinger_volatility(df, period=20, std_dev=2):
    """
    Calculate Bollinger Band volatility (standard deviation).

    Parameters:
    df (pd.DataFrame): DataFrame with 'close' column.
    period (int): Period for rolling calculation.
    std_dev (int): Standard deviation multiplier (not used in volatility calc, but for bands).

    Returns:
    pd.DataFrame: DataFrame with added 'bb_volatility' column.
    """
    bb_indicator = BollingerBands(close=df['close'], window=period, window_dev=std_dev)
    df['bb_volatility'] = bb_indicator.bollinger_mavg()  # Actually, volatility is the std, but ta doesn't expose it directly
    # BollingerBands doesn't directly give std, but we can calculate it
    df['bb_volatility'] = df['close'].rolling(window=period).std()
    return df