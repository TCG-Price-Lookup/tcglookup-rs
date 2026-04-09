//! Run with: cargo run --example search -- <YOUR_API_KEY>
//!
//! Get a free API key at https://tcgpricelookup.com/tcg-api

use tcglookup::{CardSearchParams, Client};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::args()
        .nth(1)
        .or_else(|| std::env::var("TCGLOOKUP_API_KEY").ok())
        .expect("pass API key as first arg or set TCGLOOKUP_API_KEY");

    let client = Client::new(api_key);

    let results = client
        .cards()
        .search(CardSearchParams {
            q: Some("charizard".into()),
            game: Some("pokemon".into()),
            limit: Some(5),
            ..Default::default()
        })
        .await?;

    println!("Found {} matches", results.total);
    for card in results.data {
        println!("- {} ({})", card.name, card.set.name);
    }
    Ok(())
}
