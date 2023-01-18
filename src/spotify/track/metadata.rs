use futures::future;
use rspotify::{prelude::*, ClientCredsSpotify, model::FullTrack};

pub fn get_artists(track: &FullTrack) -> String {
    let artists: Vec<String> = track.artists.iter()
        .map(|artist| artist.name.clone())
        .collect();

    artists.join("\0")
}

pub fn get_release_year(track: &FullTrack) -> String {
    let date = track.album.release_date.as_ref().unwrap();
    let splits: Vec<&str> = date.split("-").collect();

    splits.first().unwrap().to_string()
}

pub async fn get_genres(spotify: &ClientCredsSpotify, track: &FullTrack) -> String {
    let artist_ids = track.artists.iter()
        .map(|artist| artist.id.clone().unwrap())
        .map(|id| spotify.artist(id));

    let artists = future::join_all(artist_ids).await;

    let mut genres: Vec<String> = artists.iter().flat_map(|artist| {
        match artist {
            Ok(artist) => artist.genres.clone(),
            Err(_) => vec![]
        }
    }).collect();

    genres.sort();
    genres.dedup();
    genres.join("\0")
}