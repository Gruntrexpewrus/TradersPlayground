import matplotlib.pyplot as plt
import matplotlib.dates as mdates
from mplfinance.original_flavor import candlestick_ohlc

def visualize_df_with_vol_oi(df, saving_path):
    """
    This function creates and saves a comprehensive financial chart consisting of a candlestick plot,
    volume bar chart, and open interest bar chart.

    Parameters:
    - df (DataFrame): A pandas DataFrame containing financial data with columns 'startTime', 'openPrice', 
                      'highPrice', 'lowPrice', 'closePrice', 'volume', 'openInterest', and 'symbol'.
    - saving_path (str): The file path where the resulting plot will be saved.

    Functionality:
    1. Data Preparation:
       - Converts the 'startTime' column to a numerical format suitable for plotting dates.
       - Creates separate DataFrames for OHLC (Open, High, Low, Close), volume, and open interest data.

    2. Subplot Creation:
       - Initializes a figure with three vertically aligned subplots sharing the same x-axis.
         The subplots are arranged in a 3:1:1 height ratio.

    3. Candlestick Chart:
       - Plots a candlestick chart on the first subplot using the OHLC data.
         The candlesticks are colored green for upward movements and red for downward movements.
       - Configures the x-axis to display dates formatted as 'YYYY-MM-DD' and labels the y-axis as 'Price'.

    4. Volume Bar Chart:
       - Plots a bar chart of the trading volume on the second subplot.
       - Sets the y-axis label to 'Volume'.

    5. Open Interest Bar Chart:
       - Plots a bar chart of the open interest on the third subplot.
       - Sets the y-axis label to 'Open Interest'.

    6. Shared X-Axis Formatting:
       - Rotates the x-axis tick labels by 45 degrees for better readability.
       - Adjusts the vertical space between subplots to ensure they are closely packed.

    7. Figure Title:
       - Sets a title for the entire figure, incorporating the stock symbol from the DataFrame.

    8. Saving and Displaying the Plot:
       - Saves the plot to the specified file path.
       - Displays the plot using plt.show().
    """
    # Prepare data for candlestick chart
    df['date'] = mdates.date2num(df['startTime'])
    ohlc = df[['date', 'openPrice', 'highPrice', 'lowPrice', 'closePrice']].copy()
    volume = df[['date', 'volume']].copy()
    open_interest = df[['date', 'openInterest']].copy()

    # Creating subplots with shared x-axis
    fig, (ax, ax2, ax3) = plt.subplots(3, 1, sharex=True, figsize=(10, 8), gridspec_kw={'height_ratios': [3, 1, 1]})

    # Plot candlestick chart
    candlestick_ohlc(ax, ohlc.values, width=0.5, colorup='green', colordown='red', alpha=1)
    ax.xaxis_date()
    ax.xaxis.set_major_formatter(mdates.DateFormatter('%Y-%m-%d'))
    ax.set_ylabel('Price')

    # Plot volume below candlestick chart
    ax2.bar(volume['date'], volume['volume'], color='blue', width=0.5)
    ax2.set_ylabel('Volume')

    # Plot open interest below volume chart
    ax3.bar(open_interest['date'], open_interest['openInterest'], color='gray', width=0.5)
    ax3.set_ylabel('Open Interest')

    # Formatting the shared x-axis
    plt.xticks(rotation=45)
    plt.subplots_adjust(hspace=0)  # Reduce space between plots

    # Set the title for the entire figure
    fig.suptitle(f"Candlestick Chart for {df['symbol'][0]}", fontsize=16)
    
    fig.savefig(saving_path)

    # Display the plot
    plt.show()