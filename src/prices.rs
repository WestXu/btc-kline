use serde::Deserialize;

pub struct Prices {
    sns: reqwest::Client,
    pub data: Vec<Kline>,
}
impl Prices {
    pub async fn new() -> Self {
        let sns = reqwest::Client::new();
        let data = sns
            .get("https://api.binance.com/api/v3/uiKlines?symbol=BTCUSDT&interval=1s&limit=120")
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        Self { sns, data }
    }
    pub async fn update(&mut self) {
        let new_data = self
            .sns
            .get("https://api.binance.com/api/v3/uiKlines?symbol=BTCUSDT&interval=1s&limit=3")
            .send()
            .await
            .unwrap()
            .json::<Vec<Kline>>()
            .await
            .unwrap();
        let new_data_open_time = new_data[0].open_time;

        let data = std::mem::replace(&mut self.data, vec![]);
        self.data = data
            .into_iter()
            .filter(|kline| kline.open_time < new_data_open_time)
            .chain(new_data.into_iter())
            .collect();

        self.data = self.data.split_off(self.data.len() - 120);
    }
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
