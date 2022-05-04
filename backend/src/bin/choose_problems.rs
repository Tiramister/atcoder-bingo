use anyhow::Result;
use atcoder_bingo_backend::{
    crawler::problems::{get_problems, Problem},
    database::{models, DatabaseClient},
};
use chrono::{Duration, Local};
use rand::prelude::SliceRandom;
use tokio::time::sleep;

pub const BINGO_SIZE: usize = 9;
pub const BINGO_NUM: usize = 5;
const DIFF_DISTR: [(i32, i32); BINGO_NUM] = [
    (-10000, 600),
    (400, 1400),
    (1200, 2200),
    (2000, 2800),
    (2600, 10000),
];

async fn choose_problems() -> Result<Vec<Problem>> {
    let mut rng = rand::thread_rng();

    // Fetch problems and sort by difficulties.
    let mut problems = get_problems().await?;
    problems.sort_by_key(|problem| problem.difficulty);

    let mut bingo_problems = Vec::new();
    for (lower_diff, upper_diff) in DIFF_DISTR {
        // `lower_index` = the leftmost problem with difficulty >= `lower_diff`.
        // `upper_index` = the rightmost problem with difficulty >= `upper_diff`.
        // Problems whose difficulty is in [lower_diff, upper_diff) is in [lower_index, upper_index).
        // Double the difficulties so that any problems doesn't match and we can detect the precise border.
        let lower_index = problems
            .binary_search_by_key(&(lower_diff * 2 + 1), |problem| problem.difficulty * 2)
            .unwrap_or_else(|i| i);
        let upper_index = problems
            .binary_search_by_key(&(upper_diff * 2 + 1), |problem| problem.difficulty * 2)
            .unwrap_or_else(|i| i);

        // Choose problems randomly.
        let mut indices: Vec<usize> = (lower_index..upper_index).collect();
        let (chosen_indices, _) = indices.partial_shuffle(&mut rng, BINGO_SIZE);
        let mut bingo = chosen_indices
            .iter_mut()
            .map(|index| problems[*index].clone())
            .collect();

        bingo_problems.append(&mut bingo);
    }

    Ok(bingo_problems)
}

async fn store_problems(problems: &[Problem], client: &DatabaseClient) -> Result<()> {
    // See 10 mins later.
    let today = Local::now()
        .checked_add_signed(Duration::minutes(10))
        .unwrap()
        .date()
        .naive_local();

    for (position, problem) in problems.iter().enumerate() {
        let problem_entity = models::Problem {
            id: 0,
            chosen_date: today,
            position: position as i32,
            problem_id: problem.problem_id.clone(),
            contest_id: problem.contest_id.clone(),
            title: problem.title.clone(),
            difficulty: problem.difficulty,
        };
        client.insert_problem(&problem_entity).await?;
    }
    Ok(())
}

/// Choose problems if they have not been chosen today.
/// Return whether or not problems are chosen.
async fn choose_and_store_problems(client: &DatabaseClient) -> Result<bool> {
    // See 10 mins later.
    let today = Local::now()
        .checked_add_signed(Duration::minutes(10))
        .unwrap()
        .date()
        .naive_local();

    // Check if today's bingo already exists.
    let newest_chosen_date_opt = client.select_newest_chosen_date_of_problems().await?;
    let bingo_exists = match newest_chosen_date_opt {
        Some(newest_chosen_date) => newest_chosen_date == today,
        None => false,
    };
    if bingo_exists {
        return Ok(false);
    }

    // Generate and store bingo.
    let problems = choose_problems().await?;
    store_problems(&problems, client).await?;
    Ok(true)
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let client = DatabaseClient::new().await;

    loop {
        // Check if the daily bingo exists in every 5 mins
        match choose_and_store_problems(&client).await {
            Ok(true) => log::info!("New bingo is generated."),
            Ok(false) => log::info!("Today's bingo already exists."),
            Err(e) => log::error!("Failed to generate bingo: {}", e),
        }
        sleep(std::time::Duration::from_secs(300)).await;
    }
}
