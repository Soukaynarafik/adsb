use crate::aircraft::AircraftState;
use rand::Rng;
use chrono::Utc;

pub fn generate_normal_aircraft(id: usize) -> AircraftState {
    let mut rng = rand::thread_rng();
    
    let icao24 = format!("{:06X}", rng.gen::<u32>() & 0xFFFFFF);
    let callsign = format!("DLH{:03}", id + 100);

    AircraftState {
        icao24,
        callsign,
        latitude: rng.gen_range(43.5..49.5),
        longitude: rng.gen_range(-1.5..6.0),
        altitude_ft: rng.gen_range(28000.0..41000.0),
        velocity_kts: rng.gen_range(400.0..480.0),
        heading_deg: rng.gen_range(0.0..360.0),
        timestamp: Utc::now(),
        is_anomaly: false,
        anomaly_type: "NONE".to_string(),
    }
}

pub fn generate_traffic(count: usize) -> Vec<AircraftState> {
    (0..count).map(|id| generate_normal_aircraft(id)).collect()
}