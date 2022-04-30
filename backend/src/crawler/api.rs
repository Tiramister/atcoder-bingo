use anyhow::Result;
use std::time::Duration;
use tokio::time::sleep;

/// Send GET request and return its response.
/// Wait 5 secs after querying.
pub async fn get_request(url: &str) -> Result<String> {
    let body = reqwest::get(url).await?.text().await?;
    sleep(Duration::from_secs(5)).await;
    Ok(body)
}
