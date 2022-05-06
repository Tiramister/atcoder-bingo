use super::models::UserStatus;
use anyhow::Result;
use tokio_postgres::Client;

pub async fn select(
    client: &Client,
    user_id: &str,
    problem_row_id: i32,
) -> Result<Option<UserStatus>> {
    let row = client
        .query_opt(
            "SELECT * FROM user_status WHERE user_id = $1 AND problem_row_id = $2",
            &[&user_id, &problem_row_id],
        )
        .await?;

    let user_status = row.map(UserStatus::from);
    Ok(user_status)
}

/// All user status for problems with row ID (index in the database)
/// between `problem_row_id_from` and `problem_row_id_to` (inclusive).
pub async fn select_between_problem_row_id(
    client: &Client,
    problem_row_id_from: i32,
    problem_row_id_to: i32,
) -> Result<Vec<UserStatus>> {
    let row = client
        .query(
            "SELECT * FROM user_status WHERE problem_row_id >= $1 AND problem_row_id <= $2",
            &[&problem_row_id_from, &problem_row_id_to],
        )
        .await?;

    let user_status = row.into_iter().map(UserStatus::from).collect();
    Ok(user_status)
}

pub async fn insert(client: &Client, user_status: &UserStatus) -> Result<()> {
    client
        .execute(
            "INSERT INTO user_status (user_id, problem_row_id, accepted) VALUES ($1, $2, $3)",
            &[
                &user_status.user_id,
                &user_status.problem_row_id,
                &user_status.accepted,
            ],
        )
        .await?;
    Ok(())
}

pub async fn update(client: &Client, user_status: &UserStatus) -> Result<()> {
    client
        .execute(
            "UPDATE user_status SET accepted = $1 WHERE user_id = $2 AND problem_row_id = $3",
            &[
                &user_status.accepted,
                &user_status.user_id,
                &user_status.problem_row_id,
            ],
        )
        .await?;
    Ok(())
}
