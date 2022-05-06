use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError};
use atcoder_bingo_backend::database::DatabaseClient;
use chrono::Local;
use std::sync::Mutex;
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}
impl ResponseError for MyError {}

#[get("/user-status/today")]
async fn user_status_today(req: HttpRequest) -> actix_web::Result<impl Responder, MyError> {
    log::info!("Request for today's user status");

    // Get database client from state
    let client = req.app_data::<web::Data<Mutex<DatabaseClient>>>().unwrap();
    let client = client.lock().unwrap();

    // Get the range of problem IDs.
    let today = Local::today().naive_local();
    let problems = client.select_problems_by_chosen_date(&today).await?;
    let min_id = problems.iter().map(|problem| problem.id).min().unwrap();
    let max_id = problems.iter().map(|problem| problem.id).max().unwrap();

    // Filter user submissions
    let user_status = client
        .select_user_status_between_problem_row_id(min_id, max_id)
        .await?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&user_status).unwrap()))
}

#[get("/problems/today")]
async fn problem_today(req: HttpRequest) -> actix_web::Result<impl Responder, MyError> {
    log::info!("Request for today's problems");

    // Get database client from state
    let client = req.app_data::<web::Data<Mutex<DatabaseClient>>>().unwrap();
    let client = client.lock().unwrap();

    // Filter today's problems
    let today = Local::today().naive_local();
    let problems = client.select_problems_by_chosen_date(&today).await?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&problems).unwrap()))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let client = DatabaseClient::new().await;

    // Wrap with web::Data and Mutex
    let client = web::Data::new(Mutex::new(client));

    HttpServer::new(move || {
        App::new().app_data(client.clone()).service(
            web::scope("/atcoder-bingo-api")
                .service(problem_today)
                .service(user_status_today),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;
    Ok(())
}
