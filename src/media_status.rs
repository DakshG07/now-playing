use std::fmt;
use windows::Media::Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus as WinPlaybackStatus;

#[derive(Debug)]
pub enum MediaStatus {
    Closed,
    Opened,
    Changing,
    Stopped,
    Playing,
    Paused,
}

impl fmt::Display for MediaStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<WinPlaybackStatus> for MediaStatus {
    fn from(a: WinPlaybackStatus) -> Self {
        match a {
            WinPlaybackStatus::Closed => MediaStatus::Closed,
            WinPlaybackStatus::Opened => MediaStatus::Opened,
            WinPlaybackStatus::Changing => MediaStatus::Changing,
            WinPlaybackStatus::Stopped => MediaStatus::Stopped,
            WinPlaybackStatus::Playing => MediaStatus::Playing,
            WinPlaybackStatus::Paused => MediaStatus::Paused,
            // there do not exist any more cases
            _ => panic!(),
        }
    }
}
