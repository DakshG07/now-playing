mod media_status;
mod media_session;
mod cleanup_timespan;

use crate::media_session::MediaSession;
use crate::media_status::MediaStatus;
use clap::{Parser, Subcommand};

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
    /// Print the status of the song
    Status,
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
                Commands::Pause=> println!("{}", media_session.pause()),
                Commands::Toggle => println!("{}", media_session.toggle()),
            }
        } else {
            println!("{}", media_session); 
        }
    }

    Ok(())
}
