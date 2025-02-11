use crate::{generate_random_string, SpotifyResult, SpotifyScope, UrlError, SPOTIFY_AUTH_URL};
use snafu::ResultExt;
use std::string::ToString;
use url::Url;

/// Spotify Authentication
///
/// This struct follows the parameters given at [this](https://developer.spotify.com/documentation/general/guides/authorization-guide/ "Spotify Auth Documentation") link.
/// ```
pub struct SpotifyAuth {
    /// The Spotify Application Client ID
    pub client_id: String,
    /// The Spotify Application Client Secret
    pub client_secret: String,
    /// Required by the Spotify API.
    pub response_type: String,
    /// The URI to redirect to after the user grants or denies permission.
    pub redirect_uri: Url,
    /// A random generated string that can be useful for correlating requests and responses.
    pub state: String,
    /// Vec of Spotify Scopes.
    pub scope: Vec<SpotifyScope>,
    /// Whether or not to force the user to approve the app again if they’ve already done so.
    pub show_dialog: bool,
}

/// Conversion and helper functions for SpotifyAuth.
impl SpotifyAuth {
    /// Generate a new SpotifyAuth structure from values in memory.
    ///
    /// This function loads ``SPOTIFY_CLIENT_ID`` and ``SPOTIFY_REDIRECT_ID`` from values given in
    /// function parameters.
    ///
    /// This function also automatically generates a state value of length 20 using a random string generator.
    ///
    /// # Example
    ///
    /// ```
    /// # use spotify_oauth::{SpotifyAuth, SpotifyScope};
    /// // SpotifyAuth with the scope "Streaming".
    /// let auth = SpotifyAuth::new("00000000000".into(), "secret".into(), "code".into(), "http://localhost:8000/callback".into(), vec![SpotifyScope::Streaming], false);
    /// # assert_eq!(auth.scope_into_string(), "streaming");
    /// ```
    pub fn new(
        client_id: String,
        client_secret: String,
        response_type: String,
        redirect_uri: String,
        scope: Vec<SpotifyScope>,
        show_dialog: bool,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            response_type,
            redirect_uri: Url::parse(&redirect_uri).context(UrlError).unwrap(),
            state: generate_random_string(20),
            scope,
            show_dialog,
        }
    }

    /// Concatenate the scope vector into a string needed for the authorization URL.
    ///
    /// # Example
    ///
    /// ```
    /// # use spotify_oauth::{SpotifyAuth, SpotifyScope};
    /// // Default SpotifyAuth with the scope "Streaming".
    /// let auth = SpotifyAuth::new("00000000000".into(), "secret".into(), "code".into(), "http://localhost:8000/callback".into(), vec![SpotifyScope::Streaming], false);
    /// # assert_eq!(auth.scope_into_string(), "streaming");
    /// ```
    pub fn scope_into_string(&self) -> String {
        self.scope
            .iter()
            .map(|x| x.clone().to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    /// Convert the SpotifyAuth struct into the authorization URL.
    ///
    /// More information on this URL can be found [here](https://developer.spotify.com/documentation/general/guides/authorization-guide/ "Spotify Auth Documentation").
    ///
    /// # Example
    ///
    /// ```
    /// # use spotify_oauth::{SpotifyAuth, SpotifyScope};
    /// // Default SpotifyAuth with the scope "Streaming" converted into the authorization URL.
    /// let auth = SpotifyAuth::new("00000000000".into(), "secret".into(), "code".into(), "http://localhost:8000/callback".into(), vec![SpotifyScope::Streaming], false)
    ///     .authorize_url().unwrap();
    /// ```
    pub fn authorize_url(&self) -> SpotifyResult<String> {
        let mut url = Url::parse(SPOTIFY_AUTH_URL).context(UrlError)?;

        url.query_pairs_mut()
            .append_pair("client_id", &self.client_id)
            .append_pair("response_type", &self.response_type)
            .append_pair("redirect_uri", self.redirect_uri.as_str())
            .append_pair("state", &self.state)
            .append_pair("scope", &self.scope_into_string())
            .append_pair("show_dialog", &self.show_dialog.to_string());

        Ok(url.to_string())
    }
}
