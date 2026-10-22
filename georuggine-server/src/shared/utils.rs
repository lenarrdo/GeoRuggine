use chrono::Duration as ChronoDuration;
use std::time::Duration as StdDuration;
use tokio::sync::mpsc::Sender;

use crate::tui::AppEvent;

pub const INACTIVE_THRESHOLD: ChronoDuration = ChronoDuration::minutes(3);

pub fn format_chrono_duration(duration: ChronoDuration) -> String {
    let std_duration = duration.to_std().expect("Found a negative duration.");

    let cleaned_dur = StdDuration::from_secs(std_duration.as_secs());

    humantime::format_duration(cleaned_dur).to_string()
}

pub async fn send_app_events(events: Vec<AppEvent>, tx: &Sender<AppEvent>) {
    for event in events {
        tx.send(event).await.unwrap();
    }
}
