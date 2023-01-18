use id3::{Tag, TagLike, Frame, Version};
use rspotify::prelude::*;
use colored::Colorize;
use std::process;
use clap::Parser;
use arguments::Args;

mod arguments;
mod spotify;
mod output;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let file_path = args.path.clone();

    if ! file_path.exists() {
        eprintln!("Invalid file path!");
        process::exit(1)
    }

    // Create Spotify API client
    let spotify = spotify::client::create().await;

    // Fetch track metadata and audio features
    let id = spotify::id::from(args.path, args.track_id);
    let track = spotify.track(id.clone()).await.unwrap();
    let track_features = spotify.track_features(id.clone()).await.unwrap();

    let mut tag = Tag::new();

    tag.add_frame(Frame::link("WORS", track.to_owned().id.unwrap().to_string()));
    tag.add_frame(Frame::text("TIT2", track.name.clone()));
    tag.add_frame(Frame::text("TALB", track.album.name.clone()));
    tag.add_frame(Frame::text("TRCK", track.track_number.to_string()));
    tag.add_frame(Frame::text("TBPM", track_features.to_owned().tempo.to_string()));
    tag.add_frame(Frame::text("TPE1", spotify::track::metadata::get_artists(&track)));
    tag.add_frame(Frame::text("TDRL", spotify::track::metadata::get_release_year(&track)));
    tag.add_frame(Frame::text("TCON", spotify::track::metadata::get_genres(&spotify, &track).await));
    tag.add_frame(Frame::text("TKEY", spotify::track::features::get_initial_key(&track_features)));

    // Create IDv3 frames for cover art and audio features
    let track_picture_frame = spotify::track::picture::create_frame(&track).await;
    let track_feature_frames = spotify::track::features::create_frames(&track_features);

    tag.remove_all_pictures();
    tag.add_frame(track_picture_frame);

    for frame in track_feature_frames.iter() {
        tag.add_frame(frame.to_owned());
    }

    // Print a summary
    output::print(&tag);

    // Write tag to file if the --write flag was provided
    if args.write.unwrap_or(false) {
        tag.write_to_path(&file_path, Version::Id3v23).unwrap();

        let message = format!("Successfully wrote IDv3 tag to {}!", file_path.as_os_str().to_str().unwrap());
        println!("{}", message.green());
    } else {
        let message = "File was left untouched. To write changes, run with the --write flag!";
        println!("{}", message.yellow());
    }
}
