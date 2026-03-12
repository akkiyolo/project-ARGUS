use reqwest;
use serde_json::Value;

use crate::models::AircraftState;

pub async fn fetch_aircraft_data() -> Result<Vec<AircraftState>, reqwest::Error> {

    let url = "https://opensky-network.org/api/states/all";

    let response = reqwest::get(url).await?;

    let data: Value = response.json().await?;

    let mut aircraft_list = Vec::new();

    if let Some(states) = data["states"].as_array() {

        for aircraft in states {

            if let Some(arr) = aircraft.as_array() {

                let aircraft_state = AircraftState {

                    icao24: arr.get(0).and_then(|v| v.as_str()).map(String::from),

                    callsign: arr.get(1).and_then(|v| v.as_str()).map(|s| s.trim().to_string()),

                    origin_country: arr.get(2).and_then(|v| v.as_str()).map(String::from),

                    longitude: arr.get(5).and_then(|v| v.as_f64()),

                    latitude: arr.get(6).and_then(|v| v.as_f64()),

                    altitude: arr.get(7).and_then(|v| v.as_f64()),

                    velocity: arr.get(9).and_then(|v| v.as_f64()),
                };

                aircraft_list.push(aircraft_state);
            }
        }
    }

    Ok(aircraft_list)
}