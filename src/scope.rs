use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

/// Spotify Scopes for the API.
/// This enum implements FromStr and ToString / Display through strum.
///
/// All the Spotify API scopes can be found [here](https://developer.spotify.com/documentation/general/guides/scopes/ "Spotify Scopes").
///
/// # Example
///
/// ```
/// # use spotify_oauth::SpotifyScope;
/// # use std::str::FromStr;
/// // Convert string into scope.
/// let scope = SpotifyScope::from_str("streaming").unwrap();
/// # assert_eq!(scope, SpotifyScope::Streaming);
/// // It can also convert the scope back into a string.
/// let scope = scope.to_string();
/// # assert_eq!(scope, "streaming");
/// ```
#[derive(EnumString, Serialize, Deserialize, Display, Debug, Clone, PartialEq)]
pub enum SpotifyScope {
    #[strum(serialize = "user-read-recently-played")]
    UserReadRecentlyPlayed,
    #[strum(serialize = "user-top-read")]
    UserTopRead,

    #[strum(serialize = "user-library-modify")]
    UserLibraryModify,
    #[strum(serialize = "user-library-read")]
    UserLibraryRead,

    #[strum(serialize = "playlist-read-private")]
    PlaylistReadPrivate,
    #[strum(serialize = "playlist-modify-public")]
    PlaylistModifyPublic,
    #[strum(serialize = "playlist-modify-private")]
    PlaylistModifyPrivate,
    #[strum(serialize = "playlist-read-collaborative")]
    PlaylistReadCollaborative,

    #[strum(serialize = "user-read-email")]
    UserReadEmail,
    #[strum(serialize = "user-read-birthdate")]
    UserReadBirthDate,
    #[strum(serialize = "user-read-private")]
    UserReadPrivate,

    #[strum(serialize = "user-read-playback-state")]
    UserReadPlaybackState,
    #[strum(serialize = "user-modify-playback-state")]
    UserModifyPlaybackState,
    #[strum(serialize = "user-read-currently-playing")]
    UserReadCurrentlyPlaying,

    #[strum(serialize = "app-remote-control")]
    AppRemoteControl,
    #[strum(serialize = "streaming")]
    Streaming,

    #[strum(serialize = "user-follow-read")]
    UserFollowRead,
    #[strum(serialize = "user-follow-modify")]
    UserFollowModify,
}
