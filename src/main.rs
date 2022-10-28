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
    /// Play the music if it was previously paused
    Play,
    /// Pause the music if it was previously playing
    Pause,
    /// Toggle the state of the music: playing or paused
    Toggle,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let cli = Cli::parse();

    if let Some(ref command) = cli.command {
        match command {
            Commands::Title => println!("{}", get_media().await.title),
            Commands::Artist => println!("{}", get_media().await.artist),
            Commands::Position => println!("{}", get_media().await.position),
            Commands::Duration => println!("{}", get_media().await.duration),
            _ => println!(
                "{}",
                match handle_commands(&cli.command.unwrap()).await {
                    Ok(stuff) => stuff.to_string(),
                    Err(_) => "Failed.".to_owned(),
                }
            ),
        }
    } else {
        println!("{}", get_media().await);
    }
    Ok(())
}

async fn get_session() -> Result<GlobalSystemMediaTransportControlsSession, windows::core::Error> {
    let mp = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.await?;
    let current_session = mp.GetCurrentSession()?;
    Ok(current_session)
}

async fn handle_commands(cmd: &Commands) -> Result<bool, windows::core::Error> {
    let current_session = get_session().await.unwrap();
    
    match cmd {
        Commands::Pause => match current_session.TryPauseAsync() {
            Ok(res) => res.await,
            Err(err) => Err(err),
        },
        Commands::Play => match current_session.TryPlayAsync() {
            Ok(res) => res.await,
            Err(err) => Err(err),
        },
        Commands::Toggle => match current_session.TryTogglePlayPauseAsync() {
            Ok(res) => res.await,
            Err(err) => Err(err),
        },
        _ => todo!(),
    }
}

/// Just a nice little wrapper
async fn get_media() -> MediaInfo {
    get_media_info()
        .await
        .unwrap_or_else(|_| MediaInfo::empty())
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
