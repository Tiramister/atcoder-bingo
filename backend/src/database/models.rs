use chrono::NaiveDate;

/// Problem information with its estimated difficulty.
#[derive(Clone, Debug)]
pub struct Problem {
    pub id: i32,
    pub chosen_date: NaiveDate,
    pub position: i32,
    pub problem_id: String,
    pub contest_id: String,
    pub title: String,
    pub difficulty: i32,
}

impl From<tokio_postgres::Row> for Problem {
    fn from(row: tokio_postgres::Row) -> Self {
        Problem {
            id: row.get("id"),
            chosen_date: row.get("chosen_date"),
            position: row.get("position"),
            problem_id: row.get("problem_id"),
            contest_id: row.get("contest_id"),
            title: row.get("title"),
            difficulty: row.get("difficulty"),
        }
    }
}

/// User status
#[derive(Clone, Debug)]
pub struct UserStatus {
    pub user_id: String,
    pub problem_row_id: i32,
    pub accepted: bool,
}

impl From<tokio_postgres::Row> for UserStatus {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            user_id: row.get("user_id"),
            problem_row_id: row.get("problem_row_id"),
            accepted: row.get("accepted"),
        }
    }
}
