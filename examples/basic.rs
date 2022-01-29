use dotenv::dotenv;
use spotify_oauth::{
    convert_callback_into_token, generate_random_string, SpotifyAuth, SpotifyCallback, SpotifyScope,
};
use std::{env, error::Error, io::stdin, str::FromStr};
use url::Url;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // Load local .env file.
    dotenv().ok();

    // Setup Spotify Auth
    let response_type = "code".to_string();
    let scope = vec![SpotifyScope::Streaming];
    let show_dialog = false;
    let client_id = env::var("SPOTIFY_CLIENT_ID").unwrap();
    let client_secret = env::var("SPOTIFY_CLIENT_SECRET").unwrap();
    let redirect_uri = Url::parse(&env::var("SPOTIFY_REDIRECT_URI").unwrap()).unwrap();

    // Create a state value of length 20
    let state = generate_random_string(20);

    let auth = SpotifyAuth {
        client_id,
        client_secret,
        response_type,
        redirect_uri,
        state,
        scope,
        show_dialog,
    };
    let auth_url = auth.authorize_url()?;

    // Open the auth URL in the default browser of the user.
    open::that(auth_url)?;

    println!("Input callback URL:");
    let mut buffer = String::new();
    stdin().read_line(&mut buffer)?;

    let callback = SpotifyCallback::from_str(buffer.trim())?;
    // Convert the given callback URL into a token.
    let token = convert_callback_into_token(
        callback,
        auth.client_id,
        auth.client_secret,
        auth.redirect_uri,
    )
    .await?;

    println!("Token: {:#?}", token);

    Ok(())
}
