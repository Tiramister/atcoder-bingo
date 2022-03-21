use std::env;
use tokio_postgres::{Client, NoTls};

pub async fn get_client() -> Client {
    let url = env::var("POSTGRES_URL").expect("error: POSTGRES_URL is not set.");
    let (client, connection) = tokio_postgres::connect(&url, NoTls)
        .await
        .expect("error: failed to connect to PostgreSQL.");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    
    client
}
