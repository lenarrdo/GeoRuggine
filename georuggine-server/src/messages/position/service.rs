use chrono::Duration;

use crate::messages::position::models::journey::Journey;

/// Given a set of journeys, computes:
/// 0. The total time spent while moving.
/// 1. The total time spent while NOT moving.
pub fn get_movement_stats(journeys: &[Journey]) -> (Duration, Duration) {
    let mut total_time_moving = Duration::zero();
    let mut total_time_not_moving = Duration::zero();
    for journey in journeys {
        let (t_moving, t_not_moving) = journey.get_movement_stats();
        total_time_moving += t_moving;
        total_time_not_moving += t_not_moving;
    }

    (total_time_moving, total_time_not_moving)
}

/// Computes the average speed maintained over a set of journeys (if enough data is available).
pub fn get_average_speed(journeys: &[Journey]) -> Option<f64> {
    let mut total_distance = 0.0;
    let mut total_hours = 0.0;
    for journey in journeys {
        let Some((j_distance, j_hours)) = journey.get_average_speed() else {
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
