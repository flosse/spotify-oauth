use chrono::{DateTime, Utc};
use rand::{self, Rng};

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
