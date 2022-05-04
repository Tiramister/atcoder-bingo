pub mod models;
mod problem;
mod user_status;

use anyhow::Result;
use chrono::NaiveDate;
use models::{Problem, UserStatus};
use std::{env, time::Duration};
use tokio::time::sleep;
use tokio_postgres::{Client, NoTls};

pub struct DatabaseClient {
    client: Client,
}

impl DatabaseClient {
    pub async fn new() -> Self {
        Self {
            client: get_connection().await,
        }
    }

    // Problems
    pub async fn insert_problem(&self, problem: &Problem) -> Result<()> {
        problem::insert(&self.client, problem).await
    }

    pub async fn select_problems_by_chosen_date(
        &self,
        chosen_date: &NaiveDate,
    ) -> Result<Vec<Problem>> {
        problem::select_by_chosen_date(&self.client, chosen_date).await
    }

    pub async fn select_problem_by_chosen_date_and_id(
        &self,
        chosen_date: &NaiveDate,
        problem_id: &str,
    ) -> Result<Option<Problem>> {
        problem::select_by_chosen_date_and_id(&self.client, chosen_date, problem_id).await
    }

    pub async fn select_newest_chosen_date_of_problems(&self) -> Result<Option<NaiveDate>> {
        problem::select_newest_chosen_date(&self.client).await
    }

    // User status
    pub async fn select_user_status(
        &self,
        user_id: &str,
        problem_row_id: i32,
    ) -> Result<Option<UserStatus>> {
        user_status::select(&self.client, user_id, problem_row_id).await
    }

    pub async fn insert_user_status(&self, user_status: &UserStatus) -> Result<()> {
        user_status::insert(&self.client, user_status).await
    }

    pub async fn update_user_status(&self, user_status: &UserStatus) -> Result<()> {
        user_status::update(&self.client, user_status).await
    }
}

/// Try to connect to the database until success.
/// Then return the client.
async fn get_connection() -> Client {
    let url = env::var("POSTGRES_URL").expect("POSTGRES_URL is not set.");

    let mut conn = tokio_postgres::connect(&url, NoTls).await;
    while let Err(e) = conn {
        log::error!("Failed to connect to the database: {e}");
        sleep(Duration::from_secs(5)).await;
        log::error!("Try to connect again...");
        conn = tokio_postgres::connect(&url, NoTls).await;
    }

    let (client, connection) = conn.unwrap();
    log::info!("Succeed to connect to the database.");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            log::error!("Connection error: {}", e);
        }
    });

    client
}
