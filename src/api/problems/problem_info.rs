use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ProblemInfo {
    pub id: String,
    pub contest_id: String,
    pub title: String,
}

pub async fn get_problem_info() -> Result<Vec<ProblemInfo>> {
    let body = reqwest::get("https://kenkoooo.com/atcoder/resources/problems.json")
        .await?
        .text()
        .await?;

    let problems: Vec<ProblemInfo> = serde_json::from_str(&body)?;
    Ok(problems)
}
