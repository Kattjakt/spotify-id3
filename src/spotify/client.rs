use std::{env, process::exit};
use rspotify::{ClientCredsSpotify, Credentials};

pub async fn create() -> ClientCredsSpotify {
    let client_id = match env::var_os("SPOTIFY_CLIENT_ID") {
        Some(v) => v.into_string().unwrap(),
        None => {
            eprintln!("$SPOTIFY_CLIENT_ID is not set");
            exit(1)
        }
    };

    let client_secret = match env::var_os("SPOTIFY_CLIENT_SECRET") {
        Some(v) => v.into_string().unwrap(),
        None => {
            eprintln!("$SPOTIFY_CLIENT_SECRET is not set");
            exit(1)
        }
    };

    let client = ClientCredsSpotify::new(Credentials {
        id: client_id,
        secret: client_secret.into()
    });

    client.request_token().await.unwrap();

    client
}
