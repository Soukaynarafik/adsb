use crate::aircraft::AircraftState;
use axum::{
    routing::get,
    Router,
    response::{Html, IntoResponse, Json},
    extract::State,
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub type SharedState = Arc<Mutex<Vec<AircraftState>>>;

const RADAR_HTML: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <title>ADS-B Ghost Injector - Live Radar Simulation</title>
    <link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" />
    <script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js"></script>
    <style>
        body { margin: 0; padding: 0; background-color: #111; color: #fff; font-family: monospace; }
        #map { width: 100vw; height: 100vh; }
        #ui { position: absolute; top: 10px; left: 50px; z-index: 1000; background: rgba(15,15,15,0.9); padding: 15px; border: 1px solid #00ff00; border-radius: 3px; }
        h2 { margin-top: 0; color: #00ff00; letter-spacing: 1px; }
        .marker-normal { color: #00ff00; font-weight: bold; font-size: 16px; text-shadow: 0 0 3px #000; }
        .marker-anomaly { color: #ff0000; font-weight: bold; font-size: 16px; text-shadow: 0 0 3px #000; }
    </style>
</head>
<body>
    <div id="ui">
        <h2>ADS-B GHOST RADAR</h2>
        <p>Authors: Soukaynarafik & Vanessalauransot</p>
        <p>Tracked Targets: <span id="count">0</span></p>
    </div>
    <div id="map"></div>

    <script>
        var map = L.map('map').setView([46.603354, 1.888334], 6);
        L.tileLayer('https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}{r}.png', {
            attribution: '&copy; OpenStreetMap contributors'
        }).addTo(map);

        var markers = {};
        
        var normalIcon = L.divIcon({ html: '▲', className: 'marker-normal', iconSize: [15, 15] });
        var alertIcon = L.divIcon({ html: '[X]', className: 'marker-anomaly', iconSize: [25, 15] });

        async function fetchTraffic() {
            try {
                let res = await fetch('/api/traffic');
                let data = await res.json();
                document.getElementById('count').innerText = data.length;

                data.forEach(ac => {
                    let text = `<b>Callsign: ${ac.callsign}</b><br>ICAO: ${ac.icao24}<br>Alt: ${Math.round(ac.altitude_ft)} ft<br>Spd: ${Math.round(ac.velocity_kts)} kts<br>Type: ${ac.anomaly_type}`;
                    let icon = ac.is_anomaly ? alertIcon : normalIcon;

                    if (markers[ac.icao24]) {
                        markers[ac.icao24].setLatLng([ac.latitude, ac.longitude]);
                        markers[ac.icao24].setIcon(icon);
                        markers[ac.icao24].setPopupContent(text);
                    } else {
                        markers[ac.icao24] = L.marker([ac.latitude, ac.longitude], {icon: icon})
                            .bindPopup(text)
                            .addTo(map);
                    }
                });
            } catch (err) {
                console.error("Error fetching radar data", err);
            }
        }
        setInterval(fetchTraffic, 1000);
    </script>
</body>
</html>
"#;

async fn serve_radar() -> Html<&'static str> {
    Html(RADAR_HTML)
}

async fn api_traffic(State(state): State<SharedState>) -> impl IntoResponse {
    let traffic = state.lock().await;
    Json(traffic.clone())
}

pub async fn start_server(shared_state: SharedState, port: u16) {
    let app = Router::new()
        .route("/", get(serve_radar))
        .route("/api/traffic", get(api_traffic))
        .with_state(shared_state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("[INFO] Live Radar Web Server initiated. Access terminal interface at http://localhost:{}", port);
    axum::serve(listener, app).await.unwrap();
}
