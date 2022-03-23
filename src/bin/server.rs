use actix_files as fs;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, ResponseError, Result};
use askama::Template;
use atcoder_bingo_backend::{crawler::problems::Problem, database::get_client};
use chrono::Local;
use thiserror::Error;

const LEVEL_NAMES: [&str; 5] = ["NOVICE", "ADVANCED", "EXPERT", "MASTER", "ULTIMATE"];

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    pub bingos: Vec<(Vec<Problem>, &'static str)>,
}

#[derive(Error, Debug)]
enum MyError {
    #[error(transparent)]
    Database(#[from] tokio_postgres::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[error(transparent)]
    Askama(#[from] askama::Error),
}
impl ResponseError for MyError {}

#[get("/")]
async fn index() -> Result<impl Responder, MyError> {
    let client = get_client().await;

    let beginning_of_today = Local::today().and_hms(0, 0, 0);
    let rows = client.query(
        "SELECT problem_id, contest_id, title, difficulty FROM bingo WHERE created_date >= $1 ORDER BY level asc, position asc",
        &[&beginning_of_today],
    ).await?;

    let problems: Vec<Problem> = rows
        .iter()
        .map(|row| Problem {
            problem_id: row.get(0),
            contest_id: row.get(1),
            title: row.get(2),
            difficulty: row.get(3),
        })
        .collect();

    // Divide problems into 5 chunks with 9 problems
    let bingos = problems
        .chunks(9)
        .map(|bingo| bingo.to_vec())
        .zip(LEVEL_NAMES)
        .collect();

    // Rendering
    let html = IndexTemplate { bingos };
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html.render()?))
}

#[tokio::main]
async fn main() -> Result<()> {
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
