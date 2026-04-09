//! Operations on games.

use crate::client::Client;
use crate::error::Result;
use crate::types::GameListResponse;

/// Parameters for [`GamesResource::list`].
#[derive(Debug, Clone, Default)]
pub struct GameListParams {
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// Games endpoint group.
pub struct GamesResource<'a> {
    client: &'a Client,
}

impl<'a> GamesResource<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List every supported trading card game.
    pub async fn list(&self, params: GameListParams) -> Result<GameListResponse> {
        let mut q: Vec<(&str, String)> = Vec::new();
        if let Some(v) = params.limit {
            q.push(("limit", v.to_string()));
        }
        if let Some(v) = params.offset {
            q.push(("offset", v.to_string()));
        }
        self.client.get("/games", &q).await
    }
}
