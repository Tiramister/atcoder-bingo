use anyhow::Result;
use std::time::Duration;
use tokio::time::sleep;

/// Hit AtCoder Problems API.
pub async fn hit_api(url: &str) -> Result<String> {
    let body = reqwest::get(url).await?.text().await?;
    sleep(Duration::from_secs(5)).await;
    Ok(body)
}
