//! Error Type for the API.

use snafu::Snafu;

/// Generic Result for the Library
pub type SpotifyResult<T, E = SpotifyError> = Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum SpotifyError {
    #[snafu(display("Unable to parse JSON: {}", source))]
    SerdeError { source: serde_json::Error },

    #[snafu(display("Unable to parse URL: {}", source))]
    UrlError { source: url::ParseError },

    #[snafu(display("Token parsing failure: {}", context))]
    TokenFailure { context: &'static str },

    #[snafu(display("Callback URL parsing failure: {}", context))]
    CallbackFailure { context: &'static str },

    #[snafu(display("Surf http failure: {}", context))]
    SurfError {
        // NOTE:
        // 'source: Box<dyn error::Error + Send + Sync>'
        // does not work with surf v2.x anymore.
        context: String,
    },
}
