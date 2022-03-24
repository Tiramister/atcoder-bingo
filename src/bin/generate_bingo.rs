use anyhow::Result;
use atcoder_bingo_backend::{
    crawler::problems::{get_problems, Problem},
    database::get_client,
};
use chrono::Local;
use rand::prelude::SliceRandom;
use std::time::Duration;
use tokio_postgres::Client;

pub const BINGO_SIZE: usize = 3;
const DIFF_DISTR: [(i32, i32); 5] = [
    (-10000, 600),
    (400, 1400),
    (1200, 2200),
    (2000, 3000),
    (2800, 10000),
];

async fn generate_bingo() -> Result<Vec<Vec<Problem>>> {
    let mut rng = rand::thread_rng();

    // Fetch problems and sort by difficulties.
    let mut problems = get_problems().await?;
    problems.sort_by_key(|problem| problem.difficulty);

    let mut bingos = Vec::with_capacity(DIFF_DISTR.len());
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

        // choose problems from [lower_index, upper_index) randomly.
        let mut indices: Vec<usize> = (lower_index..upper_index).collect();
        let (chosen_indices, _) = indices.partial_shuffle(&mut rng, BINGO_SIZE * BINGO_SIZE);
        let bingo = chosen_indices
            .iter_mut()
            .map(|index| problems[*index].clone())
            .collect();
        bingos.push(bingo);
    }
    Ok(bingos)
}

async fn save_bingos(bingos: &[Vec<Problem>], client: &Client) -> Result<()> {
    // Write to database.
    for (level, bingo) in bingos.iter().enumerate() {
        for (position, problem) in bingo.iter().enumerate() {
            client.execute("INSERT INTO bingo (position, problem_id, contest_id, title, difficulty) VALUES ($1, $2, $3, $4, $5)", 
        &[&((level * 9 + position) as i32), &problem.problem_id, &problem.contest_id, &problem.title, &problem.difficulty]).await?;
        }
    }
    Ok(())
}

async fn generate_save_daily_bingo(client: &Client) -> Result<bool> {
    // Check if today's bingo is already exists.
    let row = client
        .query_one("SELECT max(created_time) FROM bingo", &[])
        .await?;
    let newest_timestamp: Option<chrono::DateTime<Local>> = row.get(0);

    let bingo_exists = match newest_timestamp {
        Some(timestamp) => timestamp.date() == Local::today(),
        None => false,
    };

    if bingo_exists {
        return Ok(false);
    }

    // Generate and store bingo.
    eprintln!("generating new bingo.");
    let bingos = generate_bingo().await?;
    save_bingos(&bingos, client).await?;
    Ok(true)
}

#[tokio::main]
async fn main() {
    // Connect to the database
    let mut client = get_client().await;
    while let Err(e) = client {
        eprintln!("{e}");
        tokio::time::sleep(5000);
        client = get_client().await;
    }
    let client = client.unwrap();

    loop {
        // Check if the daily bingo exists in every 5 mins
        if let Err(e) = generate_save_daily_bingo(&client).await {
            // Dump error message, but don't suspend.
            eprintln!("failed to generate bingo: {}", e);
        }
        tokio::time::sleep(Duration::from_secs(300)).await;
    }
}
