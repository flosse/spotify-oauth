//! Error Type for the API.

use thiserror::Error;

use crate::fetch::HttpClientError;

/// Generic Result for the Library
pub type SpotifyResult<T, E = SpotifyError> = Result<T, E>;

#[derive(Debug, Error)]
pub enum SpotifyError {
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    ParseUrl(#[from] url::ParseError),

    #[error(transparent)]
    HttpClient(#[from] HttpClientError),

    #[error("Token parsing failure: {}", context)]
    TokenFailure { context: &'static str },

    #[error("Callback URL parsing failure: {}", context)]
    CallbackFailure { context: &'static str },
}
