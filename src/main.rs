pub mod media_info;

pub use crate::media_info::MediaInfo;
use clap::{Parser, Subcommand};
use human_repr::HumanDuration;
use human_repr::HumanDurationData;
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
                if let Ok(title) = get_title().await {
                    println!("{}", title)
                } else {
                    println!("No Music Playing")
                }
            }
            Commands::Artist => {
                if let Ok(artist) = get_artist().await {
                    println!("{}", artist)
                } else {
                    println!("No Artist")
                }
            }
            Commands::Position => {
                if let Ok(position) = get_position().await {
                    println!("{}", position)
                } else {
                    println!("0ns")
                }
            }
        },
        None => {
            let playing = match get_media_info().await {
                Ok(song) => song,
                Err(_) => MediaInfo {
                    title: "No Music Playing".to_owned(),
                    artist: "".to_owned(),
                    position: 0_i64.human_duration(),
                },
            };
            println!("{}", playing)
        }
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
    Ok(MediaInfo {
        title: title.to_string(),
        artist: artist.to_string(),
        position: ((position.Duration / 10_i64.pow(7)).human_duration()),
    })
}

async fn get_artist() -> Result<String, windows::core::Error> {
    let current_session = get_session().await?;
    let info = current_session.TryGetMediaPropertiesAsync()?.await?;
    let artist = info.Artist()?;
    Ok(artist.to_string())
}

async fn get_position() -> Result<HumanDurationData, windows::core::Error> {
    let current_session = get_session().await?;
    let timeline = current_session.GetTimelineProperties()?;
    let position = timeline.Position()?;
    let base: i64 = 10;
    Ok((position.Duration / base.pow(7)).human_duration())
}

async fn get_title() -> Result<String, windows::core::Error> {
    let current_session = get_session().await?;
    let info = current_session.TryGetMediaPropertiesAsync()?.await?;
    let title = info.Title()?;
    Ok(title.to_string())
}
