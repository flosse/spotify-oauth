use std::{env, error::Error, io::stdin, str::FromStr};

use dotenv::dotenv;

use spotify_oauth::{
    convert_callback_into_token, generate_random_string, AppClient, SpotifyAuth, SpotifyCallback,
    SpotifyScope, SurfClient,
};

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // Load local .env file.
    dotenv().ok();

    // Setup Spotify Auth
    let response_type = "code".to_string();
    let scopes = vec![SpotifyScope::Streaming];
    let show_dialog = false;

    let id = env::var("SPOTIFY_CLIENT_ID").unwrap();
    let secret = env::var("SPOTIFY_CLIENT_SECRET").unwrap();
    let app_client = AppClient { id, secret };

    let redirect_uri = env::var("SPOTIFY_REDIRECT_URI").unwrap();

    // Create a state value of length 20
    let state = generate_random_string(20);

    let auth = SpotifyAuth {
        app_client,
        response_type,
        redirect_uri,
        state,
        scopes,
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
    let token =
        convert_callback_into_token(SurfClient, callback, &auth.app_client, auth.redirect_uri)
            .await?;

    println!("Token: {:#?}", token);

    Ok(())
}
