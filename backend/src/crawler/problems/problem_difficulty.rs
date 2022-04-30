use crate::crawler::api::get_request;
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
pub async fn get_problem_difficulties() -> Result<Vec<ProblemDifficulty>> {
    let body = get_request("https://kenkoooo.com/atcoder/resources/problem-models.json").await?;
    let map: HashMap<String, RawProblemDifficulty> = serde_json::from_str(&body)?;

    // Filter problems with a non-experimental difficulty.
    let problem_difficulties = map
        .into_iter()
        .filter_map(|(problem_id, raw_problem_difficulty)| {
            // Filter elements with non-experimental difficulty
            match (
                raw_problem_difficulty.difficulty,
                raw_problem_difficulty.is_experimental,
            ) {
                (Some(difficulty), Some(false)) => Some(ProblemDifficulty {
                    problem_id,
                    difficulty,
                }),
                _ => None,
            }
        })
        .collect();

    Ok(problem_difficulties)
}
