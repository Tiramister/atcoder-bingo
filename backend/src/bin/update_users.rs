use anyhow::Result;
use atcoder_bingo_backend::{
    crawler::submissions::{get_recent_submissions, Submission},
    database::{models::UserStatus, DatabaseClient},
};
use std::time::Duration;
use tokio::time::sleep;

async fn update_user_status(client: &DatabaseClient, submission: &Submission) -> Result<bool> {
    // Search the corresponding problem.
    let submission_date = submission.submission_time.date();

    let problem_opt = client
        .select_problem_by_chosen_date_and_id(&submission_date, &submission.problem_id)
        .await?;

    let problem_row_id = match problem_opt {
        Some(problem) => problem.id,
        None => return Ok(false),
    };

    let new_user_status = UserStatus {
        user_id: submission.user_id.clone(),
        problem_row_id,
        accepted: submission.is_accepted,
    };

    // Search the current user status.
    let old_user_status_opt = client
        .select_user_status(&new_user_status.user_id, new_user_status.problem_row_id)
        .await?;

    // Insert or update the user status if necessary.
    match old_user_status_opt {
        Some(old_user_status) => {
            if !old_user_status.accepted && new_user_status.accepted {
                client.update_user_status(&new_user_status).await?;
                Ok(true)
            } else {
                Ok(false)
            }
        }
        None => {
            client.insert_user_status(&new_user_status).await?;
            Ok(true)
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let client = DatabaseClient::new().await;

    loop {
        match get_recent_submissions(60).await {
            Ok(submissions) => {
                for submission in submissions {
                    if let Err(e) = update_user_status(&client, &submission).await {
                        log::error!("Failed to update user status: {e}");
                    }
                }
                log::info!("Finished to update user status.");
            }
            Err(e) => log::error!("Failed to fetch recent submissions: {e}"),
        }
        sleep(Duration::from_secs(30)).await;
    }
}
