//! # tcglookup
//!
//! The official Rust SDK for the [TCG Price Lookup API].
//!
//! Live trading card prices across Pokemon, Magic: The Gathering, Yu-Gi-Oh!,
//! Disney Lorcana, One Piece TCG, Star Wars: Unlimited, and Flesh and Blood.
//!
//! Get a free API key at <https://tcgpricelookup.com/tcg-api>
//!
//! ## Quickstart
//!
//! ```no_run
//! use tcglookup::{Client, CardSearchParams};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new("tlk_live_...");
//!
//!     let results = client
//!         .cards()
//!         .search(CardSearchParams {
//!             q: Some("charizard".into()),
//!             game: Some("pokemon".into()),
//!             limit: Some(5),
//!             ..Default::default()
//!         })
//!         .await?;
//!
//!     for card in results.data {
//!         println!("{} — {}", card.name, card.set.name);
//!     }
//!     Ok(())
//! }
//! ```
//!
//! [TCG Price Lookup API]: https://tcgpricelookup.com/tcg-api

mod client;
mod error;
mod resources;
mod types;

pub use client::{Client, ClientBuilder, RateLimitInfo};
pub use error::{Error, Result};
pub use resources::cards::{CardSearchParams, CardsResource, HistoryParams};
pub use resources::games::{GameListParams, GamesResource};
pub use resources::sets::{SetListParams, SetsResource};
pub use types::*;
