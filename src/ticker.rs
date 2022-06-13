use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Copy, Clone)]
pub struct Quote {
    pub ask: f32,
}

#[derive(Deserialize)]
struct OptionResult {
    quote: Quote,
}

#[derive(Deserialize)]
struct OptionChain {
    result: Vec<OptionResult>,
}
// don't know what error is returned yet
#[derive(Deserialize)]
struct ResponseError {}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Response {
    option_chain: OptionChain,
    error: Option<ResponseError>,
}

pub async fn new(ticker: String) -> Result<Quote, Box<dyn Error>> {
    let resp = reqwest::get(format!(
        "https://query2.finance.yahoo.com/v7/finance/options/{}",
        ticker
    ))
    .await?
    .json::<Response>()
    .await?;

    let quote = resp.option_chain.result[0].quote;
    Ok(quote)
}
