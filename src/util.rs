use std::time::{SystemTime, UNIX_EPOCH};

use rand::{self, Rng};

use crate::{error::*, AppClient, HttpClient, SpotifyCallback, SpotifyToken, TokenRequest};

/// Convert date and time to a unix timestamp.
///
/// # Example
///
/// ```no_run
/// // Uses elapsed seconds and the current timestamp to return a timestamp offset by the seconds.
/// # use spotify_oauth::datetime_to_timestamp;
/// let timestamp = datetime_to_timestamp(3600);
/// ```
pub fn datetime_to_timestamp(elapsed_seconds: u32) -> i64 {
    let seconds_since_unix_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    debug_assert!(seconds_since_unix_epoch < i64::MAX as u64);
    seconds_since_unix_epoch as i64 + i64::from(elapsed_seconds)
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
pub async fn convert_callback_into_token<'c, C>(
    http: C,
    callback: SpotifyCallback,
    client_id: &AppClient,
    redirect_uri: String,
) -> SpotifyResult<SpotifyToken>
where
    C: HttpClient<'c>,
{
    let code = match callback.code {
        None => {
            return Err(SpotifyError::TokenFailure {
                context: "Spotify callback code failed to parse.",
            })
        }
        Some(x) => x,
    };

    let auth_request = TokenRequest::new(client_id, code, redirect_uri);
    let buf = http.fetch_token(auth_request).await.map_err(Into::into)?;
    let mut token: SpotifyToken = serde_json::from_value(buf)?;
    token.expires_at = Some(datetime_to_timestamp(token.expires_in));

    Ok(token)
}
