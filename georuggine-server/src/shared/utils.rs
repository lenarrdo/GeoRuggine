use chrono::Duration as ChronoDuration;
use std::{fs, path::Path, time::Duration as StdDuration};

pub const INACTIVE_THRESHOLD: ChronoDuration = ChronoDuration::minutes(3);
pub const POSITION_UPDATE_INTERVAL: ChronoDuration = ChronoDuration::seconds(30);
pub const SERVER_ID: u32 = 0;

pub fn format_chrono_duration(duration: ChronoDuration) -> String {
    let std_duration = duration.to_std().expect("Found a negative duration.");

    let cleaned_dur = StdDuration::from_secs(std_duration.as_secs());

    humantime::format_duration(cleaned_dur).to_string()
}

pub fn ensure_dir_exists(url: &str) -> std::io::Result<()> {
    let path = Path::new(url);
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent)?;
    }

    Ok(())
}
