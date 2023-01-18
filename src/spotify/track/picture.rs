use std::str::FromStr;

use hyper::Uri;
use hyper_tls::HttpsConnector;
use id3::frame::{self, PictureType};
use rspotify::model::FullTrack;

pub async fn create_frame(track: &FullTrack) -> frame::Picture {
    let https = HttpsConnector::new();
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);

    let track_image = track.album.images.first().unwrap().clone();
    let uri = Uri::from_str(&track_image.url).unwrap();

    let response = client
        .get(uri)
        .await.unwrap();

    let bytes = hyper::body::to_bytes(response.into_body()).await.unwrap().to_vec();

    frame::Picture {
        mime_type: "image/jpeg".to_string(),
        picture_type: PictureType::CoverFront,
        description: "Cover".to_string(),
        data: bytes,
    }
}