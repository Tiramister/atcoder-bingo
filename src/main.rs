mod crawler;

use anyhow::Result;
use crawler::bingo_generator::generate_bingo;
use std::iter::zip;

const LEVELS: [&str; 5] = ["BEGINNER", "ADVANCED", "EXPERT", "MASTER", "ULTIMATE"];

#[tokio::main]
async fn main() -> Result<()> {
    let bingos = generate_bingo().await?;

    for (level, bingo) in zip(LEVELS, bingos) {
        println!("LEVEL {level}:");
        for problem in bingo {
            println!(
                "{} {} ({})",
                problem.contest_id, problem.title, problem.difficulty
            );
        }
        println!();
    }

    Ok(())
}
