pub mod message_repository;
pub mod message_service;

use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Tipi di messaggi che il client può inviare al server
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ClientIncomingMessage {
    Position(Coordinates),
    Text(TextMessage),
}

/// Modello per i messaggi di testo (client e server inviano e ricevono entrambi contents e timestamp generato in quel momento)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextMessage {
    pub contents: String,
    pub timestamp: String,
}

impl TextMessage {
    pub fn new(contents: String) -> Self {
        TextMessage {
            contents,
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}

/// Coordinate geografiche inviate dal client al server (il timestamp viene calcolato dal client)
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Coordinates {
    lat: f64,
    lon: f64,
    timestamp: DateTime<Utc>,
}

impl Coordinates {
    pub fn new(lat: f64, lon: f64, timestamp: DateTime<Utc>) -> Self {
        Coordinates {
            lat,
            lon,
            timestamp,
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

    pub fn received_at(&self) -> DateTime<Utc> {
        self.timestamp
    }

    pub fn lat(&self) -> f64 {
        self.lat
    }

    pub fn lon(&self) -> f64 {
        self.lon
    }
}

impl PartialEq for &Coordinates {
    fn eq(&self, other: &Self) -> bool {
        self.lat == other.lat && self.lon == other.lon
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.lat, self.lon)
    }
}

