//! Public data types returned by the TCG Price Lookup API.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single trading card with its current prices.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: String,
    pub tcgplayer_id: Option<i64>,
    pub name: String,
    pub number: Option<String>,
    pub rarity: Option<String>,
    pub variant: Option<String>,
    pub image_url: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_price_update: Option<String>,
    pub updated_at: String,
    pub set: CardSetRef,
    pub game: CardGameRef,
    pub prices: CardPrices,
}

/// Embedded set reference on a card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardSetRef {
    pub id: String,
    pub slug: String,
    pub name: String,
}

/// Embedded game reference on a card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardGameRef {
    pub id: String,
    pub slug: String,
    pub name: String,
}

/// Raw and graded prices for a card.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardPrices {
    pub raw: HashMap<String, RawConditionPrices>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graded: Option<HashMap<String, HashMap<String, GradedGradePrices>>>,
}

/// Source-keyed prices for one raw condition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawConditionPrices {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcgplayer: Option<TcgPlayerPrices>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ebay: Option<EbayAverages>,
}

/// Standard TCGPlayer market / low / mid / high points.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcgPlayerPrices {
    pub market: Option<f64>,
    pub low: Option<f64>,
    pub mid: Option<f64>,
    pub high: Option<f64>,
}

/// Rolling-window averages from eBay sold listings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EbayAverages {
    pub avg_1d: Option<f64>,
    pub avg_7d: Option<f64>,
    pub avg_30d: Option<f64>,
}

/// Per-source price block for a single grade.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GradedGradePrices {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ebay: Option<EbayAverages>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tcgplayer: Option<EbayAverages>,
}

/// Paginated card search response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardSearchResponse {
    pub data: Vec<Card>,
    pub total: usize,
    pub limit: usize,
    pub offset: usize,
}

/// Daily price history.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryResponse {
    pub data: Vec<HistoryDay>,
    pub period: String,
}

/// One day of price observations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryDay {
    pub date: String,
    pub prices: Vec<HistoryPriceRow>,
}

/// One observation within a day.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryPriceRow {
    pub source: String,
    pub condition: Option<String>,
    pub grader: Option<String>,
    pub grade: Option<String>,
    pub price_market: Option<f64>,
    pub price_low: Option<f64>,
    pub price_mid: Option<f64>,
    pub price_high: Option<f64>,
    pub avg_1d: Option<f64>,
    pub avg_7d: Option<f64>,
    pub avg_30d: Option<f64>,
}

/// One row in the sets list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetSummary {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub game: String,
    pub count: usize,
    pub released_at: Option<String>,
}

/// Paginated set list response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetListResponse {
    pub data: Vec<SetSummary>,
    pub total: usize,
    pub limit: usize,
    pub offset: usize,
}

/// One row in the games list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSummary {
    pub id: String,
    pub slug: String,
    pub name: String,
    pub count: usize,
}

/// Paginated game list response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameListResponse {
    pub data: Vec<GameSummary>,
    pub total: usize,
    pub limit: usize,
    pub offset: usize,
}
