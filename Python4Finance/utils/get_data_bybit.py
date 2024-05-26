import pandas as pd
from pybit.unified_trading import HTTP

def get_data_with_volume_bybit(session, symbol, start_date, end_date, interval = "D", category = "linear", limit = 200):
    
    data = session.get_kline(
    category=category,
    symbol=symbol,
    interval= interval,
    start = start_date,
    end = end_date,
    limit= limit,
    )


    # Convert list to DataFrame
    columns = ['startTime', 'openPrice', 'highPrice', 'lowPrice', 'closePrice', 'volume', 'turnover']
    df = pd.DataFrame(data["result"]['list'], columns=columns)

    # Adding a new column for the symbol
    df['symbol'] = symbol

    # Convert startTime to datetime and prices to floats
    df['startTime'] = pd.to_datetime(df['startTime'].astype(float), unit='ms')
    for col in ['openPrice', 'highPrice', 'lowPrice', 'closePrice']:
        df[col] = df[col].astype(float)

    # Sort the DataFrame by startTime in descending order
    df = df.sort_values(by='startTime', ascending=False)
    
    print(df.head())
    
    return df


def get_data_OI_bybit(session, symbol, start_date, end_date, interval = "1d", category = "linear", limit = 200):
    
    data_OI = session.get_open_interest(
    category = category,
    symbol = symbol,
    intervalTime = interval,
    startTime = start_date,
    endTime = end_date,
    limit = limit,
    )


    # Convert list to DataFrame
    columns = ['openInterest', 'timestamp']
    df = pd.DataFrame(data_OI["result"]['list'], columns=columns)


    # Convert startTime to datetime and prices to floats
    df['startTime'] = pd.to_datetime(df['timestamp'].astype(float), unit='ms')

    # Sort the DataFrame by startTime in descending order
    df = df.sort_values(by='startTime', ascending=False)

    # Display the DataFrame
    print(df.head())
    return df