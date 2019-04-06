#[macro_use]
extern crate serde_derive;

mod bittrex;
mod btc_markets;
mod coin_gecko;
mod csv_utils;
mod trade;

use coin_gecko::{PriceHist, PriceHists};
use std::error::Error;
use std::io;

fn run() -> Result<(), Box<Error>> {
    let currencies = &["BTC", "ETH", "MAID", "LTC"];

    let mut price_history = PriceHists::default();
    for currency in currencies {
        eprintln!("Parsing currency hist for {}", currency);
        let hist = PriceHist::from_file(&format!("price_hist/{}.csv", currency))?;
        price_history.hists.insert(currency, hist);
    }

    let btc_markets_trades = btc_markets::load_csv("data/btcmarkets.csv")?
        .into_iter()
        .map(|t| t.into_common())
        .collect::<Vec<_>>();

    let bittrex_trades = bittrex::load_csv("data/bittrex.csv", &price_history)?;

    let mut all_trades = btc_markets_trades;
    all_trades.extend(bittrex_trades);

    // Sort by transaction date
    all_trades.sort_by_key(|t| t.datetime);

    // Write CSV to stdout
    let mut writer = csv::Writer::from_writer(io::stdout());
    for trade in all_trades {
        writer.serialize(trade)?;
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Error: {:?}", e);
    }
}
