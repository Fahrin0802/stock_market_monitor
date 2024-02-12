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
    min_price: f64,
    max_price: f64,
    min_date: NaiveDate,
    max_date: NaiveDate,
    series: Vec<(NaiveDate, f64)>,
    stock_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create a drawing area
    let root = BitMapBackend::new("stock_prices.png", (800, 600)).into_drawing_area();
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
    if !is_valid_stock(stock_name, &provider) {
        eprintln!("Error: The stock symbol {} is not valid.", stock_name);
        process::exit(1);
    }

    // Get today's date and six months prior date
    let today = OffsetDateTime::now_utc();
    let six_months_ago = today - Duration::days(30 * 6);

    let quotes = get_stock_prices(stock_name, today, six_months_ago, &provider);

    let mut series = Vec::new();

    // TODO remove deprecated functions
    let min_date: NaiveDate = NaiveDate::from_ymd(
        six_months_ago.year(),
        six_months_ago.month() as u32,
        six_months_ago.day().into(),
    );
    let max_date: NaiveDate =
        NaiveDate::from_ymd(today.year(), today.month() as u32, today.day().into());
    let mut min_price: f64 = 10000.0;
    let mut max_price: f64 = 0.0;

    for item in quotes {
        let datetime_utc = Utc.timestamp(item.timestamp as i64, 0);
        let item_date: NaiveDate = NaiveDate::from_ymd(
            datetime_utc.year(),
            datetime_utc.month() as u32,
            datetime_utc.day().into(),
        );

        let x: (NaiveDate, f64) = (item_date, item.close);
        series.push(x);

        if item.close > max_price {
            max_price = item.close
        }
        if item.close < min_price {
            min_price = item.close
        }
    }

    let _ = plot_prices(min_price, max_price, min_date, max_date, series, stock_name);
}

// Quote {
//     timestamp: 1577975400,
//     open: 74.05,
//     high: 75.15,
//     low: 73.79,
//     volume: 135480400,
//     close: 75.08,
//     adjclose: 73.05
// }
