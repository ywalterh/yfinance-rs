use serde::Deserialize;
use std::error::Error;

#[derive(Default)]
pub struct Params {
    range: String,    //Valid periods: 1d,5d,1mo,3mo,6mo,1y,2y,5y,10y,ytd,max
    interval: String, // 1m,2m,5m,15m,30m,60m,90m,1h,1d,5d,1wk,1mo,3mo
    start: String,    // (YYYY-MM-DD) or _datetime
    end: String,
    prepost: bool,
    auto_adjust: bool,
    back_adjust: bool,
    time_zone: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub chart: Chart,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    pub result: Vec<Result>,
    pub error: std::option::Option<ResponseError>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ResponseError {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub meta: Meta,
    pub timestamp: Vec<i64>,
    pub indicators: Indicators,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub currency: String,
    pub symbol: String,
    pub exchange_name: String,
    pub instrument_type: String,
    pub first_trade_date: i64,
    pub regular_market_time: i64,
    pub gmtoffset: i64,
    pub timezone: String,
    pub exchange_timezone_name: String,
    pub regular_market_price: f64,
    pub chart_previous_close: f64,
    pub previous_close: f64,
    pub scale: i64,
    pub price_hint: i64,
    pub current_trading_period: CurrentTradingPeriod,
    pub trading_periods: Vec<Vec<TradingPeriod>>,
    pub data_granularity: String,
    pub range: String,
    pub valid_ranges: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentTradingPeriod {
    pub pre: Pre,
    pub regular: Regular,
    pub post: Post,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pre {
    pub timezone: String,
    pub end: i64,
    pub start: i64,
    pub gmtoffset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Regular {
    pub timezone: String,
    pub end: i64,
    pub start: i64,
    pub gmtoffset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub timezone: String,
    pub end: i64,
    pub start: i64,
    pub gmtoffset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TradingPeriod {
    pub timezone: String,
    pub end: i64,
    pub start: i64,
    pub gmtoffset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Indicators {
    pub quote: Vec<Quote>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub open: Vec<f64>,
    pub volume: Vec<i64>,
    pub high: Vec<f64>,
    pub close: Vec<f64>,
    pub low: Vec<f64>,
}

pub async fn new(ticker: &str, params: Option<Params>) -> std::result::Result<Response, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let params = params.unwrap_or(Params::default());

    let resp = client
        .get(format!(
            "https://query2.finance.yahoo.com/v8/finance/chart/{}",
            ticker
        ))
        .query(&[
            ("range", params.range),
            ("interval", params.interval),
            ("start", params.start),
            ("end", params.end),
            ("prepost", params.prepost.to_string()),
            ("auto_adjust", params.auto_adjust.to_string()),
            ("back_adjust", params.back_adjust.to_string()),
            ("time_zone", params.time_zone),
        ])
        .send()
        .await?
        .json::<Response>()
        .await?;

    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn download_history() {
        let under_test = new("AAPL", None).await;
        under_test.unwrap();
    }
}
