use std::path::PathBuf;
use rspotify::model::TrackId;

pub fn validate<'a>(src: String) -> Result<TrackId<'a>, &'a str> {
    if src.len() != 22 {
        return Err("Expected an ID with a length of 22")
    }

    for c in src.as_bytes() {
        match c {
            b'0'..=b'9' => c - b'0',
            b'a'..=b'z' => c - b'a' + 10,
            b'A'..=b'Z' => c - b'A' + 36,
            _ => return Err("Expected a Base-64 encoded ID"),
        };
    }

    let id = TrackId::from_id(src.clone());

    Ok(id.unwrap())
}

pub fn from<'a>(filename: PathBuf, track_id: Option<String>) -> TrackId<'a> {
    let filename = filename.file_stem();

    let foo = match track_id {
        Some(id) => validate(id),
        None => {
            match filename {
                Some(id) => validate(id.to_str().unwrap().to_string()),
                None => panic!("Unable to parse spotify track id from file!")
            }
        }
    };

    match foo {
        Ok(id) => id,
        Err(error) => panic!("Unable to parse spotify ID: {error}",)
    }
}

