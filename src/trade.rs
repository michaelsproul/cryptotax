use chrono::NaiveDateTime as DateTime;

#[derive(Debug)]
pub enum TradeType {
    Buy,
    Sell,
}

/// Common format for all trades.
#[derive(Debug)]
pub struct Trade {
    pub datetime: DateTime,
    pub buy_or_sell: TradeType,
    pub crypto_currency: String,
    /// Amount of crypto bought or sold.
    pub crypto_amount: f64,
    /// Price paid, or amount received in AUD.
    pub aud_equivalent: f64,
    pub info: String,
}
