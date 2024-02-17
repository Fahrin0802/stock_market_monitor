# Stock Market Monitor v0.1.0

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

- Download stock_market_monitor v0.1.0
- Navigate to the inner folder `stock_market_monitor/stock_market_monitor/`
- Build the program with `cargo build --release`
- Follow the usage instructions below

### 5. Usage Intructions

Release:

- `cargo run --release <STOCK_TICKER>`
- `cargo run --release <STOCK_TICKER> <PORT_NUMBER>`

Examples:

- `cargo run --release AAPL`
- `cargo run --release TSLA 7000`

For help with the program:

- `cargo run --release --help`

Viewing in browser:

- To view the plots in your browser, run the program and open the URL http://127.0.0.1:4567
  - (Or change to another valid port by specifying PORT_NUMBER in the launch command)\
- To stop the program use Contol+C.
