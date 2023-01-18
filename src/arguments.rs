
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
   /// Path to the .mp3
   pub path: PathBuf,

   /// The Spotify track ID to fetch metadata from. If not provided,
   /// an attempt to extrapolate it from the filename will be made.
   #[arg(short, long)]
   pub track_id: Option<String>,

   /// Write the new tags to the file
   #[arg(short, long)]
   pub write: Option<bool>
}
