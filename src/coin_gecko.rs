use crate::csv_utils::get_rows;
use chrono::NaiveDate as Date;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Row {
    snapped_at: String,
    price: Option<f64>,
    market_cap: String,
    total_volume: String,
}

#[derive(Default)]
pub struct PriceHist {
    pub data: BTreeMap<Date, Option<f64>>,
}

#[derive(Default)]
pub struct PriceHists {
    pub hists: HashMap<&'static str, PriceHist>,
}

impl PriceHist {
    pub fn from_file(filename: &str) -> Result<Self, Box<Error>> {
        let rows = get_rows::<Row>(filename)?;

        let data = rows
            .into_iter()
            .map(|row| {
                let date_len = "YYYY-MM-DD".len();
                let date = Date::parse_from_str(&row.snapped_at[..date_len], "%Y-%m-%d").unwrap();
                (date, row.price)
            })
            .collect();

        Ok(PriceHist { data })
    }
}

impl PriceHists {
    pub fn convert(&self, currency: &str, amount: f64, date: &Date) -> Option<f64> {
        self.hists
            .get(currency)
            .and_then(|hist| hist.data.get(date))
            .and_then(|op_price| op_price.map(|price| price * amount))
    }
}
