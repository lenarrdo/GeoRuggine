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

        let delta_t = curr_pos.timestamp - prev_pos.timestamp;
        let threshold_passed =
            curr_pos.timestamp - first_consecutive_position.timestamp > INACTIVE_THRESHOLD;

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

/// Restituisce la velocità media di un utente lungo un percorso
/// oppure `None` se non ci sono dati a sufficienza per calcolare la velocità.
pub fn get_average_speed(positions: &Vec<Coordinates>) -> Option<f64> {
    if positions.len() < 2 {
        return None;
    }

    let last_pos = positions.last().unwrap();
    let first_pos = positions.first().unwrap();

    let total_distance = last_pos.distance_from(first_pos);

    let duration_seconds = last_pos
        .received_at()
        .signed_duration_since(first_pos.received_at())
        .num_milliseconds() as f64
        / 1000.0;

    if duration_seconds <= 0.0 {
        return Some(0.0);
    }

    let total_hours = duration_seconds / 3600.0;

    Some(total_distance / total_hours)
}
