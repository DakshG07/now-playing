pub mod cleanup_timespan;
pub mod media_info;

pub use crate::media_info::MediaInfo;
use clap::{Parser, Subcommand};
use cleanup_timespan::Cleanup;

use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession, GlobalSystemMediaTransportControlsSessionManager,
};

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
    /// Print the title of the song
    Title,
    /// Print the atist of the song
    Artist,
    /// Print the current position in the song
    /// (may be delayed for a few seconds due to winRT restrictions)
    Position,
    /// Print the length of the song
    Duration,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let cli = Cli::parse();

    let playing = get_media_info().await.unwrap_or(MediaInfo::empty());

    if let Some(case) = cli.command {
        match case {
            Commands::Title => println!("{}", playing.title),
            Commands::Artist => println!("{}", playing.artist),
            Commands::Position => println!("{}", playing.position),
            Commands::Duration => println!("{}", playing.duration),
        }
    } else {
        println!("{:#?}", playing);
    }
    Ok(())
}

async fn get_session() -> Result<GlobalSystemMediaTransportControlsSession, windows::core::Error> {
    let mp = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.await?;
    let current_session = mp.GetCurrentSession()?;
    Ok(current_session)
}

async fn toggle_play(cmd: &Commands) {
    let current_session = get_session().await.unwrap();
    match cmd {
        Commands::Pause => {
            match current_session.TryPauseAsync() {
                Ok(res) => println!(
                    "{}",
                    match res.await {
                        Ok(_) => "".to_owned(),
                        Err(_) => "Failed.".to_owned(),
                    }
                ),
                Err(_) => println!("Failed."),
            };
        }
        Commands::Play => {
            match current_session.TryPlayAsync() {
                Ok(res) => println!(
                    "{}",
                    match res.await {
                        Ok(_) => "".to_owned(),
                        Err(_) => "Failed.".to_owned(),
                    }
                ),
                Err(_) => println!("Failed."),
            };
        }
        _ => {
            match current_session.TryTogglePlayPauseAsync() {
                Ok(res) => println!(
                    "{}",
                    match res.await {
                        Ok(_) => "".to_owned(),
                        Err(_) => "Failed.".to_owned(),
                    }
                ),
                Err(_) => println!("Failed."),
            };
        }
    };
}

async fn get_media_info() -> Result<MediaInfo, windows::core::Error> {
    let current_session = get_session().await?;

    let properties = current_session.TryGetMediaPropertiesAsync()?.await?;
    let timeline = current_session.GetTimelineProperties()?;

    Ok(MediaInfo {
        title: properties.Title()?.to_string(),
        artist: properties.Artist()?.to_string(),
        position: timeline.Position()?.cleanup(),
        duration: timeline.EndTime()?.cleanup(),
    })
}
