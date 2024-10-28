mod ingestion;
mod server;

use dotenv::dotenv;
use std::env;
use std::sync::Arc;
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    let client = Arc::new(client);

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    let ingestion_client = Arc::clone(&client);
    tokio::spawn(async move {
        if let Err(e) = ingestion::start_ingestion_loop(ingestion_client).await {
            eprintln!("Ingestion loop error: {}", e);
        }
    });

    server::start_server(client).await;

    Ok(())
}
