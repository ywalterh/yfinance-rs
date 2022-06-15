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
    // error: Option<ResponseError>,
}

pub async fn new(ticker: &str) -> Result<Quote, Box<dyn Error>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn download_ticker() {
        let under_test = new("AAPL").await;
        assert!(under_test.unwrap().ask > 0f32);

        let under_test = new("GOOG").await;
        assert!(under_test.unwrap().ask > 0f32);
    }
}
