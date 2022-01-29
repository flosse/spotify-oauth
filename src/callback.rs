use crate::{error, error::*};
use snafu::ResultExt;
use std::{str::FromStr, string::ToString};
use url::Url;

/// The Spotify Callback URL
///
/// This struct follows the parameters given at [this](https://developer.spotify.com/documentation/general/guides/authorization-guide/ "Spotify Auth Documentation") link.
///
/// The main use of this object is to convert the callback URL into an object that can be used to generate a token.
/// If needed you can also create this callback object using the ``new`` function in the struct.
///
/// # Example
///
/// ```
/// # use spotify_oauth::SpotifyCallback;
/// # use std::str::FromStr;
/// // Create a new spotify callback object using the callback url given by the authorization process.
/// // This object can then be converted into the token needed for the application.
/// let callback = SpotifyCallback::from_str("https://example.com/callback?code=NApCCgBkWtQ&state=test").unwrap();
/// # assert_eq!(callback, SpotifyCallback::new(Some("NApCCgBkWtQ".to_string()), None, String::from("test")));
/// ```
#[derive(Debug, PartialEq)]
pub struct SpotifyCallback {
    /// An authorization code that can be exchanged for an access token.
    pub(crate) code: Option<String>,
    /// The reason authorization failed.
    pub(crate) error: Option<String>,
    /// The value of the ``state`` parameter supplied in the request.
    pub(crate) state: String,
}

/// Implementation of FromStr for Spotify Callback URLs.
///
/// # Example
///
/// ```
/// # use spotify_oauth::SpotifyCallback;
/// # use std::str::FromStr;
/// // Create a new spotify callback object using the callback url given by the authorization process.
/// // This object can then be converted into the token needed for the application.
/// let callback = SpotifyCallback::from_str("https://example.com/callback?code=NApCCgBkWtQ&state=test").unwrap();
/// # assert_eq!(callback, SpotifyCallback::new(Some("NApCCgBkWtQ".to_string()), None, String::from("test")));
/// ```
impl FromStr for SpotifyCallback {
    type Err = error::SpotifyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(s).context(UrlError)?;
        let parsed: Vec<(String, String)> = url
            .query_pairs()
            .map(|x| (x.0.into_owned(), x.1.into_owned()))
            .collect();

        let has_state = parsed.iter().any(|x| x.0 == "state");
        let has_response = parsed.iter().any(|x| x.0 == "error" || x.0 == "code");

        if !has_state && !has_response {
            return Err(SpotifyError::CallbackFailure {
                context: "Does not contain any state or response type query parameters.",
            });
        } else if !has_state {
            return Err(SpotifyError::CallbackFailure {
                context: "Does not contain any state type query parameters.",
            });
        } else if !has_response {
            return Err(SpotifyError::CallbackFailure {
                context: "Does not contain any response type query parameters.",
            });
        }

        let state = match parsed.iter().find(|x| x.0 == "state") {
            None => ("state".to_string(), "".to_string()),
            Some(x) => x.clone(),
        };

        let response = match parsed.iter().find(|x| x.0 == "error" || x.0 == "code") {
            None => ("error".to_string(), "access_denied".to_string()),
            Some(x) => x.clone(),
        };

        if response.0 == "code" {
            return Ok(Self {
                code: Some(response.1),
                error: None,
                state: state.1,
            });
        } else if response.0 == "error" {
            return Ok(Self {
                code: None,
                error: Some(response.1),
                state: state.1,
            });
        }

        Err(SpotifyError::CallbackFailure {
            context: "Does not contain any state or response type query parameters.",
        })
    }
}

/// Conversion and helper functions for SpotifyCallback.
impl SpotifyCallback {
    /// Create a new Spotify Callback object with given values.
    ///
    /// # Example
    ///
    /// ```
    /// # use spotify_oauth::SpotifyCallback;
    /// // Create a new spotify callback object using the new function.
    /// // This object can then be converted into the token needed for the application.
    /// let callback = SpotifyCallback::new(Some("NApCCgBkWtQ".to_string()), None, String::from("test"));
    /// ```
    pub fn new(code: Option<String>, error: Option<String>, state: String) -> Self {
        Self { code, error, state }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_callback_code() {
        let url = String::from("http://localhost:8888/callback?code=AQD0yXvFEOvw&state=sN");

        assert_eq!(
            SpotifyCallback::from_str(&url).unwrap(),
            SpotifyCallback::new(Some("AQD0yXvFEOvw".to_string()), None, "sN".to_string())
        );
    }

    #[test]
    fn test_parse_callback_error() {
        let url = String::from("http://localhost:8888/callback?error=access_denied&state=sN");

        assert_eq!(
            SpotifyCallback::from_str(&url).unwrap(),
            SpotifyCallback::new(None, Some("access_denied".to_string()), "sN".to_string())
        );
    }

    #[test]
    fn test_invalid_response_parse() {
        let url = String::from("http://localhost:8888/callback?state=sN");

        assert_eq!(
            SpotifyCallback::from_str(&url).unwrap_err().to_string(),
            "Callback URL parsing failure: Does not contain any response type query parameters."
        );
    }

    #[test]
    fn test_invalid_parse() {
        let url = String::from("http://localhost:8888/callback");

        assert_eq!(
            SpotifyCallback::from_str(&url).unwrap_err().to_string(),
            "Callback URL parsing failure: Does not contain any state or response type query parameters."
        );
    }
}
