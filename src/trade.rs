use chrono::NaiveDateTime as DateTime;
use serde::Serializer;

const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

#[derive(Debug, Serialize)]
pub enum TradeType {
    Buy,
    Sell,
}

/// Common format for all trades.
#[derive(Debug, Serialize)]
pub struct Trade {
    #[serde(serialize_with = "serialize_datetime")]
    pub datetime: DateTime,
    pub buy_or_sell: TradeType,
    pub crypto_currency: String,
    /// Amount of crypto bought or sold.
    pub crypto_amount: f64,
    /// Price paid, or amount received in AUD.
    pub aud_equivalent: f64,
    /// Exchange where the trade occurred.
    pub source: String,
    pub info: String,
}

fn serialize_datetime<S>(datetime: &DateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&datetime.format(DATE_FORMAT).to_string())
}
