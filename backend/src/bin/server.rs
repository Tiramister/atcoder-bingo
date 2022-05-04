use actix_files as fs;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, ResponseError};
use askama::Template;
use atcoder_bingo_backend::database::{models, DatabaseClient};
use chrono::Local;
use regex::Regex;
use serde::Deserialize;
use std::fmt;
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

impl From<models::Problem> for Problem {
    fn from(problem: models::Problem) -> Self {
        Self {
            row_id: problem.id,
            problem_id: problem.problem_id.clone(),
            contest_id: problem.contest_id.clone(),
            title: problem.title.clone(),
            difficulty: problem.difficulty,
            status: Status::NoStatus,
        }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    pub problems: Vec<(Vec<Problem>, &'static str)>,
}

#[derive(Error, Debug)]
enum MyError {
    #[error(transparent)]
    Askama(#[from] askama::Error),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}
impl ResponseError for MyError {}

async fn get_todays_problems(user_id_opt: &Option<String>) -> anyhow::Result<Vec<Vec<Problem>>> {
    let client = DatabaseClient::new().await;

    // Filter today's problems
    let today = Local::today().naive_local();
    let mut problems = client.select_problems_by_chosen_date(&today).await?;

    // Sort by positions and convert
    problems.sort_by_key(|problem| problem.position);
    let mut problems: Vec<Problem> = problems.into_iter().map(Problem::from).collect();

    if let Some(user_id) = user_id_opt {
        // Validate input
        if Regex::new("^[a-zA-Z0-9_]{0,16}$")
            .unwrap()
            .is_match(user_id)
        {
            // Update status
            for problem in &mut problems {
                let user_status_opt = client.select_user_status(user_id, problem.row_id).await?;

                if let Some(user_status) = user_status_opt {
                    problem.status = if user_status.accepted {
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
    let problems = problems.chunks(9).map(|bingo| bingo.to_vec()).collect();
    Ok(problems)
}

#[derive(Deserialize)]
struct IndexParameter {
    user_id: Option<String>,
}

#[get("/")]
async fn index(query: web::Query<IndexParameter>) -> actix_web::Result<impl Responder, MyError> {
    let problems = get_todays_problems(&query.user_id).await?;

    // Rendering
    let labeled_problems = problems.into_iter().zip(LEVEL_NAMES).collect();
    let html = IndexTemplate {
        problems: labeled_problems,
    };
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html.render()?))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

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
