use crate::crawler::api::get_request;
use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ProblemInfo {
    pub id: String,
    pub contest_id: String,
    pub title: String,
}

/// Fetch problems from AtCoder Problems API.
pub async fn get_problem_info() -> Result<Vec<ProblemInfo>> {
    let body = get_request("https://kenkoooo.com/atcoder/resources/problems.json").await?;
    let problems: Vec<ProblemInfo> = serde_json::from_str(&body)?;
    Ok(problems)
}
