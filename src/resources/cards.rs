//! Operations on cards.

use crate::client::Client;
use crate::error::Result;
use crate::types::{Card, CardSearchResponse, HistoryResponse};

/// Backend hard cap on the `ids` parameter per request.
/// [`CardsResource::search`] auto-chunks larger arrays.
pub const SEARCH_IDS_CHUNK_SIZE: usize = 20;

/// Parameters for [`CardsResource::search`].
#[derive(Debug, Clone, Default)]
pub struct CardSearchParams {
    /// Free-text search across card name, set name, and number.
    pub q: Option<String>,
    /// Batch lookup. Auto-chunks above 20 IDs.
    pub ids: Option<Vec<String>>,
    /// Game slug: pokemon, mtg, yugioh, onepiece, lorcana, swu, fab.
    pub game: Option<String>,
    /// Set slug.
    pub set: Option<String>,
    /// 1-100, default 20.
    pub limit: Option<u32>,
    /// Pagination offset, default 0.
    pub offset: Option<u32>,
}

/// Parameters for [`CardsResource::history`].
#[derive(Debug, Clone, Default)]
pub struct HistoryParams {
    /// 7d, 30d, 90d, 1y. Defaults to 30d on the backend.
    pub period: Option<String>,
}

/// Cards endpoint group.
pub struct CardsResource<'a> {
    client: &'a Client,
}

impl<'a> CardsResource<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Search cards by name, set, game, or batch by IDs.
    ///
    /// Passing more than 20 IDs auto-chunks into multiple requests
    /// and merges the results.
    pub async fn search(&self, params: CardSearchParams) -> Result<CardSearchResponse> {
        let ids = params.ids.clone().unwrap_or_default();
        if ids.is_empty() {
            return self.search_once(&params, None).await;
        }
        if ids.len() <= SEARCH_IDS_CHUNK_SIZE {
            return self.search_once(&params, Some(ids.join(","))).await;
        }
        self.search_chunked(&params, ids).await
    }

    /// Get a single card by its UUID.
    pub async fn get(&self, id: &str) -> Result<Card> {
        let path = format!("/cards/{}", urlencode(id));
        self.client.get(&path, &[]).await
    }

    /// Daily price history. Trader plan and above.
    ///
    /// Free-tier API keys receive an [`Error::PlanAccess`].
    pub async fn history(&self, id: &str, params: HistoryParams) -> Result<HistoryResponse> {
        let path = format!("/cards/{}/history", urlencode(id));
        let mut q: Vec<(&str, String)> = Vec::new();
        if let Some(p) = params.period {
            q.push(("period", p));
        }
        self.client.get(&path, &q).await
    }

    async fn search_once(
        &self,
        params: &CardSearchParams,
        ids: Option<String>,
    ) -> Result<CardSearchResponse> {
        let mut q: Vec<(&str, String)> = Vec::new();
        if let Some(v) = &params.q {
            q.push(("q", v.clone()));
        }
        if let Some(v) = ids {
            q.push(("ids", v));
        }
        if let Some(v) = &params.game {
            q.push(("game", v.clone()));
        }
        if let Some(v) = &params.set {
            q.push(("set", v.clone()));
        }
        if let Some(v) = params.limit {
            q.push(("limit", v.to_string()));
        }
        if let Some(v) = params.offset {
            q.push(("offset", v.to_string()));
        }
        self.client.get("/cards/search", &q).await
    }

    async fn search_chunked(
        &self,
        params: &CardSearchParams,
        ids: Vec<String>,
    ) -> Result<CardSearchResponse> {
        let mut merged: Vec<Card> = Vec::new();
        for chunk in ids.chunks(SEARCH_IDS_CHUNK_SIZE) {
            let page = self.search_once(params, Some(chunk.join(","))).await?;
            merged.extend(page.data);
        }
        let total = merged.len();
        Ok(CardSearchResponse {
            data: merged,
            total,
            limit: params.limit.map(|v| v as usize).unwrap_or(total),
            offset: params.offset.map(|v| v as usize).unwrap_or(0),
        })
    }
}

fn urlencode(s: &str) -> String {
    // Minimal path-segment encoder that's good enough for UUIDs.
    // We avoid pulling in a full url crate to keep deps small.
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{:02X}", b));
        }
    }
    out
}
