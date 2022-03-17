use crate::bingo_generator::get_difficulties;
use log::error;

mod bingo_generator;

#[tokio::main]
async fn main() {
    env_logger::init();

    let difficulties = match get_difficulties().await {
        Ok(msg) => msg,
        Err(e) => return error!("Failed to fetch difficulties: {:?}", e),
    };

    println!("{}...", difficulties.chars().take(50).collect::<String>());
}
