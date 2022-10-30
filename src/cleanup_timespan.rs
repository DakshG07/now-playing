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

use human_repr::HumanDuration;
use human_repr::HumanDurationData;
use windows::Foundation::TimeSpan;

pub trait Cleanup {
    fn cleanup(&self) -> HumanDurationData;
}

impl Cleanup for TimeSpan {
    fn cleanup(&self) -> HumanDurationData {
        (self.Duration / 10_i64.pow(7)).human_duration()
    }
}
