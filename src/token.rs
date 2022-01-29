use crate::SpotifyScope;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::str::FromStr;

/// The Spotify Token object.
///
/// This struct follows the parameters given at [this](https://developer.spotify.com/documentation/general/guides/authorization-guide/ "Spotify Auth Documentation") link.
///
/// This object can only be formed from a correct Spotify Callback object.
///
/// # Example
///
/// ```no_run
/// # use spotify_oauth::{convert_callback_into_token, SpotifyAuth, SpotifyScope, SpotifyCallback};
/// # use std::str::FromStr;
/// # #[async_std::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
/// // Create a new Spotify auth object.
/// let auth = SpotifyAuth::new("00000000000".into(), "secret".into(), "code".into(), "http://localhost:8000/callback".into(), vec![SpotifyScope::Streaming], false);   
///
/// // Create a new Spotify token object using the callback object given by the authorization process.
/// let callback = SpotifyCallback::from_str("https://example.com/callback?code=NApCCgBkWtQ&state=test").unwrap();
/// convert_callback_into_token(callback, auth.client_id, auth.client_secret, auth.redirect_uri).await.unwrap();
/// # Ok(()) }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SpotifyToken {
    /// An access token that can be provided in subsequent calls, for example to Spotify Web API services.
    pub access_token: String,
    /// How the access token may be used.
    pub token_type: String,
    /// A Vec of scopes which have been granted for this ``access_token``.
    #[serde(deserialize_with = "deserialize_scope_field")]
    pub scope: Vec<SpotifyScope>,
    /// The time period (in seconds) for which the access token is valid.
    pub expires_in: u32,
    /// The timestamp for which the token will expire at.
    pub expires_at: Option<i64>,
    /// A token that can be sent to the Spotify Accounts service in place of an authorization code to request a new ``access_token``.
    pub refresh_token: String,
}

/// Custom parsing function for converting a vector of string scopes into SpotifyScope Enums using Serde.
/// If scope is empty it will return an empty vector.
fn deserialize_scope_field<'de, D>(de: D) -> Result<Vec<SpotifyScope>, D::Error>
where
    D: Deserializer<'de>,
{
    let result: Value = Deserialize::deserialize(de)?;
    match result {
        Value::String(ref s) => {
            let split: Vec<&str> = s.split_whitespace().collect();
            let mut parsed: Vec<SpotifyScope> = Vec::new();

            for x in split {
                parsed.push(SpotifyScope::from_str(x).unwrap());
            }

            Ok(parsed)
        }
        _ => Ok(vec![]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::datetime_to_timestamp;

    #[test]
    fn test_token_parse() {
        let token_json = r#"{
           "access_token": "NgCXRKDjGUSKlfJODUjvnSUhcOMzYjw",
           "token_type": "Bearer",
           "scope": "user-read-private user-read-email",
           "expires_in": 3600,
           "refresh_token": "NgAagAHfVxDkSvCUm_SHo"
        }"#;

        let mut token: SpotifyToken = serde_json::from_str(token_json).unwrap();
        let timestamp = datetime_to_timestamp(token.expires_in);
        token.expires_at = Some(timestamp);

        assert_eq!(
            SpotifyToken {
                access_token: "NgCXRKDjGUSKlfJODUjvnSUhcOMzYjw".to_string(),
                token_type: "Bearer".to_string(),
                scope: vec![SpotifyScope::UserReadPrivate, SpotifyScope::UserReadEmail],
                expires_in: 3600,
                expires_at: Some(timestamp),
                refresh_token: "NgAagAHfVxDkSvCUm_SHo".to_string()
            },
            token
        );
    }
}
