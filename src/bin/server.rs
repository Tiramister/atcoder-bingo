use std::fmt;

use actix_files as fs;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, ResponseError};
use askama::Template;
use atcoder_bingo_backend::database::get_postgres_client;
use chrono::Local;
use regex::Regex;
use serde::Deserialize;
use thiserror::Error;

const LEVEL_NAMES: [&str; 5] = ["NOVICE", "ADVANCED", "EXPERT", "MASTER", "ULTIMATE"];

#[derive(Clone)]
enum Status {
    NoStatus,
    Trying,
    Accepted,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Status::NoStatus => write!(f, "no-status"),
            Status::Trying => write!(f, "trying"),
            Status::Accepted => write!(f, "accepted"),
        }
    }
}

#[derive(Clone)]
struct Problem {
    row_id: i32,
    problem_id: String,
    contest_id: String,
    title: String,
    difficulty: i32,
    status: Status,
}

impl Problem {
    pub fn url(&self) -> String {
        format!(
            "https://atcoder.jp/contests/{}/tasks/{}",
            self.contest_id, self.problem_id,
        )
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    pub bingos: Vec<(Vec<Problem>, &'static str)>,
}

#[derive(Error, Debug)]
enum MyError {
    #[error(transparent)]
    Askama(#[from] askama::Error),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}
impl ResponseError for MyError {}

async fn get_todays_bingos(user_id_opt: &Option<String>) -> anyhow::Result<Vec<Vec<Problem>>> {
    let client = get_postgres_client().await;

    // Filter today's problems
    let today = Local::today().naive_local();
    let rows = client
        .query(
            "SELECT id, problem_id, contest_id, title, difficulty FROM bingos \
            WHERE created_date = $1 ORDER BY position asc",
            &[&today],
        )
        .await?;

    let mut problems: Vec<Problem> = rows
        .iter()
        .map(|row| Problem {
            row_id: row.get(0),
            problem_id: row.get(1),
            contest_id: row.get(2),
            title: row.get(3),
            difficulty: row.get(4),
            status: Status::NoStatus,
        })
        .collect();

    if let Some(user_id) = user_id_opt {
        // Validate input
        if Regex::new("^[a-zA-Z0-9_]{0,16}$").unwrap().is_match(user_id) {
            // Update status
            for problem in &mut problems {
                let rows = client
                    .query(
                        "SELECT accepted FROM user_status \
                    WHERE user_id = $1 AND problem_row_id = $2",
                        &[user_id, &problem.row_id],
                    )
                    .await?;

                if !rows.is_empty() {
                    let accepted: bool = rows[0].get(0);
                    problem.status = if accepted {
                        Status::Accepted
                    } else {
                        Status::Trying
                    };
                }
            }
        }
    }

    assert_eq!(problems.len(), 45);

    // Divide problems into 5 chunks with 9 problems
    let bingos = problems.chunks(9).map(|bingo| bingo.to_vec()).collect();
    Ok(bingos)
}

#[derive(Deserialize)]
struct IndexParameter {
    user_id: Option<String>,
}

#[get("/")]
async fn index(query: web::Query<IndexParameter>) -> actix_web::Result<impl Responder, MyError> {
    let bingos = get_todays_bingos(&query.user_id).await?;

    // Rendering
    let labeled_bingos = bingos.into_iter().zip(LEVEL_NAMES).collect();
    let html = IndexTemplate {
        bingos: labeled_bingos,
    };
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html.render()?))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/atcoder-bingo")
                .service(index)
                .service(fs::Files::new("/static", "./static").show_files_listing()),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;
    Ok(())
}
