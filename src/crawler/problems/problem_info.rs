use anyhow::Result;
use serde::Deserialize;

use crate::crawler::api::hit_api;

#[derive(Deserialize, Debug)]
pub struct ProblemInfo {
    pub id: String,
    pub contest_id: String,
    pub title: String,
}

/// Fetch problems from AtCoder Problems API.
pub async fn fetch_problem_info() -> Result<Vec<ProblemInfo>> {
    let body = hit_api("https://kenkoooo.com/atcoder/resources/problems.json").await?;
    let problems: Vec<ProblemInfo> = serde_json::from_str(&body)?;
    Ok(problems)
}
