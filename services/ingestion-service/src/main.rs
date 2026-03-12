mod fetcher;
mod models;

use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {

    println!("ARGUS Ingestion Service Running...\n");

    loop {

        match fetcher::fetch_aircraft_data().await {

            Ok(aircraft) => {

                println!("Aircraft detected: {}\n", aircraft.len());

                for a in aircraft.iter().take(5) {

                    println!(
                        "Flight: {:?} | Position: ({:?}, {:?}) | Altitude: {:?}",
                        a.callsign,
                        a.latitude,
                        a.longitude,
                        a.altitude
                    );
                }

                println!("---------------------------------\n");
            }

            Err(e) => {
                println!("Error fetching aircraft data: {:?}", e);
            }
        }

        sleep(Duration::from_secs(10)).await;
    }
}