use chrono::Duration;

use crate::{
    messages::position::models::coordinates::Coordinates, shared::utils::INACTIVE_THRESHOLD,
};

#[derive(Clone, Copy, PartialEq)]
enum State {
    Moving,
    Stopped,
}

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

        let mut state = State::Moving;
        let mut stationary_start = self.coordinates[0].timestamp;

        for window in self.coordinates.windows(2) {
            let (prev, curr) = (&window[0], &window[1]);
            let delta = curr.timestamp - prev.timestamp;
            let pos_changed = prev != curr;

            match (state, pos_changed) {
                (State::Stopped, true) => {
                    time_not_moving += delta;
                    state = State::Moving;
                    stationary_start = curr.timestamp;
                }
                (State::Moving, true) => {
                    time_moving += delta;
                    stationary_start = curr.timestamp;
                }
                (State::Stopped, false) => {
                    time_not_moving += delta;
                }
                (State::Moving, false) => {
                    let curr_elapsed = curr.timestamp - stationary_start;

                    if curr_elapsed <= INACTIVE_THRESHOLD {
                        time_moving += delta;
                    } else {
                        let prev_elapsed = prev.timestamp - stationary_start;
                        let moving_part = (INACTIVE_THRESHOLD - prev_elapsed).max(Duration::zero());

                        time_moving += moving_part;
                        time_not_moving += delta - moving_part;
                        state = State::Stopped;
                    }
                }
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
