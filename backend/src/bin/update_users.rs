use anyhow::Result;
use atcoder_bingo_backend::{
    crawler::submissions::{fetch_recent_submissions, Submission},
    database::get_postgres_client,
};
use std::time::Duration;
use tokio::time::sleep;
use tokio_postgres::Client;

async fn update_by_submission(client: &Client, submission: &Submission) -> Result<bool> {
    // Search the corresponding problem from the bingos.
    let submission_date = submission.submission_time.date();

    let rows = client
        .query(
            "SELECT id FROM bingos \
            WHERE created_date = $1 AND problem_id = $2",
            &[&submission_date, &submission.problem_id],
        )
        .await?;

    if rows.is_empty() {
        return Ok(false);
    }
    let row_id: i32 = rows[0].get(0);

    // Fetch user status
    let rows = client
        .query(
            "SELECT accepted FROM user_status \
            WHERE user_id = $1 AND problem_row_id = $2",
            &[&submission.user_id, &row_id],
        )
        .await?;

    let updated = if rows.is_empty() {
        client
            .execute(
                "INSERT INTO user_status (user_id, problem_row_id) VALUES ($1, $2)",
                &[&submission.user_id, &row_id],
            )
            .await?;
        true
    } else {
        let already_accepted: bool = rows[0].get(0);
        !already_accepted && submission.is_accepted
    };

    if updated {
        client
            .execute(
                "UPDATE user_status SET accepted = $1 \
                WHERE user_id = $2 AND problem_row_id = $3",
                &[&submission.is_accepted, &submission.user_id, &row_id],
            )
            .await?;
    }

    Ok(updated)
}

#[tokio::main]
async fn main() {
    // Try connecting to the database until success
    let client = get_postgres_client().await;

    loop {
        match fetch_recent_submissions(5).await {
            Ok(submissions) => {
                for submission in submissions {
                    if let Err(e) = update_by_submission(&client, &submission).await {
                        eprintln!("Failed to store a submission: {e}");
                    }
                }
                eprintln!("Finished to update.");
            }
            Err(e) => eprintln!("Failed to fetch recent submissions: {e}"),
        }
        sleep(Duration::from_secs(30)).await;
    }
}
