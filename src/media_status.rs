use windows::Media::Control::GlobalSystemMediaTransportControlsSessionPlaybackStatus as WinPlaybackStatus;
use std::fmt;

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

impl MediaStatus {
    pub fn from_win(a: WinPlaybackStatus) -> Self {
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
