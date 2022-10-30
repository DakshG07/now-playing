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
