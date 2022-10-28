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
    Toggle,
    Play,
    Pause,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let cli = Cli::parse();

    match cli.command {
        Some(ref case) => match case {
            Commands::Title => println!("{}", get_media().await),
            Commands::Artist => println!("{}", get_media().await),
            Commands::Position => println!("{}", get_media().await),
            _ => println!(
                "{}",
                match handle_commands(&cli.command.unwrap()).await {
                    Ok(stuff) => stuff.to_string().to_owned(),
                    Err(_) => "Failed.".to_owned(),
                }
            ),
        },
        None => println!(
            "{}",
            get_media_info()
                .await
                .unwrap_or_else(|_| MediaInfo::empty())
        ),
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
        _ => match current_session.TryTogglePlayPauseAsync() {
            Ok(res) => res.await,
            Err(err) => Err(err),
        },
    }
}

async fn get_media() -> MediaInfo {
    // Literally just a nice little wrapper
    get_media_info()
        .await
        .unwrap_or_else(|_| MediaInfo::empty())
}

async fn get_media_info() -> Result<MediaInfo, windows::core::Error> {
    let current_session = get_session().await?;

    let properties = current_session.TryGetMediaPropertiesAsync()?.await?;
    let title = properties.Title()?;
    let artist = properties.Artist()?;

    let timeline = current_session.GetTimelineProperties()?;
    let position = timeline.Position()?;

    Ok(MediaInfo {
        title: title.to_string(),
        artist: artist.to_string(),
        position: ((position.Duration / 10_i64.pow(7)).human_duration()),
    })
}
