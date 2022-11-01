mod cleanup_timespan;
mod media_session;
mod media_status;

use crate::media_session::MediaSession;
use crate::media_status::MediaStatus;
use clap::{Parser, Subcommand};
#[derive(Debug, Parser)]
#[command(
    name = clap::crate_name!(),
    author = clap::crate_authors!(),
    version = clap::crate_version!(),
    propagate_version = true,
    about = clap::crate_description!()
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Print the title of the media
    Title,
    /// Print the artist of the media
    Artist,
    /// Print the current position in the media
    /// (may be delayed for a few seconds due to WinRT restrictions)
    Position,
    /// Print the length of the media
    Duration,
    /// Print the status of the media
    Status,
    /// Play the media if it was previously paused
    Play,
    /// Pause the media if it was previously playing
    Pause,
    /// Toggle the state of the media between playing and paused
    Toggle,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let cli = Cli::parse();

    if let Ok(media_session) = MediaSession::new().await {
        if let Some(command) = cli.command {
            match command {
                // commands that fetch the state
                Commands::Title => println!("{}", media_session.get_title()),
                Commands::Artist => println!("{}", media_session.get_artist()),
                Commands::Position => println!("{}", media_session.get_position()),
                Commands::Duration => println!("{}", media_session.get_duration()),
                Commands::Status => println!("{:#?}", media_session.get_status()),
                // commands that modify the state
                Commands::Play => println!("{}", media_session.play()),
                Commands::Pause => println!("{}", media_session.pause()),
                Commands::Toggle => println!("{}", media_session.toggle()),
            }
        } else {
            println!("{}", media_session);
        }
    }

    Ok(())
}
