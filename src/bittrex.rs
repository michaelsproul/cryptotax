use crate::coin_gecko::PriceHists;
use crate::csv_utils::get_rows;
use crate::trade::Trade;
use crate::trade::TradeType::{Buy, Sell};
use chrono::NaiveDateTime as DateTime;
use std::error::Error;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
#[allow(dead_code)]
pub struct Row {
    order_uuid: String,
    exchange: String,
    #[serde(rename = "Type")]
    typ: String,
    quantity: f64,
    limit: f64,
    commission_paid: f64,
    price: f64,
    opened: String,
    closed: String,
}

const DATE_FMT1: &'static str = "%-m/%-d/%-y %-H:%M";
const DATE_FMT2: &'static str = "%-m/%-d/%-Y %-H:%M:%S %p";

impl Row {
    fn into_trades(self, price_hists: &PriceHists) -> Vec<Trade> {
        let datetime = DateTime::parse_from_str(&self.closed, DATE_FMT1)
            .or_else(|_| DateTime::parse_from_str(&self.closed, DATE_FMT2))
            .unwrap();

        let currencies = self.exchange.split('-').collect::<Vec<_>>();
        assert_eq!(currencies.len(), 2);

        let (sell_currency, buy_currency, sell_amount, buy_amount) = if self.typ == "LIMIT_BUY" {
            (currencies[0], currencies[1], self.price, self.quantity)
        } else {
            assert_eq!(self.typ, "LIMIT_SELL");
            (currencies[1], currencies[0], self.quantity, self.price)
        };

        let date = datetime.date();
        let op_aud_equiv = price_hists
            .convert(sell_currency, sell_amount, &date)
            .or_else(|| price_hists.convert(buy_currency, buy_amount, &date));

        let aud_equivalent = if let Some(ae) = op_aud_equiv {
            ae
        } else {
            println!(
                "Warning: skipping a transaction due to missing data for {} and {}",
                sell_currency, buy_currency
            );
            return vec![];
        };

        let sell_trade = Trade {
            datetime,
            buy_or_sell: Sell,
            crypto_currency: sell_currency.into(),
            crypto_amount: sell_amount,
            aud_equivalent,
            source: "Bittrex".into(),
            info: "".into(),
        };

        let buy_trade = Trade {
            datetime,
            buy_or_sell: Buy,
            crypto_currency: buy_currency.into(),
            crypto_amount: buy_amount,
            aud_equivalent,
            source: "Bittrex".into(),
            info: "".into(),
        };

        vec![sell_trade, buy_trade]
    }
}

pub fn load_csv(filename: &str, price_hists: &PriceHists) -> Result<Vec<Trade>, Box<Error>> {
    let rows = get_rows::<Row>(filename)?;
    let trades = rows
        .into_iter()
        .flat_map(|row| row.into_trades(price_hists))
        .collect();
    Ok(trades)
}
