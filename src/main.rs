#[macro_use]
extern crate serde_derive;

mod btc_markets;
mod coin_gecko;
mod csv_utils;
mod trade;

use coin_gecko::PriceHist;
use std::collections::HashMap;
use std::error::Error;
use std::io;

fn run() -> Result<(), Box<Error>> {
    let currencies = &["BTC", "ETH", "MAID", "LTC"];

    let mut price_history = HashMap::new();
    for currency in currencies {
        println!("Parsing currency hist for {}", currency);
        let hist = PriceHist::from_file(&format!("price_hist/{}.csv", currency))?;
        price_history.insert(currency, hist);
    }

    let btc_markets_trades = btc_markets::load_csv("data/btcmarkets.csv")?;

    let all_transactions: Vec<_> = btc_markets_trades
        .into_iter()
        .map(|t| t.into_common())
        .collect();

    for t in all_transactions {
        println!("{:?}", t);
    }

    // Write CSV to stdout
    /*
    let mut writer = csv::Writer::from_writer(io::stdout());
    for trade in btc_markets_trades {
        writer.serialize(trade)?;
    }
    */

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Error: {:?}", e);
    }
}
