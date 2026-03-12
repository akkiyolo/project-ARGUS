mod fetcher;

#[tokio::main]
async fn main() {

    println!("ARGUS Ingestion Service Starting...");

    match fetcher::fetch_aircraft_data().await {

        Ok(data) => {
            println!("Aircraft Data Received:");
            println!("{:#?}", data);
        }

        Err(e) => {
            println!("Error fetching data: {:?}", e);
        }
    }
}