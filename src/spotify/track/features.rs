use std::collections::HashMap;

use rspotify::model::AudioFeatures;
use id3::frame;

pub fn create_frames(features: &AudioFeatures) -> Vec<frame::ExtendedText> {
  vec![
    frame::ExtendedText {
        description: "acousticness".to_string(),
        value: features.acousticness.to_string()
    },
    frame::ExtendedText {
        description: "danceability".to_string(),
        value: features.danceability.to_string()
    },
    frame::ExtendedText {
        description: "energy".to_string(),
        value: features.energy.to_string()
    },
    frame::ExtendedText {
        description: "instrumentalness".to_string(),
        value: features.instrumentalness.to_string()
    },
    frame::ExtendedText {
        description: "liveness".to_string(),
        value: features.liveness.to_string()
    },
    frame::ExtendedText {
        description: "speechiness".to_string(),
        value: features.speechiness.to_string()
    },
    frame::ExtendedText {
        description: "valence".to_string(),
        value: features.valence.to_string()
    },
  ]
}

pub fn get_initial_key(features: &AudioFeatures) -> String {
    let key_map = HashMap::from([
      (-1, "o"),
      ( 0, "C"),
      ( 1, "Cb"),
      ( 2, "D"),
      ( 3, "Db"),
      ( 4, "E"),
      ( 5, "F"),
      ( 6, "Fb"),
      ( 7, "G"),
      ( 8, "Gb"),
      ( 9, "A"),
      ( 10, "Ab"),
      ( 11, "B"),
    ]);

    key_map.get(&features.key).unwrap().to_string()
}