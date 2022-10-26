pub mod media_info;

pub use crate::media_info::MediaInfo;
use clap::{Parser, Subcommand};
use human_repr::HumanDuration;
use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;

#[derive(Parser)]
#[command(name = "now-playing")]
#[command(author = "Dukk <acedaksh07@gmail.com>")]
#[command(version = "1.0.0")]
#[command(about = "Gets information about currently playing media on Windows.")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Title,
    Artist,
    Position,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let cli = Cli::parse();

    let playing = match get_media_info().await {
        Ok(song) => song,
        Err(_) => MediaInfo {
            title: "No Song Playing".to_owned(),
            artist: "".to_owned(),
            position: 0.human_duration(),
        }, // No media playing
    };

    match cli.command {
        Some(case) => match case {
            Commands::Title => println!("{}", playing.title),
            Commands::Artist => println!("{}", playing.artist),
            Commands::Position => println!("{}", playing.position),
        },
        None => println!("{}", playing), // Default
    }
    Ok(())
}

async fn get_media_info() -> Result<MediaInfo, windows::core::Error> {
    let mp = match GlobalSystemMediaTransportControlsSessionManager::RequestAsync() {
        // Gets the async TransportControlsSessionManager so that we can work with it
        Ok(stuff) => match stuff.await {
            Ok(more_stuff) => more_stuff,
            Err(err) => return Err(err),
        },
        Err(err) => return Err(err),
    };
    let current_session = match mp.GetCurrentSession() {
        // Gets current media player
        Ok(stuff) => stuff,
        Err(err) => return Err(err),
    };
    let timeline = match current_session.GetTimelineProperties() {
        // Gets current media player
        Ok(stuff) => stuff,
        Err(err) => return Err(err),
    };
    let info = match current_session.TryGetMediaPropertiesAsync() {
        // Get media properties
        Ok(stuff) => match stuff.await {
            Ok(stuf) => stuf,
            Err(err) => return Err(err),
        },
        Err(err) => return Err(err),
    };
    let title = match info.Title() {
        Ok(stuff) => stuff,
        Err(err) => return Err(err),
    };
    let artist = match info.Artist() {
        Ok(stuff) => stuff,
        Err(err) => return Err(err),
    };
    let position = match timeline.Position() {
        Ok(stuff) => stuff,
        Err(err) => return Err(err),
    };
    let base: i64 = 10;
    // Return song title
    Ok(MediaInfo {
        title: title.to_string(),
        artist: artist.to_string(),
        position: ((position.Duration / base.pow(7)).human_duration()),
    })
}
