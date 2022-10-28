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
