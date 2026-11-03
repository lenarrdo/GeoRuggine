use chrono::Duration;

use crate::{
    messages::position::models::coordinates::Coordinates, shared::utils::INACTIVE_THRESHOLD,
};

#[derive(Default)]
pub struct Journey {
    _id: u32,
    coordinates: Vec<Coordinates>,
}

impl Journey {
    pub fn new(id: u32, coordinates: Vec<Coordinates>) -> Self {
        Journey {
            _id: id,
            coordinates,
        }
    }

    /// Returns a pair where:
    /// 0. Time spent while moving.
    /// 1. Time spent while NOT moving.
    pub fn get_movement_stats(&self) -> (Duration, Duration) {
        if self.coordinates.is_empty() {
            return (Duration::zero(), Duration::zero());
        }

        let mut time_moving = Duration::zero();
        let mut time_not_moving = Duration::zero();
        let mut first_consecutive_position: &Coordinates = self.coordinates.first().unwrap();

        for window in self.coordinates.windows(2) {
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

    /// Returns a pair where:
    /// 0. Total distance travelled
    /// 1. Number of hours required to travel that distance.
    pub fn get_average_speed(&self) -> Option<(f64, f64)> {
        if self.coordinates.len() < 2 {
            return None;
        }

        let last_pos = self.coordinates.last().unwrap();
        let first_pos = self.coordinates.first().unwrap();
        let duration_seconds = last_pos
            .timestamp
            .signed_duration_since(first_pos.timestamp)
            .num_milliseconds() as f64
            / 1000.0;

        if duration_seconds <= 0.0 {
            return None;
        }

        let total_hours = duration_seconds / 3600.0;

        Some((self.compute_total_distance(), total_hours))
    }

    /// Computes the total distance (in kilometers) travelled in a single journey.
    pub fn compute_total_distance(&self) -> f64 {
        self.coordinates
            .windows(2)
            .map(|w| w[0].distance_from(&w[1]))
            .sum()
    }

    /// records a new position
    pub fn push(&mut self, position: Coordinates) {
        self.coordinates.push(position);
    }
}
