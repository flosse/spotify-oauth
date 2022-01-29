//! # Spotify OAuth
//!
//! An implementation of the Spotify Authorization Code Flow in Rust.
//!
//! # Basic Example
//!
//! ```no_run
//! use std::{io::stdin, str::FromStr, error::Error};
//! use spotify_oauth::{SpotifyAuth, SpotifyCallback, SpotifyScope};
//!
//! #[async_std::main]
//! async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
//!
//!     // Setup Spotify Auth URL
//!     let auth = SpotifyAuth::new_from_env("code".into(), vec![SpotifyScope::Streaming], false);
//!     let auth_url = auth.authorize_url()?;
//!
//!     // Open the auth URL in the default browser of the user.
//!     open::that(auth_url)?;
//!
//!     println!("Input callback URL:");
//!     let mut buffer = String::new();
//!     stdin().read_line(&mut buffer)?;
//!
//!     // Convert the given callback URL into a token.
//!     let token = SpotifyCallback::from_str(buffer.trim())?
//!         .convert_into_token(auth.client_id, auth.client_secret, auth.redirect_uri).await?;
//!
//!     println!("Token: {:#?}", token);
//!
//!     Ok(())
//! }
//! ```

mod auth;
mod callback;
mod error;
mod scope;
mod token;
mod util;

use crate::error::*;

pub use crate::{auth::*, callback::*, scope::*, token::*, util::*};

const SPOTIFY_AUTH_URL: &str = "https://accounts.spotify.com/authorize";
const SPOTIFY_TOKEN_URL: &str = "https://accounts.spotify.com/api/token";
