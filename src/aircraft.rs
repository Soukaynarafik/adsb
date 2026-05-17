use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub struct Aircraft {
    pub timestamp: String,
    pub icao: String,
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: i32,
    pub speed: i32,
    pub heading: i32,
}