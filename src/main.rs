#[macro_use]
extern crate serde_derive;

mod btc_markets;

use std::io;
use std::error::Error;

fn run() -> Result<(), Box<Error>> {
    let btc_markets_trades = btc_markets::load_csv("data/btcmarkets.csv")?;

    // Write CSV to stdout
    let mut writer = csv::Writer::from_writer(io::stdout());
    for trade in btc_markets_trades {
        writer.serialize(trade)?;
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        println!("Error: {:?}", e);
    }
}
