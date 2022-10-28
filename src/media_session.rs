use std::fmt;
use crate::cleanup_timespan::Cleanup;
use crate::MediaStatus;
use human_repr::HumanDurationData;
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession,
    GlobalSystemMediaTransportControlsSessionManager,
    GlobalSystemMediaTransportControlsSessionMediaProperties,
    GlobalSystemMediaTransportControlsSessionTimelineProperties,
};

pub struct MediaSession {
    session: GlobalSystemMediaTransportControlsSession,
    properties: GlobalSystemMediaTransportControlsSessionMediaProperties,
    timeline: GlobalSystemMediaTransportControlsSessionTimelineProperties,
}

impl MediaSession {
    pub async fn new() -> Result<Self, windows::core::Error> {
        let mp = GlobalSystemMediaTransportControlsSessionManager::RequestAsync()?.await?;
        let session = mp.GetCurrentSession()?;
        let properties = session.TryGetMediaPropertiesAsync()?.await?;
        let timeline = session.GetTimelineProperties()?;
        Ok(Self {
            session: session,
            properties: properties, //.session.TryGetMediaPropertiesAsync()?.await?,
            timeline: timeline, //res.session.GetTimelineProperties()?,
        })
    }
    pub fn get_artist(&self) -> String {
        self.properties.Title().unwrap_or_default().to_string()
    }
    pub fn get_title(&self) -> String {
        self.properties.Artist().unwrap_or_default().to_string()
    }
    pub fn get_position(&self) -> HumanDurationData {
        self.timeline.Position().unwrap_or_default().cleanup()
    }
    pub fn get_duration(&self) -> HumanDurationData {
        self.timeline.EndTime().unwrap_or_default().cleanup()
    }
    pub fn get_status(&self) -> MediaStatus {
        if let Ok(p) = self.session.GetPlaybackInfo() {
            if let Ok(s) = p.PlaybackStatus() {
                return MediaStatus::from_win(s);
            }
        }
        MediaStatus::Closed
    }

    pub fn play(&self) -> bool {
        if let Ok(res) = self.session.TryPlayAsync() {
            match res.get() {
                Ok(a) => a,
                Err(_) => false,
            }
        } else {
            false
        }
    }
    pub fn pause(&self) -> bool {
        if let Ok(res) = self.session.TryPauseAsync() {
            match res.get() {
                Ok(a) => a,
                Err(_) => false,
            }
        } else {
            false
        }
    }
    pub fn toggle(&self) -> bool {
        if let Ok(res) = self.session.TryTogglePlayPauseAsync() {
            match res.get() {
                Ok(a) => a,
                Err(_) => false,
            }
        } else {
            false
        }
    }

    // to implement:
    // pub fn stop() -> Result<()> {}
    // pub fn skip() -> Result<()> {}
    // pub fn previous() -> Result<()> {}

    // maybe: pub fn set_position(a: u64) -> Result<()> {}
    // also shuffle
    
    // use TimelinePropertiesChanged function callback
    // use PlaybackInfoChanged function callback
}

impl fmt::Display for MediaSession {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} - {} ({})", self.get_title(), self.get_artist(), self.get_position())
    }
}
