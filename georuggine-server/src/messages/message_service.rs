use chrono::Duration;

use crate::messages::Coordinates;

const INACTIVE_THRESHOLD: Duration = Duration::minutes(3);

/// Restituisce una coppia in cui:
/// 0. Quantità di tempo passata in movimento.
/// 1. Quantità di tempo passata da fermo (pause).
pub fn get_movement_stats(positions: &Vec<Coordinates>) -> (Duration, Duration) {
    if positions.is_empty() {
        return (Duration::zero(), Duration::zero());
    }

    let mut time_moving = Duration::zero();
    let mut time_not_moving = Duration::zero();
    let mut first_consecutive_position: &Coordinates = positions.first().unwrap();

    for window in positions.windows(2) {
        let prev_pos = &window[0];
        let curr_pos = &window[1];

        let delta_t = curr_pos.received_at - prev_pos.received_at;
        let threshold_passed =
            curr_pos.received_at - first_consecutive_position.received_at > INACTIVE_THRESHOLD;

        if prev_pos != curr_pos {
            time_moving += delta_t;
            first_consecutive_position = prev_pos;
        } else if prev_pos == curr_pos && threshold_passed {
            time_not_moving += delta_t;
        } else {
            time_moving += delta_t;
        }
    }

    (time_moving, time_not_moving)
}
