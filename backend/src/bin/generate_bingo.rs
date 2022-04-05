use anyhow::Result;
use atcoder_bingo_backend::{
    crawler::problems::{fetch_problems, Problem},
    database::get_postgres_client,
};
use chrono::{Local, NaiveDate};
use rand::prelude::SliceRandom;
use std::time::Duration;
use tokio::time::sleep;
use tokio_postgres::Client;

pub const BINGO_SIZE: usize = 3;
const DIFF_DISTR: [(i32, i32); 5] = [
    (-10000, 600),
    (400, 1400),
    (1200, 2200),
    (2000, 2800),
    (2600, 10000),
];

async fn generate_bingo() -> Result<Vec<Problem>> {
    let mut rng = rand::thread_rng();

    // Fetch problems and sort by difficulties.
    let mut problems = fetch_problems().await?;
    problems.sort_by_key(|problem| problem.difficulty);

    let mut bingo_problems = Vec::new();
    for (lower_diff, upper_diff) in DIFF_DISTR {
        // The minimum index whose difficulty is no less than `lower_diff`.
        // Double the difficulties so that any problems doesn't match and we can detect the precise border.
        // Problems whose difficulty is in [lower_diff, upper_diff) is in [lower_index, upper_index).
        let lower_index = problems
            .binary_search_by_key(&(lower_diff * 2 + 1), |problem| problem.difficulty * 2)
            .unwrap_or_else(|i| i);
        let upper_index = problems
            .binary_search_by_key(&(upper_diff * 2 + 1), |problem| problem.difficulty * 2)
            .unwrap_or_else(|i| i);

        // Choose problems from [lower_index, upper_index) randomly.
        let mut indices: Vec<usize> = (lower_index..upper_index).collect();
        let (chosen_indices, _) = indices.partial_shuffle(&mut rng, BINGO_SIZE * BINGO_SIZE);
        let mut bingo = chosen_indices
            .iter_mut()
            .map(|index| problems[*index].clone())
            .collect();

        bingo_problems.append(&mut bingo);
    }

    Ok(bingo_problems)
}

async fn store_bingos(problems: &[Problem], client: &Client) -> Result<()> {
    let today = Local::today().naive_local();

    // Store to the database.
    for (position, problem) in problems.iter().enumerate() {
        client
            .execute(
                "INSERT INTO bingos \
                (created_date, position, problem_id, contest_id, title, difficulty) \
                VALUES ($1, $2, $3, $4, $5, $6)",
                &[
                    &today,
                    &(position as i32),
                    &problem.problem_id,
                    &problem.contest_id,
                    &problem.title,
                    &problem.difficulty,
                ],
            )
            .await?;
    }
    Ok(())
}

async fn generate_and_store_daily_bingo(client: &Client) -> Result<bool> {
    // Check if today's bingo already exists.
    let row = client
        .query_one("SELECT max(created_date) FROM bingos", &[])
        .await?;
    let newest_date_opt: Option<NaiveDate> = row.get(0);

    let bingo_exists = match newest_date_opt {
        Some(newest_date) => newest_date == Local::today().naive_local(),
        None => false,
    };

    if bingo_exists {
        return Ok(false);
    }

    // Generate and store bingo.
    let bingos = generate_bingo().await?;
    store_bingos(&bingos, client).await?;
    Ok(true)
}

#[tokio::main]
async fn main() {
    let client = get_postgres_client().await;

    loop {
        // Check if the daily bingo exists in every 5 mins
        match generate_and_store_daily_bingo(&client).await {
            Ok(true) => eprintln!("New bingo is generated."),
            Ok(false) => eprintln!("Today's bingo already exists."),
            Err(e) => eprintln!("Failed to generate bingo: {}", e),
        }
        sleep(Duration::from_secs(300)).await;
    }
}
