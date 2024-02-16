# Stock Market Monitor

### 1. Crates used and purpose

### 2. Financial analysis algorithm
Mention volatility formula (high - low) / close > 0.02
### 3. Charting Setup
2 charts are created by the program in png format as follows:
- chart 1 contains the daily closing price of the selected stock
- chart 2 contains the daily closing price and the volatility error bars to show the day's low, high and close.

### 4. Project setup
The program first calls the Yahoo api to collect all the data for the requested stock for the last 6 months.
Then the volatility data is calculated for each day's data based on the formula above. Using this data, 2 charts are created as a png. One contains only the daily closing price data and the other also contains the volatility data. 
Lastly, the backend is hosted on the local host using the specified port. The frontend can request either of the two plots and display it to the user. 

### 5. Usage intructions
`cargo run <STOCK_TICKER>` or `cargo run <STOCK_TICKER> <PORT_NUMBER>`\
Ex. `cargo run AAPL` or `cargo run TSLA 7000`

For help with the program\
`cargo run -- --help`

To view fancy plots in your browser, run the program and open the url http://127.0.0.1:4567 
(Or change to another valid port by specifying PORT_NUMBER in the launch command)
To stop the program use Contol+C.