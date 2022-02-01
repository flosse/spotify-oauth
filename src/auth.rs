use std::string::ToString;

use url::Url;

use crate::{generate_random_string, AppClient, SpotifyResult, SpotifyScope, SPOTIFY_AUTH_URL};

/// Spotify Authentication
///
/// This struct follows the parameters given at [this](https://developer.spotify.com/documentation/general/guides/authorization-guide/ "Spotify Auth Documentation") link.
pub struct SpotifyAuth {
    /// The Spotify Application Client ID & Secret
    pub app_client: AppClient,
    /// Required by the Spotify API.
    pub response_type: String,
    /// The URL to redirect to after the user grants or denies permission.
    pub redirect_uri: String,
    /// A random generated string that can be useful for correlating requests and responses.
    pub state: String,
    /// Vec of Spotify Scopes.
    pub scopes: Vec<SpotifyScope>,
    /// Whether or not to force the user to approve the app again if they’ve already done so.
    pub show_dialog: bool,
}

/// Concatenate the scope vector into a string needed for the authorization URL.
pub fn scopes_string(scopes: impl IntoIterator<Item = impl ToString>) -> String {
    scopes
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

/// Conversion and helper functions for SpotifyAuth.
impl SpotifyAuth {
    /// Construct a new SpotifyAuth structure.
    ///
    /// This function also automatically generates a state value of length 20 using a random string generator.
    pub fn new(
        app_client: AppClient,
        response_type: String,
        redirect_uri: String,
        scopes: Vec<SpotifyScope>,
        show_dialog: bool,
    ) -> Self {
        Self {
            app_client,
            response_type,
            redirect_uri,
            state: generate_random_string(20),
            scopes,
            show_dialog,
        }
    }

    /// Concatenate the scope vector into a string needed for the authorization URL.
    pub fn scope_into_string(&self) -> String {
        scopes_string(&self.scopes)
    }

    /// Convert the SpotifyAuth struct into the authorization URL.
    ///
    /// More information on this URL can be found [here](https://developer.spotify.com/documentation/general/guides/authorization-guide/ "Spotify Auth Documentation").
    pub fn authorize_url(&self) -> SpotifyResult<String> {
        let mut url = Url::parse(SPOTIFY_AUTH_URL)?;

        let Self {
            app_client:
                AppClient {
                    id: client_id,
                    secret: _,
                },
            redirect_uri,
            response_type,
            state,
            scopes,
            show_dialog,
        } = self;
        url.query_pairs_mut()
            .append_pair("client_id", client_id)
            .append_pair("response_type", response_type)
            .append_pair("redirect_uri", redirect_uri)
            .append_pair("state", state)
            .append_pair("scope", &scopes_string(scopes))
            .append_pair("show_dialog", &show_dialog.to_string());

        Ok(url.to_string())
    }
}
