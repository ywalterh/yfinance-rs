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

pub async fn new(ticker: &str, params: Option<Params>) -> Result<(), Box<dyn Error>> {
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
        .text()
        .await?;

    println!("{}", resp);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn download_history() {
        let under_test = new("AAPL", None).await;
        assert!(false);
    }
}
