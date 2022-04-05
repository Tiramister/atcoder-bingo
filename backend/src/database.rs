use std::{env, time::Duration};
use tokio::time::sleep;
use tokio_postgres::{Client, NoTls};

/// Get postgres client.
/// Try to connect until success.
pub async fn get_postgres_client() -> Client {
    let url = env::var("POSTGRES_URL").expect("error: POSTGRES_URL is not set.");

    let mut conn = tokio_postgres::connect(&url, NoTls).await;
    while let Err(e) = conn {
        eprintln!("Failed to connect to the database: {e}");
        sleep(Duration::from_secs(5)).await;
        eprintln!("Try to connect again...");
        conn = tokio_postgres::connect(&url, NoTls).await;
    }

    let (client, connection) = conn.unwrap();
    eprintln!("Succeed to connect to the database.");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    client
}
