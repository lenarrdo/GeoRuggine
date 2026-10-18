use std::fmt::{Debug, Display};

use chrono::{DateTime, Local};

#[derive(Clone)]
pub struct Coordinates {
    lat: f64,
    lon: f64,
    received_at: DateTime<Local>,
}

impl PartialEq for Coordinates {
    fn eq(&self, other: &Self) -> bool {
        self.lat == other.lat && self.lon == other.lon
    }
}

impl Coordinates {
    pub fn new(lat: f64, lon: f64) -> Self {
        Coordinates {
            lat,
            lon,
            received_at: Local::now(),
        }
    }

    /// Haversine formula implementation to compute the distance between `self` and `other`.
    pub fn distance_from(&self, other: &Coordinates) -> f64 {
        const EARTH_RADIUS_KM: f64 = 6371.0;

        let d_lat = (other.lat - self.lat).to_radians();
        let d_lon = (other.lon - self.lon).to_radians();

        let lat1_rad = self.lat.to_radians();
        let lat2_rad = other.lat.to_radians();

        let a = (d_lat / 2.0).sin().powi(2)
            + lat1_rad.cos() * lat2_rad.cos() * (d_lon / 2.0).sin().powi(2);

        let c = 2.0 * a.sqrt().asin();

        EARTH_RADIUS_KM * c
    }

    pub fn received_at(&self) -> DateTime<Local> {
        self.received_at
    }

    pub fn lat(&self) -> f64 {
        self.lat
    }

    pub fn lon(&self) -> f64 {
        self.lon
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.lat, self.lon)
    }
}

impl Debug for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("coord")
            .field("lat", &self.lat)
            .field("lon", &self.lon)
            .finish()
    }
}
