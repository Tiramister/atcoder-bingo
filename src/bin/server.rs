use actix_files as fs;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, ResponseError};
use askama::Template;
use atcoder_bingo_backend::{crawler::problems::Problem, database::get_postgres_client};
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
    Askama(#[from] askama::Error),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}
impl ResponseError for MyError {}

async fn get_todays_bingos() -> anyhow::Result<Vec<Vec<Problem>>> {
    let client = get_postgres_client().await;

    // Filter today's problems
    let today = Local::today().naive_local();
    let rows = client
        .query(
            "SELECT problem_id, contest_id, title, difficulty FROM bingo \
            WHERE created_date == $1 ORDER BY position asc",
            &[&today],
        )
        .await?;

    let problems: Vec<Problem> = rows
        .iter()
        .map(|row| Problem {
            problem_id: row.get(0),
            contest_id: row.get(1),
            title: row.get(2),
            difficulty: row.get(3),
        })
        .collect();
    assert_eq!(problems.len(), 45);

    // Divide problems into 5 chunks with 9 problems
    let bingos = problems.chunks(9).map(|bingo| bingo.to_vec()).collect();
    Ok(bingos)
}

#[get("/")]
async fn index() -> actix_web::Result<impl Responder, MyError> {
    let bingos = get_todays_bingos().await?;

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
