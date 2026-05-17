use crate::aircraft::AircraftState;
use rand::Rng;
use chrono::Utc;

pub fn apply_position_spoofing(aircrafts: &mut [AircraftState]) {
    let mut rng = rand::thread_rng();
    for ac in aircrafts.iter_mut() {
        ac.latitude += rng.gen_range(-5.0..5.0);
        ac.longitude += rng.gen_range(-5.0..5.0);
        ac.is_anomaly = true;
        ac.anomaly_type = "SPOOF_POSITION".to_string();
    }
}

pub fn apply_impossible_speeds(aircrafts: &mut [AircraftState]) {
    let mut rng = rand::thread_rng();
    for ac in aircrafts.iter_mut() {
        ac.velocity_kts = rng.gen_range(2500.0..3500.0);
        ac.is_anomaly = true;
        ac.anomaly_type = "IMPOSSIBLE_SPEED".to_string();
    }
}

pub fn apply_inconsistent_altitudes(aircrafts: &mut [AircraftState]) {
    let mut rng = rand::thread_rng();
    for ac in aircrafts.iter_mut() {
        ac.altitude_ft = if rng.gen_bool(0.5) { -1200.0 } else { 135000.0 };
        ac.is_anomaly = true;
        ac.anomaly_type = "INCONSISTENT_ALTITUDE".to_string();
    }
}

pub fn generate_flood(count: usize) -> Vec<AircraftState> {
    let mut rng = rand::thread_rng();
    let mut flood_traffic = Vec::new();

    for i in 0..count {
        let icao24 = format!("{:06X}", rng.gen::<u32>() & 0xFFFFFF);
        let callsign = format!("GHOST{:04}", i);

        flood_traffic.push(AircraftState {
            icao24,
            callsign,
            latitude: rng.gen_range(48.0..49.0),
            longitude: rng.gen_range(2.0..3.0),
            altitude_ft: rng.gen_range(1000.0..15000.0),
            velocity_kts: rng.gen_range(150.0..600.0),
            heading_deg: rng.gen_range(0.0..360.0),
            timestamp: Utc::now(),
            is_anomaly: true,
            anomaly_type: "FLOOD_ATTACK".to_string(),
        });
    }
    flood_traffic
}