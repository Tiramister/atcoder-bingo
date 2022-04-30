use crate::crawler::api::get_request;
use anyhow::Result;
use chrono::{Duration, Local, NaiveDateTime};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RawSubmission {
    id: u32,
    epoch_second: i64,
    problem_id: String,
    user_id: String,
    result: String,
}

#[derive(Debug)]
pub struct Submission {
    pub id: u32,
    pub submission_time: NaiveDateTime,
    pub problem_id: String,
    pub user_id: String,
    pub is_accepted: bool,
}

/// Fetch at most 1000 submissions from `begin_time`.
async fn get_submissions_from(begin_time: &NaiveDateTime) -> Result<Vec<Submission>> {
    let begin_time_epoch = begin_time.timestamp();

    // Fetch raw submissions.
    let body = get_request(&format!(
        "https://kenkoooo.com/atcoder/atcoder-api/v3/from/{begin_time_epoch}"
    ))
    .await?;
    let raw_submissions: Vec<RawSubmission> = serde_json::from_str(&body)?;

    // Convert into `Submission`.
    let submissions = raw_submissions
        .iter()
        .map(|raw_submission| {
            // epoch to NaiveDateTime
            let submission_time = NaiveDateTime::from_timestamp(raw_submission.epoch_second, 0);
            Submission {
                id: raw_submission.id,
                submission_time,
                problem_id: raw_submission.problem_id.clone(),
                user_id: raw_submission.user_id.clone(),
                is_accepted: raw_submission.result == "AC",
            }
        })
        .collect();
    Ok(submissions)
}

/// Fetch all submissions in these `minutes` minutes.
pub async fn get_recent_submissions(minutes: i64) -> Result<Vec<Submission>> {
    let now = Local::now().naive_local();
    let mut begin_time = now.checked_sub_signed(Duration::minutes(minutes)).unwrap();

    let mut submissions = Vec::new();
    loop {
        eprintln!("Fetching submissions from {begin_time:?}...");

        let mut recent_submissions = get_submissions_from(&begin_time).await?;

        // Stash several information before `recent_submissions` is moved.
        let submission_num = recent_submissions.len();
        begin_time = recent_submissions
            .iter()
            .map(|submission| submission.submission_time)
            .max()
            .unwrap_or(now);

        submissions.append(&mut recent_submissions);

        eprintln!("{submission_num} submissions are obtained.");
        if submission_num < 1000 {
            break;
        }
    }

    Ok(submissions)
}
