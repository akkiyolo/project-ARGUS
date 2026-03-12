use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AircraftState {
    pub icao24: String,
    pub callsign: Option<String>,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub altitude: Option<f64>,
    pub velocity: Option<f64>,
}