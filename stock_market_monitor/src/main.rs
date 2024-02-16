use std::process;
use time::{Duration, OffsetDateTime};
use yahoo::YahooConnector;
use yahoo_finance_api as yahoo;
use yahoo_finance_api::Quote;

use plotters::drawing::IntoDrawingArea;
use plotters::prelude::*;
use plotters::style::RGBColor;

use chrono::{Datelike, NaiveDate, TimeZone, Utc};

use clap::Parser;

use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

fn get_stock_prices(
    stock_name: &str,
    end_date: OffsetDateTime,
    start_date: OffsetDateTime,
    provider: &YahooConnector,
) -> Vec<Quote> {
    // returns historic quotes with daily interval
    let resp =
        tokio_test::block_on(provider.get_quote_history(stock_name, start_date, end_date)).unwrap();
    return resp.quotes().unwrap();
}

fn plot_prices(
    image_name: &str,
    min_price: f64,
    max_price: f64,
    min_date: NaiveDate,
    max_date: NaiveDate,
    series: Vec<(NaiveDate, f64)>,
    volatile_days: Vec<(NaiveDate, Quote)>,
    stock_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a drawing area
    let root = BitMapBackend::new(image_name, (800, 600)).into_drawing_area();
    root.fill(&RGBColor(255, 255, 255))?;

    // Configure a line chart
    let mut chart = ChartBuilder::on(&root)
        .caption(
            stock_name.to_owned() + " Stock Prices",
            ("Arial", 30).into_font(),
        )
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(min_date..max_date, min_price..max_price)?;

    // Draw the line series
    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(
        series.iter().map(|(x, y)| (*x, *y)),
        &RGBColor(255, 0, 0),
    ))?;

    // Draw the volatility data
    chart
        .draw_series(
            volatile_days.iter().map(|(x, y)| {
                ErrorBar::new_vertical(*x, y.low, y.close, y.high, BLUE.filled(), 10)
            }),
        )
        .unwrap();

    Ok(())
}

fn is_valid_stock(stock_name: &str, provider: &YahooConnector) -> bool {
    match tokio_test::block_on(provider.get_latest_quotes(stock_name, "1d")) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Generate plots from inputted stock names
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the stock ticker, ex. AAPL
    stock_name: String,
}

fn main() {
    let args = Args::parse();
    let stock_name: &str = &args.stock_name;

    let provider = yahoo::YahooConnector::new();

    // Check if stock symbol is valid
    if !is_valid_stock(stock_name, &provider) {
        eprintln!("Error: The stock symbol {} is not valid.", stock_name);
        process::exit(1);
    }

    // Get today's date and six months prior date
    let today = OffsetDateTime::now_utc();
    let six_months_ago = today - Duration::days(30 * 6);

    // Get daily stock quotes from yahoo finance
    let quotes: Vec<Quote> = get_stock_prices(stock_name, today, six_months_ago, &provider);

    // Convert date format to Naive Date
    let min_date: NaiveDate = match NaiveDate::from_ymd_opt(
        six_months_ago.year(),
        six_months_ago.month() as u32,
        six_months_ago.day().into(),
    ) {
        Some(date) => date,
        None => {
            panic!("Invalid date");
        }
    };
    let max_date: NaiveDate = match NaiveDate::from_ymd_opt(
        today.year(), 
        today.month() as u32, 
        today.day().into()) {
            Some(date) => date,
            None => {
                panic!("Invalid date");
            }
        };

    //This iterator is created to help derive the daily data, volatile days and min/max close
    let date_quote_pairs  = quotes.iter().map(|quote| {
        let datetime_utc = Utc.timestamp_opt(quote.timestamp as i64, 0).unwrap();
        let quote_date: NaiveDate = match NaiveDate::from_ymd_opt(
            datetime_utc.year(),
            datetime_utc.month() as u32,
            datetime_utc.day().into(),
        ) {
            Some(date) => date,
            None => {
                panic!("Invalid date");
            }
        };
        (quote_date, quote)
    });

    // Derive daily closing prices
    let series: Vec<(NaiveDate, f64)> = date_quote_pairs.clone().map(|(quote_date, quote)|{
        (quote_date, quote.close)
    }).collect();

    // Derive volatility data
    let volatile_days: Vec<(NaiveDate, Quote)> = date_quote_pairs.clone().filter(|(_, quote)| {
        (quote.high - quote.low) / quote.close > 0.02
    }).map(|(quote_date, quote)|{
        (quote_date, quote.clone())
    }).collect();

    // Derive max and min closing prices over 6 months
    let (min_quote_date, min_quote_price) = date_quote_pairs.clone().min_by(|(_, quote1), (_, quote2)| {
        quote1.close.partial_cmp(&quote2.close).unwrap()})
    .map(|(date, quote)| (date, quote.close))
    .unwrap();
    let (max_quote_date, max_quote_price) = date_quote_pairs.max_by(|(_, quote1), (_, quote2)| {
        quote1.close.partial_cmp(&quote2.close).unwrap()})
    .map(|(date, quote)| (date, quote.close))
    .unwrap();
    println!(
        "{} Stats:\nMax Closing Price: ${:.2} on {}\nMin Closing Price: ${:.2} on {}",
        stock_name, max_quote_price, max_quote_date, min_quote_price, min_quote_date
    );

    // Create a plot with volatility data and one without the volatility data
    let _ = plot_prices(
        "volatile_stock_prices.png",
        min_quote_price,
        max_quote_price,
        min_date,
        max_date,
        series.clone(),
        volatile_days,
        stock_name,
    );
    let _ = plot_prices(
        "stock_prices.png",
        min_quote_price,
        max_quote_price,
        min_date,
        max_date,
        series,
        Vec::new(),
        stock_name,
    );

    // Serve the generated plots on local host
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let _ = match handle_connection(stream) {
            Ok(()) => true,
            Err(error) => panic!{"Problem Handling Request: {:?}", error}
        };
    }
}

fn handle_connection(mut stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    // Handle the recieved request
    if buffer.starts_with(b"GET / HTTP/1.1\r\n") { // Serve the html file
        let status_line = "HTTP/1.1 200 OK";
        let contents = std::fs::read_to_string("src/plots.html").unwrap();
        let length = contents.len();

        stream.write_all(format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}").as_bytes())?;
        Ok(())
    } else if buffer.starts_with(b"GET /stock_image.png HTTP/1.1\r\n") { // Serve the plot without volatility data
        write_file_to_stream("stock_prices.png", stream)
    
    } else if buffer.starts_with(b"GET /volatile_image.png HTTP/1.1\r\n") { // Serve the plot with volatility data
        write_file_to_stream("volatile_stock_prices.png", stream)
    } else { // Invalid request case
        println!("Invalid Request");
        Ok(())
    }
}

fn write_file_to_stream(name: &str, mut stream: TcpStream)  -> std::io::Result<()> {
    let mut file = std::fs::File::open(name)?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents)?;
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: stocks_image/png\r\nContent-Length: {}\r\n\r\n",
            contents.len()
        );
        let mut response = response.into_bytes();
  
        response.extend(contents);
        stream.write_all(&response)
}