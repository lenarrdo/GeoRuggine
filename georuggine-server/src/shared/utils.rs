use std::time::Duration as StdDuration;

use chrono::Duration as ChronoDuration;

pub fn format_chrono_duration(duration: ChronoDuration) -> String {
    let std_duration = duration.to_std().expect("Found a negative duration.");

    let cleaned_dur = StdDuration::from_secs(std_duration.as_secs());

    humantime::format_duration(cleaned_dur).to_string()
}
