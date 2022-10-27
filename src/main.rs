pub mod media_info;

pub use crate::media_info::MediaInfo;
use clap::{Parser, Subcommand};
use human_repr::HumanDuration;
use windows::Media::Control::GlobalSystemMediaTransportControlsSession;
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

    match cli.command {
        Some(case) => match case {
            Commands::Title => {
                let playing = match get_title().await {
                    Ok(song) => song,
                    Err(_) => MediaInfo::empty(),
                };
                println!("{}", playing.title)
            }
            Commands::Artist => {
                let playing = match get_artist().await {
                    Ok(song) => song,
                    Err(_) => MediaInfo::empty(),
                };
                println!("{}", playing.artist)
            }
            Commands::Position => {
                let playing = match get_position().await {
                    Ok(song) => song,
                    Err(_) => MediaInfo::empty(),
                };
                println!("{}", playing.position)
            }
        },
        None => {
            let playing = match get_media_info().await {
                Ok(song) => song,
                Err(_) => MediaInfo::empty(),
            };
            println!("{}", playing)
        } // Default
    }
    Ok(())
}

async fn get_session() -> Result<GlobalSystemMediaTransportControlsSession, windows::core::Error> {
    let mp = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.await?;
    let current_session = mp.GetCurrentSession()?;
    Ok(current_session)
}

async fn get_media_info() -> Result<MediaInfo, windows::core::Error> {
    let current_session = get_session().await?;
    let info = current_session.TryGetMediaPropertiesAsync()?.await?;
    let title = info.Title()?;
    let artist = info.Artist()?;
    let timeline = current_session.GetTimelineProperties()?;
    let position = timeline.Position()?;
    // Return song title
    Ok(MediaInfo {
        title: title.to_string(),
        artist: artist.to_string(),
        position: ((position.Duration / 10_i64.pow(7)).human_duration()),
    })
}

async fn get_artist() -> Result<MediaInfo, windows::core::Error> {
    let current_session = get_session().await?;
    let info = current_session.TryGetMediaPropertiesAsync()?.await?;
    let artist = info.Artist()?;
    // Return song title
    Ok(MediaInfo {
        title: "".to_owned(),
        artist: artist.to_string(),
        position: 0.human_duration(),
    })
}

async fn get_position() -> Result<MediaInfo, windows::core::Error> {
    let current_session = get_session().await?;
    let timeline = current_session.GetTimelineProperties()?;
    let position = timeline.Position()?;
    let base: i64 = 10;
    // Return song title
    Ok(MediaInfo {
        title: "".to_owned(),
        artist: "".to_owned(),
        position: ((position.Duration / base.pow(7)).human_duration()),
    })
}

async fn get_title() -> Result<MediaInfo, windows::core::Error> {
    let current_session = get_session().await?;
    let info = current_session.TryGetMediaPropertiesAsync()?.await?;
    let title = info.Title()?;
    // Return song title
    Ok(MediaInfo {
        title: title.to_string(),
        artist: "".to_owned(),
        position: 0.human_duration(),
    })
}
