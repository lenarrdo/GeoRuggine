use chrono::Duration;

use crate::{messages::Coordinates, shared::utils::INACTIVE_THRESHOLD};

pub fn get_movement_stats(journeys: &Vec<Vec<Coordinates>>) -> (Duration, Duration) {
    let mut acc = (Duration::zero(), Duration::zero());
    for journey in journeys {
        let stats = get_movement_stats_aux(journey);
        acc.0 += stats.0;
        acc.1 += stats.1;
    }

    acc
}

/// Dato un Vec<Coordinates> rappresentante le posizioni di una singola journe
/// 0. Quantità di tempo passata in movimento.
/// 1. Quantità di tempo passata da fermo (pause).
fn get_movement_stats_aux(journey: &Vec<Coordinates>) -> (Duration, Duration) {
    if journey.is_empty() {
        return (Duration::zero(), Duration::zero());
    }

    let mut time_moving = Duration::zero();
    let mut time_not_moving = Duration::zero();
    let mut first_consecutive_position: &Coordinates = journey.first().unwrap();

    for window in journey.windows(2) {
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
pub fn get_average_speed(journeys: &Vec<Vec<Coordinates>>) -> Option<f64> {
    let mut total_distance = 0.0;
    let mut total_hours = 0.0;
    for journey in journeys {
        let Some((j_distance, j_hours)) = get_average_speed_aux(journey) else {
            continue;
        };

        total_distance += j_distance;
        total_hours += j_hours;
    }

    if total_hours > 0.0 {
        Some(total_distance / total_hours)
    } else {
        None
    }
}

/// Data una specifica journey, restituisce una coppia in cui:
/// 0. Totale distanza percorsa
/// 1. Totale ore impiegate
fn get_average_speed_aux(journey: &Vec<Coordinates>) -> Option<(f64, f64)> {
    if journey.len() < 2 {
        return None;
    }

    let last_pos = journey.last().unwrap();
    let first_pos = journey.first().unwrap();

    let total_distance = last_pos.distance_from(first_pos);

    let duration_seconds = last_pos
        .received_at()
        .signed_duration_since(first_pos.received_at())
        .num_milliseconds() as f64
        / 1000.0;

    if duration_seconds <= 0.0 {
        return None;
    }

    let total_hours = duration_seconds / 3600.0;

    Some((total_distance, total_hours))
}
