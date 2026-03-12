use reqwest;
use serde_json::Value;

pub async fn fetch_aircraft_data() -> Result<Value, reqwest::Error> {

    let url = "https://opensky-network.org/api/states/all";

    let response = reqwest::get(url).await?;

    let data: Value = response.json().await?;

    Ok(data)
}