use std::fmt;
use windows::Media::Control::GlobalSystemMediaTransportControlsSessionManager;

struct MediaInfo {
    pub title: String,
    pub time: i64,
}

impl fmt::Display for MediaInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.title, self.time)
    }
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let playing = match get_media_info().await {
        Ok(song) => song,
        Err(_) => MediaInfo {
            title: "No Song Playing".to_owned(),
            time: 0,
        }, // No media playing
    };
    println!("{}", playing);
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
    let time = match timeline.Position() {
        Ok(stuff) => stuff,
        Err(err) => return Err(err),
    };
    println!("{:?}", time);
    // Return song title
    Ok(MediaInfo {
        title: title.to_string(),
        time: time.Duration,
    })
}
