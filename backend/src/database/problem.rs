use super::models::Problem;
use anyhow::Result;
use chrono::NaiveDate;
use tokio_postgres::Client;

pub async fn insert(client: &Client, problem: &Problem) -> Result<()> {
    client
        .execute(
            "INSERT INTO problems \
            (chosen_date, position, problem_id, contest_id, title, difficulty) \
            VALUES ($1, $2, $3, $4, $5, $6)",
            &[
                &problem.chosen_date,
                &problem.position,
                &problem.problem_id,
                &problem.contest_id,
                &problem.title,
                &problem.difficulty,
            ],
        )
        .await?;
    Ok(())
}

pub async fn select_by_chosen_date(
    client: &Client,
    chosen_date: &NaiveDate,
) -> Result<Vec<Problem>> {
    let rows = client
        .query(
            "SELECT * FROM problems WHERE chosen_date = $1 ORDER BY position asc",
            &[chosen_date],
        )
        .await?;

    let problems: Vec<Problem> = rows.into_iter().map(Problem::from).collect();

    Ok(problems)
}

pub async fn select_by_chosen_date_and_id(
    client: &Client,
    chosen_date: &NaiveDate,
    problem_id: &str,
) -> Result<Option<Problem>> {
    let row = client
        .query_opt(
            "SELECT * FROM problems \
            WHERE chosen_date = $1 AND problem_id = $2",
            &[chosen_date, &problem_id],
        )
        .await?;

    let problem = row.map(Problem::from);
    Ok(problem)
}

pub async fn select_newest_chosen_date(client: &Client) -> Result<Option<NaiveDate>> {
    let row = client
        .query_one("SELECT max(chosen_date) FROM problems", &[])
        .await?;
    let newest_chosen_date = row.try_get("max").ok();
    Ok(newest_chosen_date)
}
