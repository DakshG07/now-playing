use std::fmt;
use human_repr::HumanDurationData;

pub struct MediaInfo {
    pub title: String,
    pub artist: String,
    pub position: HumanDurationData,
}

impl fmt::Display for MediaInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.position != "0ns" && !self.artist.is_empty() {
            // normal format
            write!(f, "{} - {} ({})", self.title, self.artist, self.position)
        } else if !self.artist.is_empty() && self.position == "0ns" {
            // position empty
            write!(f, "{} - {}", self.title, self.artist)
        } else if self.position != "0ns" && self.artist.is_empty() {
            // artist empty
            write!(f, "{} ({})", self.title, self.position)
        } else {
            write!(f, "{}", self.title)
        }
    }
}
