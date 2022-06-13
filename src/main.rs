mod ticker;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let quote = ticker::new("AAPL".into());
    println!("{}", quote.await?.ask);
    Ok(())
}
