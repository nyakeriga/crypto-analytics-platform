import pandas as pd
import numpy as np

def normalize_data(df):
    """
    Clean, handle missing values, and normalize OHLCV data.

    :param df: DataFrame with columns ['timestamp', 'open', 'high', 'low', 'close', 'volume']
    :return: Cleaned and normalized DataFrame
    """
    # Sort by timestamp
    df = df.sort_values('timestamp').reset_index(drop=True)
    
    # Drop duplicates
    df = df.drop_duplicates(subset='timestamp')
    
    # Handle missing values
    # For prices, interpolate
    price_cols = ['open', 'high', 'low', 'close']
    df[price_cols] = df[price_cols].interpolate(method='linear')
    
    # For volume, fill with 0 if missing
    df['volume'] = df['volume'].fillna(0)
    
    # Drop any remaining NaN rows
    df = df.dropna()
    
    # Normalize prices using min-max scaling
    for col in price_cols:
        min_val = df[col].min()
        max_val = df[col].max()
        if max_val > min_val:
            df[f'{col}_norm'] = (df[col] - min_val) / (max_val - min_val)
        else:
            df[f'{col}_norm'] = 0.0
    
    # Normalize volume separately
    min_vol = df['volume'].min()
    max_vol = df['volume'].max()
    if max_vol > min_vol:
        df['volume_norm'] = (df['volume'] - min_vol) / (max_vol - min_vol)
    else:
        df['volume_norm'] = 0.0
    
    return df

if __name__ == "__main__":
    # Example usage
    # df = pd.read_csv('btc_usdt_1h.csv')
    # df = normalize_data(df)
    # df.to_csv('btc_usdt_1h_normalized.csv', index=False)
    pass