//! Operations on sets.

use crate::client::Client;
use crate::error::Result;
use crate::types::SetListResponse;

/// Parameters for [`SetsResource::list`].
#[derive(Debug, Clone, Default)]
pub struct SetListParams {
    pub game: Option<String>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// Sets endpoint group.
pub struct SetsResource<'a> {
    client: &'a Client,
}

impl<'a> SetsResource<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List sets across all games, or filter by game slug.
    pub async fn list(&self, params: SetListParams) -> Result<SetListResponse> {
        let mut q: Vec<(&str, String)> = Vec::new();
        if let Some(v) = params.game {
            q.push(("game", v));
        }
        if let Some(v) = params.limit {
            q.push(("limit", v.to_string()));
        }
        if let Some(v) = params.offset {
            q.push(("offset", v.to_string()));
        }
        self.client.get("/sets", &q).await
    }
}
