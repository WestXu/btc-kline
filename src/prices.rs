use serde::Deserialize;

pub fn get_kline() -> Vec<Kline> {
    reqwest::blocking::get(
        "https://api.binance.com/api/v3/uiKlines?symbol=BTCUSDT&interval=15m&limit=96",
    )
    .unwrap()
    .json()
    .unwrap()
}

#[derive(Debug, Deserialize)]
pub struct Kline {
    pub open_time: u64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume: String,
    pub close_time: u64,
    _quote_asset_volume: String,
    _number_of_trades: u64,
    _taker_buy_base_asset_volume: String,
    _taker_buy_quote_asset_volume: String,
    _unused_field: String,
}

// [
//   [
//     1499040000000,      // Kline open time
//     "0.01634790",       // Open price
//     "0.80000000",       // High price
//     "0.01575800",       // Low price
//     "0.01577100",       // Close price
//     "148976.11427815",  // Volume
//     1499644799999,      // Kline Close time
//     "2434.19055334",    // Quote asset volume
//     308,                // Number of trades
//     "1756.87402397",    // Taker buy base asset volume
//     "28.46694368",      // Taker buy quote asset volume
//     "0"                 // Unused field, ignore.
//   ]
// ]
