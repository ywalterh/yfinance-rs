use serde::Deserialize;
use std::error::Error;

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub option_chain: OptionChain,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionChain {
    pub result: Vec<Result>,
    pub error: std::option::Option<ResponseError>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ResponseError {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub underlying_symbol: String,
    pub expiration_dates: Vec<i64>,
    pub strikes: Vec<f64>,
    pub has_mini_options: bool,
    pub quote: Quote,
    pub options: Vec<Option>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Quote {
    pub language: String,
    pub region: String,
    pub quote_type: String,
    pub type_disp: String,
    pub quote_source_name: String,
    pub triggerable: bool,
    pub custom_price_alert_confidence: String,
    pub currency: String,
    pub exchange: String,
    pub short_name: String,
    pub long_name: String,
    pub message_board_id: String,
    pub exchange_timezone_name: String,
    pub exchange_timezone_short_name: String,
    pub gmt_off_set_milliseconds: i64,
    pub market: String,
    pub esg_populated: bool,
    pub first_trade_date_milliseconds: i64,
    pub price_hint: i64,
    pub regular_market_change: f64,
    pub regular_market_change_percent: f64,
    pub regular_market_time: i64,
    pub regular_market_price: f64,
    pub regular_market_day_high: f64,
    pub regular_market_day_range: String,
    pub regular_market_day_low: f64,
    pub regular_market_volume: i64,
    pub regular_market_previous_close: f64,
    pub bid: f64,
    pub ask: f64,
    pub bid_size: i64,
    pub ask_size: i64,
    pub full_exchange_name: String,
    pub financial_currency: String,
    pub regular_market_open: f64,
    #[serde(rename = "averageDailyVolume3Month")]
    pub average_daily_volume3month: i64,
    #[serde(rename = "averageDailyVolume10Day")]
    pub average_daily_volume10day: i64,
    pub fifty_two_week_low_change: f64,
    pub fifty_two_week_low_change_percent: f64,
    pub fifty_two_week_range: String,
    pub fifty_two_week_high_change: f64,
    pub fifty_two_week_high_change_percent: f64,
    pub fifty_two_week_low: f64,
    pub fifty_two_week_high: f64,
    pub trailing_annual_dividend_rate: f64,
    #[serde(rename = "trailingPE")]
    pub trailing_pe: f64,
    pub trailing_annual_dividend_yield: f64,
    pub ytd_return: f64,
    pub trailing_three_month_returns: f64,
    pub trailing_three_month_nav_returns: f64,
    pub eps_trailing_twelve_months: f64,
    pub pre_market_change: f64,
    pub pre_market_change_percent: f64,
    pub pre_market_time: i64,
    pub pre_market_price: f64,
    pub market_state: String,
    pub shares_outstanding: i64,
    pub book_value: f64,
    pub fifty_day_average: f64,
    pub fifty_day_average_change: f64,
    pub fifty_day_average_change_percent: f64,
    pub two_hundred_day_average: f64,
    pub two_hundred_day_average_change: f64,
    pub two_hundred_day_average_change_percent: f64,
    pub market_cap: i64,
    pub price_to_book: f64,
    pub source_interval: i64,
    pub exchange_data_delayed_by: i64,
    pub page_view_growth_weekly: f64,
    pub tradeable: bool,
    pub symbol: String
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Option {
    pub expiration_date: i64,
    pub has_mini_options: bool,
    pub calls: Vec<Call>,
    pub puts: Vec<Put>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Call {
    pub contract_symbol: String,
    pub strike: f64,
    pub currency: String,
    pub last_price: f64,
    pub change: f64,
    pub percent_change: f64,
    pub open_interest: std::option::Option<i64>,
    pub bid: f64,
    pub ask: f64,
    pub contract_size: String,
    pub expiration: i64,
    pub last_trade_date: i64,
    pub implied_volatility: f64,
    pub in_the_money: bool,
    pub volume: std::option::Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Put {
    pub contract_symbol: String,
    pub strike: f64,
    pub currency: String,
    pub last_price: f64,
    pub change: f64,
    pub percent_change: f64,
    pub volume: std::option::Option<i64>,
    pub open_interest: std::option::Option<i64>,
    pub bid: f64,
    pub ask: f64,
    pub contract_size: String,
    pub expiration: i64,
    pub last_trade_date: i64,
    pub implied_volatility: f64,
    pub in_the_money: bool,
}

pub async fn new(ticker: &str) -> std::result::Result<Response, Box<dyn Error>> {
    let resp = reqwest::get(format!(
        "https://query2.finance.yahoo.com/v7/finance/options/{}",
        ticker
    ))
    .await?
    .json::<Response>()
    .await?;
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn download_ticker() {
        let under_test = new("AAPL").await;
        assert!(under_test.unwrap().option_chain.result[0].quote.regular_market_price > 0f64);

        let under_test = new("GOOG").await;
        assert!(under_test.unwrap().option_chain.result[0].quote.regular_market_price > 0f64);
    }

    #[tokio::test]
    async fn download_ticker_regression_qqq() {
        let under_test = new("QQQ").await;
        assert!(under_test.unwrap().option_chain.result[0].quote.regular_market_price > 0f64);
    }
}
