//! Async HTTP client for the TCG Price Lookup API.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT};

use crate::error::{from_response, Error, Result};
use crate::resources::cards::CardsResource;
use crate::resources::games::GamesResource;
use crate::resources::sets::SetsResource;

const DEFAULT_BASE_URL: &str = "https://api.tcgpricelookup.com/v1";
const DEFAULT_USER_AGENT: &str = concat!("tcglookup-rs/", env!("CARGO_PKG_VERSION"));
const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Rate-limit window state captured from the most recent response.
#[derive(Debug, Clone, Copy, Default)]
pub struct RateLimitInfo {
    pub limit: Option<u32>,
    pub remaining: Option<u32>,
}

/// Async client for the TCG Price Lookup REST API.
///
/// Construct via [`Client::new`] or [`Client::builder`]. The client is
/// cheap to clone (internally `Arc`-shared HTTP pool) and safe to share
/// across tasks.
#[derive(Clone)]
pub struct Client {
    inner: Arc<ClientInner>,
}

struct ClientInner {
    api_key: String,
    base_url: String,
    user_agent: String,
    http: reqwest::Client,
    rate_limit: Mutex<RateLimitInfo>,
}

impl Client {
    /// Construct a client with default settings.
    ///
    /// Get a free API key at <https://tcgpricelookup.com/tcg-api>.
    pub fn new(api_key: impl Into<String>) -> Self {
        ClientBuilder::new(api_key).build()
    }

    /// Start configuring a custom client.
    pub fn builder(api_key: impl Into<String>) -> ClientBuilder {
        ClientBuilder::new(api_key)
    }

    /// Returns the [`CardsResource`] handle.
    pub fn cards(&self) -> CardsResource<'_> {
        CardsResource::new(self)
    }

    /// Returns the [`SetsResource`] handle.
    pub fn sets(&self) -> SetsResource<'_> {
        SetsResource::new(self)
    }

    /// Returns the [`GamesResource`] handle.
    pub fn games(&self) -> GamesResource<'_> {
        GamesResource::new(self)
    }

    /// Returns the most recent rate-limit window state.
    pub fn rate_limit(&self) -> RateLimitInfo {
        *self.inner.rate_limit.lock().unwrap()
    }

    /// Internal: GET a path with optional query params and decode the JSON body.
    pub(crate) async fn get<T>(&self, path: &str, query: &[(&str, String)]) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}{}", self.inner.base_url, path);
        let req = self
            .inner
            .http
            .get(&url)
            .header("X-API-Key", &self.inner.api_key)
            .header(ACCEPT, "application/json")
            .header(USER_AGENT, &self.inner.user_agent)
            .query(query);
        let res = req.send().await?;
        let status = res.status();
        self.capture_rate_limit(res.headers());
        let body = res.text().await?;
        if !status.is_success() {
            return Err(from_response(status.as_u16(), body));
        }
        if body.is_empty() {
            return Err(Error::Api {
                status: status.as_u16(),
                message: "empty response body".to_string(),
                body: String::new(),
            });
        }
        serde_json::from_str(&body).map_err(Error::from)
    }

    fn capture_rate_limit(&self, headers: &HeaderMap) {
        let parse = |k: &str| {
            headers
                .get(k)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u32>().ok())
        };
        let mut rl = self.inner.rate_limit.lock().unwrap();
        rl.limit = parse("x-ratelimit-limit");
        rl.remaining = parse("x-ratelimit-remaining");
    }
}

/// Builder for [`Client`] with custom configuration.
pub struct ClientBuilder {
    api_key: String,
    base_url: String,
    user_agent: String,
    timeout: Duration,
}

impl ClientBuilder {
    /// Start a builder with the default base URL and timeout.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: DEFAULT_BASE_URL.to_string(),
            user_agent: DEFAULT_USER_AGENT.to_string(),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
        }
    }

    /// Override the API base URL (useful for tests).
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into().trim_end_matches('/').to_string();
        self
    }

    /// Override the per-request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Override the User-Agent header.
    pub fn user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = ua.into();
        self
    }

    /// Build the configured [`Client`].
    pub fn build(self) -> Client {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        let http = reqwest::Client::builder()
            .timeout(self.timeout)
            .default_headers(headers)
            .build()
            .expect("reqwest client construction should never fail");
        Client {
            inner: Arc::new(ClientInner {
                api_key: self.api_key,
                base_url: self.base_url,
                user_agent: self.user_agent,
                http,
                rate_limit: Mutex::new(RateLimitInfo::default()),
            }),
        }
    }
}
