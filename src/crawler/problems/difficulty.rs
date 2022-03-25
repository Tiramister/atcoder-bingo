use crate::crawler::api::hit_api;
use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
struct RawProblemDifficulty {
    difficulty: Option<i32>,
    is_experimental: Option<bool>,
}

#[derive(Debug)]
pub struct ProblemDifficulty {
    pub problem_id: String,
    pub difficulty: i32,
}

/// Fetch difficulties from AtCoder Problems API.
pub async fn fetch_difficulties() -> Result<Vec<ProblemDifficulty>> {
    let body = hit_api("https://kenkoooo.com/atcoder/resources/problem-models.json").await?;

    let map: HashMap<String, RawProblemDifficulty> = serde_json::from_str(&body)?;
    let difficulties = map
        .into_iter()
        .filter_map(|(problem_id, problem)| {
            // Filter elements with non-experimental difficulty
            match (problem.difficulty, problem.is_experimental) {
                (Some(difficulty), Some(false)) => Some(ProblemDifficulty {
                    problem_id,
                    difficulty,
                }),
                _ => None,
            }
        })
        .collect();
    Ok(difficulties)
}
