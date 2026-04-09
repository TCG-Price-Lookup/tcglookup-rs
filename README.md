# tcglookup-rs

[![Crates.io](https://img.shields.io/crates/v/tcglookup.svg)](https://crates.io/crates/tcglookup)
[![docs.rs](https://docs.rs/tcglookup/badge.svg)](https://docs.rs/tcglookup)
[![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)
[![Powered by TCG Price Lookup](https://img.shields.io/badge/powered%20by-TCG%20Price%20Lookup-purple.svg)](https://tcgpricelookup.com/tcg-api)

The official Rust SDK for the [**TCG Price Lookup API**](https://tcgpricelookup.com/tcg-api) — live trading card prices across **Pokemon, Magic: The Gathering, Yu-Gi-Oh!, Disney Lorcana, One Piece TCG, Star Wars: Unlimited, and Flesh and Blood**.

One API for every major trading card game. TCGPlayer market prices, eBay sold averages, and PSA / BGS / CGC graded comps — all in one place.

## Install

```toml
[dependencies]
tcglookup = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

## Quickstart

```rust
use tcglookup::{Client, CardSearchParams};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("tlk_live_...");

    let results = client
        .cards()
        .search(CardSearchParams {
            q: Some("charizard".into()),
            game: Some("pokemon".into()),
            limit: Some(5),
            ..Default::default()
        })
        .await?;

    for card in results.data {
        println!("{} — {}", card.name, card.set.name);
    }
    Ok(())
}
```

## Get an API key

Sign up at [tcgpricelookup.com/tcg-api](https://tcgpricelookup.com/tcg-api). Free tier includes 10,000 requests per month with TCGPlayer market prices. Trader plan unlocks eBay sold averages, PSA / BGS / CGC graded prices, and full price history.

## API surface

### Cards

```rust
// Search
client.cards().search(CardSearchParams {
    q: Some("blue-eyes white dragon".into()),
    game: Some("yugioh".into()),  // pokemon | mtg | yugioh | onepiece | lorcana | swu | fab
    set: Some("lob".into()),
    limit: Some(20),
    ..Default::default()
}).await?;

// Get one
let card = client.cards().get("<card-uuid>").await?;

// Daily price history (Trader plan)
let hist = client.cards().history("<card-uuid>", HistoryParams {
    period: Some("30d".into()),  // 7d | 30d | 90d | 1y
}).await?;
```

### Sets

```rust
let sets = client.sets().list(SetListParams {
    game: Some("mtg".into()),
    limit: Some(50),
    ..Default::default()
}).await?;
```

### Games

```rust
let games = client.games().list(GameListParams::default()).await?;
```

### Batch lookups

Pass any `Vec<String>` and the SDK auto-chunks into 20-ID batches:

```rust
let results = client.cards().search(CardSearchParams {
    ids: Some(vec!["uuid1".into(), "uuid2".into() /* ... */]),
    ..Default::default()
}).await?;
```

## Error handling

```rust
use tcglookup::Error;

match client.cards().history("<uuid>", Default::default()).await {
    Ok(hist) => { /* ... */ }
    Err(Error::PlanAccess { .. }) => {
        eprintln!("History requires Trader plan — upgrade at tcgpricelookup.com/tcg-api");
    }
    Err(Error::RateLimit { .. }) => {
        let rl = client.rate_limit();
        eprintln!("Rate limited. Quota: {:?}/{:?}", rl.remaining, rl.limit);
    }
    Err(e) => eprintln!("API error: {e}"),
}
```

## Configuration

```rust
use std::time::Duration;
use tcglookup::Client;

let client = Client::builder("tlk_live_...")
    .base_url("https://api.tcgpricelookup.com/v1")
    .timeout(Duration::from_secs(60))
    .user_agent("my-app/1.0")
    .build();
```

## Sister SDKs

- [tcglookup-js](https://github.com/TCG-Price-Lookup/tcglookup-js) — JavaScript / TypeScript
- [tcglookup-py](https://github.com/TCG-Price-Lookup/tcglookup-py) — Python
- [tcglookup-go](https://github.com/TCG-Price-Lookup/tcglookup-go) — Go
- [tcglookup-php](https://github.com/TCG-Price-Lookup/tcglookup-php) — PHP
- [tcglookup CLI](https://www.npmjs.com/package/tcglookup) — terminal client

The full developer ecosystem index lives at **[awesome-tcg](https://github.com/TCG-Price-Lookup/awesome-tcg)**.

## License

MIT — see [LICENSE](LICENSE).

---

Built by [TCG Price Lookup](https://tcgpricelookup.com). Get a free API key at [tcgpricelookup.com/tcg-api](https://tcgpricelookup.com/tcg-api).
