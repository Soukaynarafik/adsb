use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AircraftState {
    pub icao24: String,
    pub callsign: String,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude_ft: f64,
    pub velocity_kts: f64,
    pub heading_deg: f64,
    pub timestamp: DateTime<Utc>,
    pub is_anomaly: bool,
    pub anomaly_type: String,
}

impl AircraftState {
    pub fn update_position(&mut self, delta_seconds: f64) {
        let rad = self.heading_deg.to_radians();
        let distance_deg_per_sec = self.velocity_kts / 3600.0;
        let distance_moved = distance_deg_per_sec * delta_seconds;

        self.latitude += rad.cos() * distance_moved;
        self.longitude += (rad.sin() * distance_moved) / self.latitude.to_radians().cos();
        
        self.timestamp = Utc::now();
    }
}