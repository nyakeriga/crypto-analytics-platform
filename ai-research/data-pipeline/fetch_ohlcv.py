import requests
import pandas as pd
import time
from datetime import datetime

def fetch_ohlcv(symbol, interval, start_date, end_date, output_file, format='csv'):
    """
    Fetch historical OHLCV data from Binance API and save to CSV or Parquet.

    :param symbol: Trading pair, e.g., 'BTCUSDT'
    :param interval: Kline interval, e.g., '1h'
    :param start_date: Start date in 'YYYY-MM-DD' format
    :param end_date: End date in 'YYYY-MM-DD' format
    :param output_file: Output file path without extension
    :param format: 'csv' or 'parquet'
    """
    base_url = 'https://api.binance.com/api/v3/klines'
    start_ts = int(pd.Timestamp(start_date).timestamp() * 1000)
    end_ts = int(pd.Timestamp(end_date).timestamp() * 1000)
    
    all_data = []
    current_start = start_ts
    
    while current_start < end_ts:
        params = {
            'symbol': symbol,
            'interval': interval,
            'startTime': current_start,
            'limit': 1000
        }
        response = requests.get(base_url, params=params)
        response.raise_for_status()
        data = response.json()
        
        if not data:
            break
        
        all_data.extend(data)
        current_start = data[-1][6] + 1  # close_time + 1
    
    # Convert to DataFrame
    df = pd.DataFrame(all_data, columns=[
        'timestamp', 'open', 'high', 'low', 'close', 'volume',
        'close_time', 'quote_asset_volume', 'number_of_trades',
        'taker_buy_base_asset_volume', 'taker_buy_quote_asset_volume', 'ignore'
    ])
    
    # Keep only OHLCV
    df = df[['timestamp', 'open', 'high', 'low', 'close', 'volume']]
    
    # Convert types
    df['timestamp'] = pd.to_datetime(df['timestamp'], unit='ms')
    for col in ['open', 'high', 'low', 'close', 'volume']:
        df[col] = df[col].astype(float)
    
    # Save
    if format == 'csv':
        df.to_csv(f"{output_file}.csv", index=False)
    elif format == 'parquet':
        df.to_parquet(f"{output_file}.parquet", index=False)
    else:
        raise ValueError("Format must be 'csv' or 'parquet'")

if __name__ == "__main__":
    # Example usage
    fetch_ohlcv('BTCUSDT', '1h', '2023-01-01', '2023-01-02', 'btc_usdt_1h', 'csv')