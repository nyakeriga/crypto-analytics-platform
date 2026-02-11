import pandas as pd
import numpy as np

def calculate_volume_imbalance(df):
    """
    Calculate volume imbalance.

    Assumes df has 'buy_volume' and 'sell_volume' columns.

    Parameters:
    df (pd.DataFrame): DataFrame with 'buy_volume', 'sell_volume' columns.

    Returns:
    pd.DataFrame: DataFrame with added 'volume_imbalance' column.
    """
    total_volume = df['buy_volume'] + df['sell_volume']
    df['volume_imbalance'] = (df['buy_volume'] - df['sell_volume']) / total_volume
    return df

def calculate_delta(df):
    """
    Calculate order flow delta.

    Assumes df has 'buy_volume' and 'sell_volume' columns.

    Parameters:
    df (pd.DataFrame): DataFrame with 'buy_volume', 'sell_volume' columns.

    Returns:
    pd.DataFrame: DataFrame with added 'delta' column.
    """
    df['delta'] = df['buy_volume'] - df['sell_volume']
    return df