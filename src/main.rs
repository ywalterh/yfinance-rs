use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
struct Quote {
    ask: f32,
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let resp = reqwest::get("https://query2.finance.yahoo.com/v7/finance/options/AAPL")
        .await?
        .json::<Response>()
        .await?;

    println!("{}", resp.option_chain.result[0].quote.ask);
    Ok(())
}
