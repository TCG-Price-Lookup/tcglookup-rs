//! Error types for the TCG Price Lookup SDK.

use thiserror::Error;

/// SDK result alias.
pub type Result<T> = std::result::Result<T, Error>;

/// All errors the SDK can return.
///
/// Use a `match` to branch on specific HTTP status conditions, or
/// inspect [`Error::status`] for the raw HTTP code.
#[derive(Debug, Error)]
pub enum Error {
    /// 401 — missing or invalid API key.
    #[error("authentication failed: {message}")]
    Authentication { message: String, body: String },

    /// 403 — your plan does not include access to this resource.
    ///
    /// Free-tier API keys hit this on price history endpoints.
    /// Upgrade at <https://tcgpricelookup.com/tcg-api>.
    #[error("plan access denied: {message}")]
    PlanAccess { message: String, body: String },

    /// 404 — card / set / game does not exist.
    #[error("not found: {message}")]
    NotFound { message: String, body: String },

    /// 429 — rate limit exceeded.
    #[error("rate limited: {message}")]
    RateLimit { message: String, body: String },

    /// Any other non-2xx response from the API.
    #[error("api error (HTTP {status}): {message}")]
    Api {
        status: u16,
        message: String,
        body: String,
    },

    /// Network or transport error.
    #[error("transport error: {0}")]
    Transport(#[from] reqwest::Error),

    /// JSON decode error.
    #[error("decode error: {0}")]
    Decode(#[from] serde_json::Error),
}

impl Error {
    /// Returns the HTTP status code if this error originated from an API
    /// response, or `None` for transport / decode errors.
    pub fn status(&self) -> Option<u16> {
        match self {
            Error::Authentication { .. } => Some(401),
            Error::PlanAccess { .. } => Some(403),
            Error::NotFound { .. } => Some(404),
            Error::RateLimit { .. } => Some(429),
            Error::Api { status, .. } => Some(*status),
            _ => None,
        }
    }
}

pub(crate) fn from_response(status: u16, body: String) -> Error {
    let message = extract_message(&body).unwrap_or_else(|| format!("HTTP {status}"));
    match status {
        401 => Error::Authentication { message, body },
        403 => Error::PlanAccess { message, body },
        404 => Error::NotFound { message, body },
        429 => Error::RateLimit { message, body },
        _ => Error::Api { status, message, body },
    }
}

fn extract_message(body: &str) -> Option<String> {
    #[derive(serde::Deserialize)]
    struct Err {
        error: Option<String>,
    }
    serde_json::from_str::<Err>(body).ok().and_then(|e| e.error)
}
