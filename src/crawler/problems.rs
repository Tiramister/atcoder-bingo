mod difficulty;
mod problem_info;

use anyhow::Result;
use difficulty::fetch_difficulties;
use problem_info::{fetch_problem_info, ProblemInfo};
use std::collections::HashMap;
use tokio::try_join;

/// Problem information with its estimated difficulty.
#[derive(Clone, Debug)]
pub struct Problem {
    pub problem_id: String,
    pub contest_id: String,
    pub title: String,
    pub difficulty: i32,
}
impl Problem {
    pub fn url(&self) -> String {
        format!(
            "https://atcoder.jp/contests/{}/tasks/{}",
            self.contest_id, self.problem_id,
        )
    }
}

/// Fetch problems with their difficulties.
pub async fn fetch_problems() -> Result<Vec<Problem>> {
    // Fetch necessary information from AtCoder Problems API.
    let (problem_difficulties, problem_info) =
        try_join!(fetch_difficulties(), fetch_problem_info())?;

    // Convert `problem_info` into HashMap so that we can retrieve them by `problem_id` efficiently.
    let problem_info_map: HashMap<String, ProblemInfo> = problem_info
        .into_iter()
        .map(|problem| (problem.id.clone(), problem))
        .collect();

    // Join `problem_difficulties` and `problem_info` by their `problem_id`.
    let merged_problems = problem_difficulties
        .into_iter()
        .filter_map(|difficulty| {
            // Search corresponding `problem_info`.
            let problem_info_opt = problem_info_map.get(&difficulty.problem_id);

            // Convert into `Problem`.
            problem_info_opt.map(|problem_info| Problem {
                problem_id: difficulty.problem_id,
                contest_id: problem_info.contest_id.clone(),
                title: problem_info.title.clone(),
                difficulty: difficulty.difficulty,
            })
        })
        .collect();

    Ok(merged_problems)
}
