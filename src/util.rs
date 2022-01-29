use crate::{error::*, SpotifyCallback, SpotifyToken};
use chrono::{DateTime, Utc};
use rand::{self, Rng};
use snafu::ResultExt;
use std::collections::HashMap;
use url::Url;

const SPOTIFY_TOKEN_URL: &str = "https://accounts.spotify.com/api/token";

/// Convert date and time to a unix timestamp.
///
/// # Example
///
/// ```no_run
/// // Uses elapsed seconds and the current timestamp to return a timestamp offset by the seconds.
/// # use spotify_oauth::datetime_to_timestamp;
/// let timestamp = datetime_to_timestamp(3600);
/// ```
pub fn datetime_to_timestamp(elapsed: u32) -> i64 {
    let utc: DateTime<Utc> = Utc::now();
    utc.timestamp() + i64::from(elapsed)
}

/// Generate a random alphanumeric string with a given length.
///
/// # Example
///
/// ```no_run
/// // Uses elapsed seconds and the current timestamp to return a timestamp offset by the seconds.
/// # use spotify_oauth::generate_random_string;
/// let timestamp = generate_random_string(20);
/// ```
pub fn generate_random_string(length: usize) -> String {
    String::from_utf8_lossy(
        &rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(length)
            .collect::<Vec<_>>(),
    )
    .to_string()
}

/// Converts the Spotify Callback object into a Spotify Token object.
pub async fn convert_callback_into_token(
    callback: SpotifyCallback,
    client_id: String,
    client_secret: String,
    redirect_uri: Url,
) -> SpotifyResult<SpotifyToken> {
    let mut payload: HashMap<String, String> = HashMap::new();
    payload.insert("grant_type".to_owned(), "authorization_code".to_owned());
    payload.insert(
        "code".to_owned(),
        match callback.code {
            None => {
                return Err(SpotifyError::TokenFailure {
                    context: "Spotify callback code failed to parse.",
                })
            }
            Some(x) => x,
        },
    );
    payload.insert("redirect_uri".to_owned(), redirect_uri.to_string());

    // Form authorisation header.
    let auth_value = base64::encode(&format!("{}:{}", client_id, client_secret));

    // POST the request.
    let mut response = surf::post(SPOTIFY_TOKEN_URL)
        .header("Authorization", format!("Basic {}", auth_value))
        .body(surf::Body::from_form(&payload).unwrap())
        .send()
        .await
        .map_err(|err| SpotifyError::SurfError {
            context: format!("{err:?}"),
        })?;

    // Read the response body.
    let buf = response.body_string().await.unwrap();

    if response.status().is_success() {
        let mut token: SpotifyToken = serde_json::from_str(&buf).context(SerdeError)?;
        token.expires_at = Some(datetime_to_timestamp(token.expires_in));

        return Ok(token);
    }

    Err(SpotifyError::TokenFailure {
        context: "Failed to convert callback into token",
    })
}
