use crate::csv_utils::get_rows;
use crate::trade;
use crate::trade::TradeType::{Buy, Sell};
use chrono::NaiveDateTime as DateTime;
use itertools::Itertools;
use std::error::Error;

pub fn load_csv(filename: &str) -> Result<Vec<Trade>, Box<Error>> {
    let rows = get_rows::<Row>(filename)?;

    let trades_grouped = rows
        .into_iter()
        .filter(|r| r.record_type == "Trade")
        .group_by(|r| r.reference_id);

    let mut trades = vec![];
    for (_, txn_parts) in &trades_grouped {
        let mut trade = Trade::default();
        for part in txn_parts {
            trade.process_part(&part);
        }
        trades.push(trade);
    }

    Ok(trades)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Row {
    creation_time: String,
    record_type: String,
    action: String,
    currency: String,
    // TODO: use rationals instead of floats?
    amount: f64,
    description: String,
    reference_id: u32,
}

#[derive(Default, Debug, Serialize)]
pub struct Trade {
    creation_time: String,
    reference_id: u32,
    sold_currency: String,
    sold_amount: f64,
    bought_currency: String,
    bought_amount: f64,
    description: String,
}

impl Trade {
    fn update_common_info(&mut self, row: &Row) {
        if self.reference_id == 0 {
            self.reference_id = row.reference_id;
            self.description = row.description.clone();
            self.creation_time = row.creation_time.clone();
        }
        assert_eq!(self.reference_id, row.reference_id);
    }

    fn add_buy_part(&mut self, currency: &str, amount: f64) {
        assert!(amount >= 0.0);
        if self.bought_currency == "" {
            self.bought_currency = currency.into();
        }
        assert_eq!(self.bought_currency, currency);
        self.bought_amount += amount;
    }

    fn add_sell_part(&mut self, currency: &str, amount: f64) {
        assert!(amount >= 0.0);
        if self.sold_currency == "" {
            self.sold_currency = currency.into();
        }
        assert_eq!(self.sold_currency, currency);
        self.sold_amount += amount;
    }

    fn add_txn_fee(&mut self, currency: &str, amount: f64) {
        if currency == self.bought_currency {
            self.bought_amount += -1.0 * amount;
        } else if currency == self.sold_currency {
            self.sold_amount += amount;
        } else {
            panic!(
                "currency not one of the ones bought or sold in this txn: {} not in {}, {}",
                currency, self.bought_currency, self.sold_currency
            );
        }
    }

    fn process_part(&mut self, part: &Row) {
        self.update_common_info(part);

        match part.record_type.as_str() {
            "Trade" => self.process_trade(part),
            "Fund Transfer" => self.process_transfer(part),
            ty => panic!("Unknown record type: {}", ty),
        }
    }

    fn process_trade(&mut self, part: &Row) {
        if part.action == "Trading Fee" {
            self.add_txn_fee(&part.currency, part.amount);
        } else if part.amount >= 0.0 {
            self.add_buy_part(&part.currency, part.amount);
        } else {
            self.add_sell_part(&part.currency, -1.0 * part.amount);
        }
    }

    fn process_transfer(&mut self, _: &Row) {
        // TODO: do something sensible here
    }

    pub fn into_common(self) -> trade::Trade {
        let datetime = DateTime::parse_from_str(&self.creation_time, "%+").unwrap();

        // Crypto sell
        if self.bought_currency == "AUD" {
            trade::Trade {
                datetime,
                buy_or_sell: Sell,
                crypto_currency: self.sold_currency,
                crypto_amount: self.sold_amount,
                aud_equivalent: self.bought_amount,
                info: format!("{}: {}", self.reference_id, self.description),
            }
        }
        // Crypto buy
        else {
            trade::Trade {
                datetime,
                buy_or_sell: Buy,
                crypto_currency: self.bought_currency,
                crypto_amount: self.bought_amount,
                aud_equivalent: self.sold_amount,
                info: format!("{}: {}", self.reference_id, self.description),
            }
        }
    }
}
