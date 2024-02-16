# Stock Market Monitor

### 1. Crates Used and Purpose

- std: Process management, I/O operations, and TCP communication
- time: Used for dealing with time intervals and formatting
- yahoo_finance_api: Fetching stock quotes based on stock ticker
- plotters: Creating the plots
- chrono: Used for converting between different date representations
- clap: Parsing command line arguments

### 2. Financial Analysis Algorithm

We determined which days were volatile using this formula:\
$|\frac{high-low}{close}| > 0.02$\
This means that their total price varied by more than 2%

### 3. Charting Setup

The `plotters` was used to generate two line charts by the program in png format as follows:

- `stock_prices.png` contains the daily closing price of the selected stock
- `volatile_stock_prices.png` contains the daily closing price and the volatility error bars to show the day's low, high and close.

Both line charts feature axes dependant on the specific stocks maximum and minumum date and price.

### 4. Project Setup

The program first calls the Yahoo API to collect all the data for the requested stock for the last 6 months.
Then the volatility data is calculated for each day's data based on the formula above. Using this data, 2 charts are created as a png. One contains only the daily closing price data and the other also contains the volatility data.
Lastly, the backend is hosted on the local host using the specified port. The frontend can request either of the two plots and display it to the user.

### 5. Usage Intructions

Debug:

- `cargo run <STOCK_TICKER>`
- `cargo run <STOCK_TICKER> <PORT_NUMBER>`

Release:

- `cargo run <STOCK_TICKER>`
- `cargo run <STOCK_TICKER> <PORT_NUMBER>`

Examples:

- `cargo run AAPL`
- `cargo run --release TSLA 7000`

For help with the program:

- `cargo run -- --help`

Viewing in browser:

- To view the plots in your browser, run the program and open the URL http://127.0.0.1:4567
  - (Or change to another valid port by specifying PORT_NUMBER in the launch command)\
- To stop the program use Contol+C.
