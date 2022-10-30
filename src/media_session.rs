/*
 * Copyright (C) 2022 DakshG07.
 *
 * This file is part of now-playing.
 *
 * now-playing is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * now-playing is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 */

use crate::cleanup_timespan::Cleanup;
use crate::MediaStatus;
use human_repr::HumanDurationData;
use std::fmt;
use windows::Media::Control::{
    GlobalSystemMediaTransportControlsSession, GlobalSystemMediaTransportControlsSessionManager,
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
            session,
            properties,
            timeline,
        })
    }

    pub fn get_artist(&self) -> String {
        self.properties.Artist().unwrap_or_default().to_string()
    }

    pub fn get_title(&self) -> String {
        self.properties.Title().unwrap_or_default().to_string()
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
                return MediaStatus::from(s);
            }
        }
        MediaStatus::Closed
    }

    pub fn play(&self) -> bool {
        if let Ok(res) = self.session.TryPlayAsync() {
            res.get().unwrap_or(false)
        } else {
            false
        }
    }

    pub fn pause(&self) -> bool {
        if let Ok(res) = self.session.TryPauseAsync() {
            res.get().unwrap_or(false)
        } else {
            false
        }
    }

    pub fn toggle(&self) -> bool {
        if let Ok(res) = self.session.TryTogglePlayPauseAsync() {
            res.get().unwrap_or(false)
        } else {
            false
        }
    }

    // pub fn stop() -> bool {
    //     todo!()
    // }

    // pub fn skip() -> bool {
    //     todo!()
    // }

    // pub fn previous() -> bool {
    //     todo!()
    // }

    // pub fn set_position(new_pos: u64) -> bool {
    //     todo!()
    // }
}

impl fmt::Display for MediaSession {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {} ({})",
            self.get_title(),
            self.get_artist(),
            self.get_position()
        )
    }
}
